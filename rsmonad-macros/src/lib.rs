//! Provides the `monad! {...}` macro, which parses (1) a data structure definition, (2) a function `bind`, and (3) a function `consume`, and implements the most idiomatic macro available as Rust continues to evolve.
//! # Use
//! ```rust
//! // use rsmonad::prelude::*; // <-- In your code, use this; here, though, we redefine a simpler `Maybe`, so we can't import everything
//! use rsmonad::prelude::{Monad, monad};
//!
//! monad! {
//!     /// Encodes the possibility of failure.
//!     enum Maybe<A> {
//!         Nothing,
//!         Just(A),
//!     }
//!
//!     fn bind(self, f) {
//!         match self {
//!             Nothing => Nothing,
//!             Just(b) => f(b),
//!         }
//!     }
//!
//!     fn consume(a) {
//!         Just(a)
//!     }
//! }
//!
//! fn could_overflow(x: u8) -> Maybe<u8> {
//!     x.checked_add(1).map_or(Nothing, Just)
//! }
//!
//! # fn main() {
//! assert_eq!(
//!     Nothing >> could_overflow,
//!     Nothing
//! );
//! assert_eq!(
//!     Just(1) >> could_overflow,
//!     Just(2)
//! );
//! assert_eq!(
//!     Just(255) >> could_overflow,
//!     Nothing
//! );
//! # }
//! ```

#![deny(warnings)]
#![warn(
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo,
    missing_docs,
    rustdoc::all
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::string_add,
    clippy::wildcard_enum_match_arm
)]

use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

/// Write the boilerplate for a monad given the minimal definition.
#[proc_macro]
pub fn monad(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match transmute(ts.into()) {
        Ok(out) => out,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Gets the next character and makes sure it exists.
macro_rules! next {
    ($tokens:ident, $span:expr, $msg:expr $(,)?) => {
        $tokens.next().ok_or_else(|| syn::Error::new($span, $msg))?
    };
}

/// Exits immediately with a custom compilation error.
macro_rules! bail {
    ($span:expr, $msg:expr $(,)?) => {
        return Err(syn::Error::new($span, $msg))
    };
}

/// Matches very safely against a token tree without making you repeat yourself.
macro_rules! match_tt {
    ($tokens:ident, $Type:ident, $msg:expr, $prev_span:expr $(,)?) => {
        match next!($tokens, $prev_span, concat!($msg, " after this")) {
            TokenTree::$Type(matched) => matched,
            other => bail!(other.span(), $msg),
        }
    };
}

/// Speed through attributes, pasting them verbatim
fn skip_attributes(
    out: &mut TokenStream,
    tokens: &mut proc_macro2::token_stream::IntoIter,
) -> syn::Result<TokenTree> {
    loop {
        let tt = next!(
            tokens,
            Span::call_site(),
            "Expected a data structure definition",
        );
        let TokenTree::Punct(pound) = tt else {
            return Ok(tt);
        };
        if pound.as_char() != '#' {
            bail!(pound.span(), "Expected a data structure definition; found a single character (that is not a '#' before an attribute)");
        }
        pound.to_tokens(out);

        let attr = match_tt!(tokens, Group, "Expected an attribute", pound.span());
        if attr.delimiter() != Delimiter::Bracket {
            bail!(attr.span(), "Expected an attribute in [...] brackets")
        }
        attr.to_tokens(out);
    }
}

/// Actually transform the AST, returning an error without boilerplate to be handled above.
#[allow(clippy::too_many_lines)]
fn transmute(raw_ts: TokenStream) -> syn::Result<TokenStream> {
    let mut out = TokenStream::new();
    let mut tokens = raw_ts.into_iter();

    // Parse the data-structure declaration
    let mut data_structure = TokenStream::new();
    let mut structure = match skip_attributes(&mut out, &mut tokens)? {
        TokenTree::Ident(i) => i,
        tt => bail!(tt.span(), "Expected a data structure definition",),
    };
    let mut publicity = None;
    structure.to_tokens(&mut data_structure);
    if structure == "pub" {
        publicity = Some(structure);
        structure = match_tt!(
            tokens,
            Ident,
            "Expected a data structure definition",
            publicity.span(),
        );
        structure.to_tokens(&mut data_structure);
    }
    let name = match_tt!(tokens, Ident, "Expected a name", Span::call_site());
    name.to_tokens(&mut data_structure);

    // Parse generics
    let generic_open = match_tt!(tokens, Punct, "Expected generics, e.g. `<A>`", name.span());
    if generic_open.as_char() != '<' {
        bail!(generic_open.span(), "Expected generics, e.g. `<A>`");
    }
    generic_open.to_tokens(&mut data_structure);
    let mut inception: u8 = 1;
    'generic_loop: loop {
        let generic = next!(tokens, generic_open.span(), "Unterminated generics");
        generic.to_tokens(&mut data_structure);
        if let TokenTree::Punct(ref maybe_close) = generic {
            match maybe_close.as_char() {
                '<' => {
                    inception = inception.checked_add(1).ok_or_else(|| {
                        syn::Error::new(
                            maybe_close.span(),
                            "Call Christopher Nolan: this inception is too deep",
                        )
                    })?;
                }
                '>' => {
                    inception = inception.wrapping_sub(1);
                    if inception == 0 {
                        break 'generic_loop;
                    }
                }
                _ => (),
            }
        }
    }

    // Parse the definition itself
    let def_block = match_tt!(
        tokens,
        Group,
        "Expected a definition block, e.g. `{...}`",
        data_structure.span()
    );
    def_block.to_tokens(&mut data_structure);
    if def_block.delimiter() == Delimiter::Parenthesis {
        let semicolon = match_tt!(
            tokens,
            Punct,
            "Expected a semicolon (after a tuple-struct)",
            def_block.span()
        );
        if semicolon.as_char() != ';' {
            bail!(
                semicolon.span(),
                "Expected a semicolon (after a tuple-struct)",
            );
        }
        semicolon.to_tokens(&mut data_structure);
    }

    // Parse as either a `struct` or `enum`
    let (ident, generics, fields) = if structure == "enum" {
        from_enum(
            &mut out,
            syn::parse2(data_structure).map_err(move |e| {
                syn::Error::new(
                    e.span(),
                    e.to_string() + " (`syn` error while parsing as an enum)",
                )
            })?,
            publicity,
        )?
    } else if structure == "struct" {
        from_struct(
            &mut out,
            syn::parse2(data_structure).map_err(move |e| {
                syn::Error::new(
                    e.span(),
                    e.to_string() + " (`syn` error while parsing as a struct)",
                )
            })?,
        )?
    } else {
        bail!(structure.span(), "Expected either `struct` or `enum`");
    };

    // Parse `bind`
    let mut bind = TokenStream::new();
    let t_fn = match_tt!(tokens, Ident, "Expected `fn`", def_block.span());
    if t_fn != "fn" {
        bail!(t_fn.span(), "Expected `fn`",);
    }
    t_fn.to_tokens(&mut bind);
    let t_bind = match_tt!(tokens, Ident, "Expected `bind`", t_fn.span());
    if t_bind != "bind" {
        bail!(t_bind.span(), "Expected `bind`")
    }
    t_bind.to_tokens(&mut bind);
    let args = match_tt!(
        tokens,
        Group,
        "Expected arguments immediately after `bind` (no need to repeat the <A: ...> bound)",
        t_bind.span(),
    );
    if args.delimiter() != Delimiter::Parenthesis {
        bail!(args.span(), "Expected arguments immediately after `bind`");
    }
    let args = proc_macro2::Group::new(Delimiter::Parenthesis, {
        let mut args_ts = TokenStream::new();
        let mut bare = args.stream().into_iter();
        let t_self = match skip_attributes(&mut args_ts, &mut bare)? {
            TokenTree::Ident(i) => i,
            tt => bail!(tt.span(), "Expected `self`"),
        };
        if t_self != "self" {
            bail!(t_self.span(), "Expected `self`");
        }
        t_self.to_tokens(&mut args_ts);
        let comma = match_tt!(bare, Punct, "Expected a comma", t_self.span());
        if comma.as_char() != ',' {
            bail!(comma.span(), "Expected a comma");
        }
        comma.to_tokens(&mut args_ts);
        let f = match skip_attributes(&mut args_ts, &mut bare)? {
            TokenTree::Ident(i) => i,
            tt => bail!(tt.span(), "Expected `f`"),
        };
        f.to_tokens(&mut args_ts);
        proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone).to_tokens(&mut args_ts);
        proc_macro2::Ident::new("F", Span::call_site()).to_tokens(&mut args_ts);
        args_ts
    });
    args.to_tokens(&mut bind);
    let def_block = match_tt!(
        tokens,
        Group,
        "Expected a function definition block (please don't try to specify return type; it's extremely long and will change as Rust evolves)",
        args.span(),
    );
    if def_block.delimiter() != Delimiter::Brace {
        bail!(def_block.span(), "Expected a function definition block");
    }
    def_block.to_tokens(&mut bind);
    let mut bind: syn::ImplItemFn = syn::parse2(bind)?;
    let inline_always: syn::MetaList = syn::parse2(quote! { inline(always) })?;
    bind.attrs.push(syn::Attribute {
        pound_token: syn::token::Pound {
            spans: [Span::call_site()],
        },
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket {
            span: *inline_always.delimiter.span(),
        },
        meta: syn::Meta::List(inline_always.clone()),
    });
    bind.sig.generics.lt_token = Some(syn::token::Lt {
        spans: [Span::call_site()],
    });
    bind.sig.generics.gt_token = Some(syn::token::Gt {
        spans: [Span::call_site()],
    });
    bind.sig.generics.params.push_value({
        let Some(syn::GenericParam::Type(gpt)) = generics.params.first() else {
            bail!(generics.span(), "Expected at least one generic argument");
        };
        let mut gpt = gpt.clone();
        gpt.ident = syn::Ident::new("B", Span::call_site());
        syn::GenericParam::Type(gpt)
    });

    bind.sig
        .generics
        .params
        .push(syn::GenericParam::Type(syn::parse2(
            quote! { F: Fn(A) -> Self::Hkt<B> },
        )?));
    bind.sig.output = syn::ReturnType::Type(
        syn::token::RArrow {
            spans: [Span::call_site(), Span::call_site()],
        },
        Box::new(syn::Type::Path(syn::parse2(quote! { Self::Hkt<B> })?)),
    );

    // Parse `consume`
    let mut consume = TokenStream::new();
    let t_fn = match_tt!(tokens, Ident, "Expected `fn`", def_block.span(),);
    if t_fn != "fn" {
        bail!(t_fn.span(), "Expected `fn`",);
    }
    t_fn.to_tokens(&mut consume);
    let t_consume = match_tt!(tokens, Ident, "Expected `consume`", t_fn.span());
    if t_consume != "consume" {
        bail!(t_bind.span(), "Expected `consume`")
    }
    t_consume.to_tokens(&mut consume);
    let args = match_tt!(
        tokens,
        Group,
        "Expected arguments immediately after `consume` (no need to repeat the <A: ...> bound)",
        t_bind.span(),
    );
    if args.delimiter() != Delimiter::Parenthesis {
        bail!(
            args.span(),
            "Expected arguments immediately after `consume`"
        );
    }
    let args = proc_macro2::Group::new(Delimiter::Parenthesis, {
        let mut args_ts = TokenStream::new();
        let mut bare = args.stream().into_iter();
        let a = match skip_attributes(&mut args_ts, &mut bare)? {
            TokenTree::Ident(i) => i,
            tt => bail!(tt.span(), "Expected `a`"),
        };
        a.to_tokens(&mut args_ts);
        proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone).to_tokens(&mut args_ts);
        proc_macro2::Ident::new("A", Span::call_site()).to_tokens(&mut args_ts);
        args_ts
    });
    args.to_tokens(&mut consume);
    let def_block = match_tt!(
        tokens,
        Group,
        "Expected a function definition block (please don't try to specify return type; it's extremely long and will change as Rust evolves)",
        args.span(),
    );
    if def_block.delimiter() != Delimiter::Brace {
        bail!(def_block.span(), "Expected a function definition block");
    }
    def_block.to_tokens(&mut consume);
    let mut consume: syn::ImplItemFn = syn::parse2(consume)?;
    consume.attrs.push(syn::Attribute {
        pound_token: syn::token::Pound {
            spans: [Span::call_site()],
        },
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket {
            span: *inline_always.delimiter.span(),
        },
        meta: syn::Meta::List(inline_always),
    });
    consume.sig.generics.lt_token = Some(syn::token::Lt {
        spans: [Span::call_site()],
    });
    consume.sig.generics.gt_token = Some(syn::token::Gt {
        spans: [Span::call_site()],
    });
    consume.sig.output = syn::ReturnType::Type(
        syn::token::RArrow {
            spans: [Span::call_site(), Span::call_site()],
        },
        Box::new(syn::Type::Path(syn::parse2(quote! { Self })?)),
    );

    // Make an `impl... Monad`
    syn::ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: syn::token::Impl {
            span: Span::call_site(),
        },
        generics: generics.clone(),
        trait_: Some((
            None,
            syn::Path {
                leading_colon: None,
                segments: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push_value(syn::PathSegment {
                        ident: syn::Ident::new("Monad", Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: syn::token::Lt {
                                    spans: [Span::call_site()],
                                },
                                args: generics
                                    .params
                                    .iter()
                                    .map(move |gp| {
                                        let syn::GenericParam::Type(gpt) = gp else {
                                            // SAFETY:
                                            // Checked earlier in the function. Not mutable.
                                            unsafe { core::hint::unreachable_unchecked() }
                                        };
                                        syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                            qself: None,
                                            path: syn::Path {
                                                leading_colon: None,
                                                segments: {
                                                    let mut punc =
                                                        syn::punctuated::Punctuated::new();
                                                    punc.push_value(syn::PathSegment {
                                                        ident: gpt.ident.clone(),
                                                        arguments: syn::PathArguments::None,
                                                    });
                                                    punc
                                                },
                                            },
                                        }))
                                    })
                                    .collect(),
                                gt_token: syn::token::Gt {
                                    spans: [Span::call_site()],
                                },
                            },
                        ),
                    });
                    p
                },
            },
            syn::token::For {
                span: Span::call_site(),
            },
        )),
        self_ty: Box::new(syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push_value(syn::PathSegment {
                        ident: ident.clone(),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: syn::token::Lt {
                                    spans: [Span::call_site()],
                                },
                                args: generics
                                    .params
                                    .iter()
                                    .map(move |gp| {
                                        let syn::GenericParam::Type(gpt) = gp else {
                                            // SAFETY:
                                            // Checked earlier in the function. Not mutable.
                                            unsafe { core::hint::unreachable_unchecked() }
                                        };
                                        syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                            qself: None,
                                            path: syn::Path {
                                                leading_colon: None,
                                                segments: {
                                                    let mut punc =
                                                        syn::punctuated::Punctuated::new();
                                                    punc.push_value(syn::PathSegment {
                                                        ident: gpt.ident.clone(),
                                                        arguments: syn::PathArguments::None,
                                                    });
                                                    punc
                                                },
                                            },
                                        }))
                                    })
                                    .collect(),
                                gt_token: syn::token::Gt {
                                    spans: [Span::call_site()],
                                },
                            },
                        ),
                    });
                    p
                },
            },
        })),
        brace_token: syn::token::Brace {
            span: proc_macro2::Group::new(Delimiter::Brace, TokenStream::new()).delim_span(),
        },
        items: vec![
            syn::ImplItem::Type(syn::ImplItemType {
                attrs: vec![],
                vis: syn::Visibility::Inherited,
                defaultness: None,
                type_token: syn::token::Type {
                    span: Span::call_site(),
                },
                ident: syn::Ident::new("Hkt", Span::call_site()),
                generics: {
                    let mut g = generics.clone();
                    let Some(syn::GenericParam::Type(gpt)) = g.params.first_mut() else {
                        bail!(g.span(), "Expected at least one generic argument");
                    };
                    gpt.ident = syn::Ident::new("B", Span::call_site());
                    g
                },
                /*
                syn::Generics {
                    lt_token: Some(syn::token::Lt {
                        spans: [Span::call_site()],
                    }),
                    params: {
                        let mut p = syn::punctuated::Punctuated::new();
                        p.push_value(syn::GenericParam::Type(syn::TypeParam {
                            attrs: vec![],
                            ident: syn::Ident::new("B", Span::call_site()),
                            colon_token: None,
                            bounds: syn::punctuated::Punctuated::new(),
                            eq_token: None,
                            default: None,
                        }));
                        p
                    },
                    gt_token: Some(syn::token::Gt {
                        spans: [Span::call_site()],
                    }),
                    where_clause: None,
                },
                */
                eq_token: syn::token::Eq {
                    spans: [Span::call_site()],
                },
                ty: syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: {
                            let mut p = syn::punctuated::Punctuated::new();
                            p.push_value(syn::PathSegment {
                                ident: ident.clone(),
                                arguments: syn::PathArguments::AngleBracketed(
                                    syn::AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: syn::token::Lt {
                                            spans: [Span::call_site()],
                                        },
                                        args: {
                                            let mut p = syn::punctuated::Punctuated::new();
                                            p.push_value(syn::GenericArgument::Type(
                                                syn::Type::Path(syn::TypePath {
                                                    qself: None,
                                                    path: syn::Path {
                                                        leading_colon: None,
                                                        segments: {
                                                            let mut b =
                                                                syn::punctuated::Punctuated::new();
                                                            b.push_value(syn::PathSegment {
                                                                ident: syn::Ident::new(
                                                                    "B",
                                                                    Span::call_site(),
                                                                ),
                                                                arguments: syn::PathArguments::None,
                                                            });
                                                            b
                                                        },
                                                    },
                                                }),
                                            ));
                                            p
                                        },
                                        gt_token: syn::token::Gt {
                                            spans: [Span::call_site()],
                                        },
                                    },
                                ),
                            });
                            p
                        },
                    },
                }),
                semi_token: syn::token::Semi {
                    spans: [Span::call_site()],
                },
            }),
            syn::ImplItem::Fn(bind),
            syn::ImplItem::Fn(consume),
        ],
    }
    .to_tokens(&mut out);

    syn::ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: syn::token::Impl {
            span: Span::call_site(),
        },
        generics: {
            let mut g: syn::Generics =
                syn::parse2(quote! { <> })?;
            g.params.push_value({
                let Some(syn::GenericParam::Type(gpt)) = generics.params.first() else {
                    bail!(generics.span(), "Expected at least one generic argument");
                };
                syn::GenericParam::Type(gpt.clone())
            });
            g.params.push({
                let Some(syn::GenericParam::Type(gpt)) = generics.params.first() else {
                    bail!(generics.span(), "Expected at least one generic argument");
                };
                let mut gpt = gpt.clone();
                gpt.ident = syn::Ident::new("B", Span::call_site());
                syn::GenericParam::Type(gpt)
            });

            let mut fn_trait: syn::TypeParam = syn::parse2(quote! { F: Fn(A) -> FixItInPost })?;
            match fn_trait.bounds.last_mut().ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    "rsmonad-internal error: expected generic arguments",
                )
            })? {
                syn::TypeParamBound::Trait(t) => {
                    match &mut t
                        .path
                        .segments
                        .last_mut()
                        .ok_or_else(|| {
                            syn::Error::new(
                            Span::call_site(),
                            "rsmonad-internal error: zero-length path in final type parameter",
                        )
                        })?
                        .arguments
                    {
                        syn::PathArguments::Parenthesized(p) => p.output = syn::ReturnType::Type(syn::parse2(quote! { -> })?, Box::new(syn::Type::Path(syn::TypePath { qself: None, path: syn::Path { leading_colon: None, segments: {
                            let mut rtype = syn::punctuated::Punctuated::new();
                            rtype.push_value(syn::PathSegment { ident: ident.clone(), arguments: syn::PathArguments::AngleBracketed(syn::parse2(quote! { <B> })?) });
                            rtype
                        } } }))),
                        _ => bail!(Span::call_site(),"rsmonad-internal error: Expected parenthesized arguments in a `Fn` trait"),
                    }
                }
                tpb => bail!(tpb.span(), "Expected a trait"),
            }
            g.params.push(syn::GenericParam::Type(fn_trait));
            g
        },
        trait_: Some((
            None,
            syn::parse2(quote! { core::ops::Shr<F> })?,
            syn::parse2(quote! { for })?,
        )),
        self_ty: Box::new(syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push_value(syn::PathSegment {
                        ident: ident.clone(),
                        arguments: syn::PathArguments::AngleBracketed(syn::parse2(quote! { <A> })?),
                    });
                    p
                },
            },
        })),
        brace_token: syn::token::Brace {
            span: proc_macro2::Group::new(Delimiter::Brace, TokenStream::new()).delim_span(),
        },
        items: vec![
            syn::ImplItem::Type(syn::parse2(quote! { type Output = <Self as Monad<A>>::Hkt<B>; })?),
            syn::ImplItem::Fn(syn::parse2(quote! { #[inline(always)] fn shr(self, f: F) -> Self::Output { self.bind(f) } })?),
        ],
    }
    .to_tokens(&mut out);

    syn::ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: syn::token::Impl {
            span: Span::call_site(),
        },
        generics: syn::parse2(quote! { <A: quickcheck::Arbitrary> })?,
        trait_: Some((
            None,
            syn::parse2(quote! { quickcheck::Arbitrary })?,
            syn::token::For {
                span: Span::call_site(),
            },
        )),
        self_ty: Box::new(syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push_value(syn::PathSegment {
                        ident: ident.clone(),
                        arguments: syn::PathArguments::AngleBracketed(syn::parse2(quote! { <A> })?),
                    });
                    p
                },
            },
        })),
        brace_token: syn::token::Brace {
            span: proc_macro2::Group::new(Delimiter::Brace, TokenStream::new()).delim_span(),
        },
        items: vec![syn::ImplItem::Fn({
            let mut def: syn::ImplItemFn =
                syn::parse2(quote! { fn arbitrary(g: &mut quickcheck::Gen) -> FixItInPost {} })?;
            def.sig.output = syn::ReturnType::Type(
                syn::parse2(quote! { -> })?,
                Box::new(syn::parse2(quote! { Self })?),
            );
            def.block.stmts.push(syn::Stmt::Expr(
                match fields {
                    Fields::EnumVariants(variants) => {
                        let mut elems = syn::punctuated::Punctuated::new();
                        for variant in variants {
                            let body = if matches!(variant.fields, syn::Fields::Unit) {
                                Box::new(syn::Expr::Path(syn::ExprPath {
                                    attrs: vec![],
                                    qself: None,
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut p = syn::punctuated::Punctuated::new();
                                            p.push_value(syn::PathSegment {
                                                ident: variant.ident,
                                                arguments: syn::PathArguments::None,
                                            });
                                            p
                                        },
                                    },
                                }))
                            } else {
                                Box::new(syn::Expr::Call(syn::ExprCall {
                                    attrs: vec![],
                                    func: Box::new(syn::Expr::Path(syn::ExprPath {
                                        attrs: vec![],
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut p = syn::punctuated::Punctuated::new();
                                                p.push_value(syn::PathSegment {
                                                    ident: variant.ident,
                                                    arguments: syn::PathArguments::None,
                                                });
                                                p
                                            },
                                        },
                                    })),
                                    paren_token: syn::token::Paren {
                                        span: proc_macro2::Group::new(
                                            Delimiter::Parenthesis,
                                            TokenStream::new(),
                                        )
                                        .delim_span(),
                                    },
                                    args: {
                                        let mut p = syn::punctuated::Punctuated::new();
                                        match variant.fields {
                                            // SAFETY:
                                            // Logically impossible. See `if` statement at definition of `body`.
                                            syn::Fields::Unit => unsafe { core::hint::unreachable_unchecked() },
                                            syn::Fields::Unnamed(members) => {
                                                for member in members.unnamed {
                                                    p.push(syn::Expr::Call({
                                                        let mut init: syn::ExprCall = syn::parse2(quote! { <FixItInPost as quickcheck::Arbitrary>::arbitrary(gen) })?;
                                                        match init.func.as_mut() {
                                                            syn::Expr::Path(path) => {
                                                                let Some(qself) = &mut path.qself else {
                                                                    bail!(init.span(), "rsmonad-internal error: couldn't parse qself in `<T as quickcheck::Arbitrary>::arbitrary(gen)`");
                                                                };
                                                                *qself.ty.as_mut() = member.ty;
                                                            }
                                                            _ => bail!(init.span(), "rsmonad-internal error: couldn't parse `<T as quickcheck::Arbitrary>::arbitrary(gen)` as a path"),
                                                        }
                                                        init
                                                    }));
                                                }
                                            }
                                            syn::Fields::Named(members) => {
                                                for member in members.named {
                                                    p.push(syn::Expr::Call({
                                                        let mut init: syn::ExprCall = syn::parse2(quote! { <FixItInPost as quickcheck::Arbitrary>::arbitrary(gen) })?;
                                                        match init.func.as_mut() {
                                                            syn::Expr::Path(path) => {
                                                                let Some(qself) = &mut path.qself else {
                                                                    bail!(init.span(), "rsmonad-internal error: couldn't parse qself in `<T as quickcheck::Arbitrary>::arbitrary(gen)`");
                                                                };
                                                                *qself.ty.as_mut() = member.ty;
                                                            }
                                                            _ => bail!(init.span(), "rsmonad-internal error: couldn't parse `<T as quickcheck::Arbitrary>::arbitrary(gen)` as a path"),
                                                        }
                                                        init
                                                    }));
                                                }
                                            }
                                        }
                                        p
                                    },
                                }))
                            };
                            let closure = syn::Expr::Closure(syn::ExprClosure {
                                attrs: vec![],
                                lifetimes: None,
                                constness: None,
                                movability: None,
                                asyncness: None,
                                capture: Some(syn::token::Move { span: Span::call_site() }),
                                or1_token: syn::token::Or {
                                    spans: [Span::call_site()],
                                },
                                inputs: {
                                    let mut inputs = syn::punctuated::Punctuated::new();
                                    inputs.push_value(syn::Pat::Ident(syn::PatIdent {
                                        attrs: vec![],
                                        by_ref: None,
                                        mutability: None,
                                        ident: syn::Ident::new("gen", Span::call_site()),
                                        subpat: None,
                                    }));
                                    inputs
                                },
                                or2_token: syn::token::Or {
                                    spans: [Span::call_site()],
                                },
                                output: syn::ReturnType::Default,
                                body,
                            });
                            let paren = syn::Expr::Paren(syn::ExprParen {
                                attrs: vec![],
                                paren_token: syn::token::Paren {
                                    span: proc_macro2::Group::new(
                                        Delimiter::Parenthesis,
                                        TokenStream::new(),
                                    )
                                    .delim_span(),
                                },
                                expr: Box::new(closure),
                            });
                            elems.push(syn::Expr::Cast(syn::ExprCast {
                                attrs: vec![],
                                expr: Box::new(paren),
                                as_token: syn::token::As { span: Span::call_site() },
                                ty: Box::new(syn::parse2(quote! { fn(&mut quickcheck::Gen) -> Self })?),
                            }));
                        }
                        let mut choose: syn::ExprCall = syn::parse2(quote! { g.choose::<fn(&mut quickcheck::Gen) -> Self>(&[]).unwrap()(g) })?;
                        let syn::Expr::MethodCall(pre_call) = choose.func.as_mut() else {
                            bail!(Span::call_site(), "rsmonad-internal error: expected a method call")
                        };
                        let syn::Expr::MethodCall(pre_pre_call) = pre_call.receiver.as_mut() else {
                            bail!(Span::call_site(), "rsmonad-internal error: expected a method call")
                        };
                        let Some(syn::Expr::Reference(array_ref)) = pre_pre_call.args.first_mut() else {
                            bail!(Span::call_site(), "rsmonad-internal error: expected a single reference argument")
                        };
                        let syn::Expr::Array(closures) = array_ref.expr.as_mut() else {
                            bail!(choose.args.span(), "rsmonad-internal error: expected an array")
                        };
                        closures.elems = elems;
                        syn::Expr::Call(choose)
                    }
                    Fields::StructMembers(members) => match members {
                        syn::Fields::Unit => syn::Expr::Path(syn::ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments: {
                                    let mut p = syn::punctuated::Punctuated::new();
                                    p.push_value(syn::PathSegment {
                                        ident: syn::Ident::new("Self", Span::call_site()),
                                        arguments: syn::PathArguments::None,
                                    });
                                    p
                                }
                            }
                        }),
                        syn::Fields::Named(named) => {
                            syn::Expr::Struct(syn::ExprStruct {
                                attrs: vec![],
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut p = syn::punctuated::Punctuated::new();
                                        p.push_value(syn::PathSegment {
                                            ident: syn::Ident::new("Self", Span::call_site()),
                                            arguments: syn::PathArguments::None,
                                        });
                                        p
                                    }
                                },
                                brace_token: syn::token::Brace {
                                    span: proc_macro2::Group::new(
                                        Delimiter::Brace,
                                        TokenStream::new(),
                                    )
                                    .delim_span(),
                                },
                                fields: {
                                    let mut p = syn::punctuated::Punctuated::new();
                                    for member in named.named {
                                        p.push(syn::FieldValue {
                                            attrs: vec![],
                                            member: syn::Member::Named(member.ident.clone().ok_or_else(|| syn::Error::new(member.span(), "Expected a named field"))?),
                                            colon_token: Some(syn::token::Colon { spans: [Span::call_site()] }),
                                            expr: syn::Expr::Call({
                                                let mut init: syn::ExprCall = syn::parse2(quote! { <FixItInPost as quickcheck::Arbitrary>::arbitrary(g) })?;
                                                match init.func.as_mut() {
                                                    syn::Expr::Path(path) => {
                                                        let Some(qself) = &mut path.qself else {
                                                            bail!(init.span(), "rsmonad-internal error: couldn't parse qself in `<T as quickcheck::Arbitrary>::arbitrary(g)`");
                                                        };
                                                        *qself.ty.as_mut() = member.ty;
                                                    }
                                                    _ => bail!(init.span(), "rsmonad-internal error: couldn't parse `<T as quickcheck::Arbitrary>::arbitrary(g)` as a path"),
                                                }
                                                init
                                            }),
                                        });
                                    }
                                    p
                                },
                                dot2_token: None,
                                rest: None,
                            })
                        },
                        syn::Fields::Unnamed(unnamed) => {
                            syn::Expr::Call(syn::ExprCall {
                                attrs: vec![],
                                func: Box::new(syn::Expr::Path(syn::ExprPath {
                                    attrs: vec![],
                                    qself: None,
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut p = syn::punctuated::Punctuated::new();
                                            p.push_value(syn::PathSegment {
                                                ident: syn::Ident::new("Self", Span::call_site()),
                                                arguments: syn::PathArguments::None,
                                            });
                                            p
                                        },
                                    },
                                })),
                                paren_token: syn::token::Paren {
                                    span: proc_macro2::Group::new(
                                        Delimiter::Parenthesis,
                                        TokenStream::new(),
                                    )
                                    .delim_span(),
                                },
                                args: {
                                    let mut args = syn::punctuated::Punctuated::new();
                                    for member in unnamed.unnamed {
                                        args.push(
                                            syn::Expr::Call({
                                                let mut init: syn::ExprCall = syn::parse2(quote! { <FixItInPost as quickcheck::Arbitrary>::arbitrary(g) })?;
                                                match init.func.as_mut() {
                                                    syn::Expr::Path(path) => {
                                                        let Some(qself) = &mut path.qself else {
                                                            bail!(init.span(), "rsmonad-internal error: couldn't parse qself in `<T as quickcheck::Arbitrary>::arbitrary(g)`");
                                                        };
                                                        *qself.ty.as_mut() = member.ty;
                                                    }
                                                    _ => bail!(init.span(), "rsmonad-internal error: couldn't parse `<T as quickcheck::Arbitrary>::arbitrary(g)` as a path"),
                                                }
                                                init
                                            }),
                                        );
                                    }
                                    args
                                },
                            })
                        },
                    },
                },
                None,
            ));
            def
        })],
    }
    .to_tokens(&mut out);

    #[cfg(feature = "quickcheck")]
    syn::ItemMod {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        unsafety: None,
        mod_token: syn::token::Mod {
            span: Span::call_site(),
        },
        ident: syn::Ident::new(
            &(heck::ToSnakeCase::to_snake_case(ident.to_string().as_str()) + "_monad_laws"),
            Span::call_site(),
        ),
        content: Some((
            (syn::token::Brace {
                span: proc_macro2::Group::new(Delimiter::Brace, TokenStream::new()).delim_span(),
            }),
            vec![
                syn::Item::Use(syn::parse2(quote! { use super::*; })?),
                {
                    let mut u: syn::ItemUse =
                        syn::parse2(quote! { use super::FixItInPost as UnderInvestigation; })?;
                    let syn::UseTree::Path(upath) = &mut u.tree else {
                        bail!(u.span(), "Expected a two-term path (i.e. UseTree) like `super::Maybe` but didn't find a `UseTree::Path`");
                    };
                    let syn::UseTree::Rename(name) = upath.tree.as_mut() else {
                        bail!(u.span(), "Expected a two-term path (i.e. UseTree) like `super::Maybe` but didn't find a `UseTree::Rename`");
                    };
                    name.ident = ident;
                    syn::Item::Use(u)
                },
                syn::Item::Use(syn::parse2(quote! { use u64 as A; })?),
                syn::Item::Macro(syn::parse2(quote! {
                    quickcheck::quickcheck! {
                        fn prop_left_identity(a: A) -> bool {
                            rsmonad::monad_laws::left_identity::<_, _, UnderInvestigation<A>, _>(a, &rsmonad::monad_laws::hash_consume::<UnderInvestigation<A>, A>)
                        }
                        fn prop_right_identity(m: UnderInvestigation<A>) -> bool {
                            rsmonad::monad_laws::right_identity(m)
                        }
                        fn prop_associativity(m: UnderInvestigation<A>) -> bool {
                            rsmonad::monad_laws::associativity(m, &rsmonad::monad_laws::hash_consume::<UnderInvestigation<A>, A>, &rsmonad::monad_laws::hash_consume::<UnderInvestigation<A>, A>)
                        }
                    }
                })?),
            ],
        )),
        semi: None,
    }
    .to_tokens(&mut out);

    Ok(out)
}

/// Attribute deriving common traits.
fn derives() -> syn::Result<syn::Attribute> {
    let ml: syn::MetaList = syn::parse2(
        quote! {derive(Clone, Debug, /* Default, */ Eq, Hash, Ord, PartialEq, PartialOrd)},
    )
    .map_err(move |e| {
        syn::Error::new(
            e.span(),
            "rsmonad-internal error: couldn't parse #[derive(...)]. Please file an error--we want to fix what went wrong!",
        )
    })?;
    Ok(syn::Attribute {
        pound_token: syn::token::Pound {
            spans: [Span::call_site()],
        },
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket {
            span: *ml.delimiter.span(),
        },
        meta: syn::Meta::List(ml),
    })
}

/// Attribute allowing exhaustive enums and structs.
fn exhaustion() -> syn::Result<syn::Attribute> {
    let ml: syn::MetaList = syn::parse2(
        quote! { allow(clippy::non_exhaustive_enums, clippy::non_exhaustive_structs) },
    )
    .map_err(move |e| {
        syn::Error::new(
            e.span(),
            "rsmonad-internal error: couldn't parse #[allow(...)]. Please file an error--we want to fix what went wrong!",
        )
    })?;
    Ok(syn::Attribute {
        pound_token: syn::token::Pound {
            spans: [Span::call_site()],
        },
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket {
            span: *ml.delimiter.span(),
        },
        meta: syn::Meta::List(ml),
    })
}

/// Either `struct` or `enum` fields.
enum Fields {
    /// Enum variants.
    EnumVariants(syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>),
    /// Struct members.
    StructMembers(syn::Fields),
}

/// Parse an enum.
fn from_enum(
    out: &mut TokenStream,
    mut item: syn::ItemEnum,
    publicity: Option<proc_macro2::Ident>,
) -> syn::Result<(syn::Ident, syn::Generics, Fields)> {
    item.attrs.push(exhaustion()?);
    item.attrs.push(derives()?);
    item.to_tokens(out);
    if let Some(p) = publicity {
        p.to_tokens(out);
    }
    syn::Ident::new("use", Span::call_site()).to_tokens(out);
    item.ident.to_tokens(out);
    proc_macro2::Punct::new(':', proc_macro2::Spacing::Joint).to_tokens(out);
    proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone).to_tokens(out);
    proc_macro2::Group::new(Delimiter::Brace, {
        let mut ctors = TokenStream::new();
        for ctor in &item.variants {
            ctor.ident.to_tokens(&mut ctors);
            proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone).to_tokens(&mut ctors);
        }
        ctors
    })
    .to_tokens(out);
    proc_macro2::Punct::new(';', proc_macro2::Spacing::Alone).to_tokens(out);
    Ok((
        item.ident,
        item.generics,
        Fields::EnumVariants(item.variants),
    ))
}

/// Parse a struct.
fn from_struct(
    out: &mut TokenStream,
    mut item: syn::ItemStruct,
) -> syn::Result<(syn::Ident, syn::Generics, Fields)> {
    item.attrs.push(exhaustion()?);
    item.attrs.push(derives()?);
    item.to_tokens(out);
    Ok((
        item.ident,
        item.generics,
        Fields::StructMembers(item.fields),
    ))
}
