//! Macro for easy autocompletion of monad boilerplate until Rust's type system strengthens and we can derive it all.

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
    clippy::question_mark_used
)]

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;

/// Fill in the boilerplate for a monad definition.
#[proc_macro]
pub fn monad(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match transform(ts.into()) {
        Ok(done) => done,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

/// Immediately returns an error with a span (first arg) and message (second).
macro_rules! bail {
    ($span:expr, $msg:expr $(,)?) => {
        return ::core::result::Result::Err(::syn::Error::new($span, $msg))
    };
}

/// Immediately returns an error when a span is unknown.
macro_rules! bail_na {
    ($msg:expr $(,)?) => {
        bail!(Span::call_site(), $msg)
    };
}

/// Construct an identifier.
macro_rules! ident {
    ($s:tt $(,)?) => {
        Ident::new(stringify!($s), ::proc_macro2::Span::call_site())
    };
}

/// Construct punctuation either on its own or (equivalently) last in a sequence like `::`.
macro_rules! punct {
    ($c:expr $(,)?) => {
        ::proc_macro2::Punct::new($c, ::proc_macro2::Spacing::Alone)
    };
}

/// Writes a path separator (::) to the given token stream.
macro_rules! pathsep {
    ($ts:expr) => {
        Punct::new(':', Spacing::Joint).to_tokens($ts);
        punct!(':').to_tokens($ts);
    };
}

/// Writes #[inline(always)] to the given token stream.
macro_rules! inline_always {
    ($ts:expr) => {
        punct!('#').to_tokens($ts);
        Group::new(Delimiter::Bracket, {
            let mut inline = TokenStream::new();
            ident!(inline).to_tokens(&mut inline);
            Group::new(Delimiter::Parenthesis, ident!(always).into_token_stream())
                .to_tokens(&mut inline);
            inline
        })
        .to_tokens($ts);
    };
}

/// Transforms the original Rust-like AST into proper Rust with a monad implemented.
#[allow(clippy::too_many_lines)] // TODO: refactor & remove
fn transform(ts: TokenStream) -> Result<TokenStream, syn::Error> {
    let mut tokens = ts.into_iter();

    // Initialize an empty TokenStream
    let mut out = TokenStream::new();

    // Make sure there's at least some token in the invocation
    let Some(tt) = tokens.next() else {
        bail_na!("Empty invocation; add a monad definition");
    };
    if let TokenTree::Punct(ref p) = tt {
        // Translates three-comma docs into #[doc...
        if p.as_char() != '#' {
            bail!(
                p.span(),
                format_args!(
                    "Expected documentation, which Rust would automatically translate to `#[doc...`, but the first character was {:#?}",
                    p.as_char()
                )
            )
        }
        p.to_tokens(&mut out);
    }

    // Parse docs
    match tokens.next() {
        Some(TokenTree::Group(g)) => {
            if g.delimiter() != Delimiter::Bracket {
                bail!(
                    g.span(),
                    format_args!("Expected documentation, which Rust would automatically translate to `#[doc...`, but the delimiter after `#` was {:#?}", g.delimiter())
                )
            }
            let mut docs = g.stream().into_iter();
            if let Some(tt) = docs.next() {
                if let TokenTree::Ident(i) = tt {
                    if i != "doc" {
                        bail!(
                            i.span(),
                            format_args!("Expected documentation, which Rust would automatically translate to `#[doc...`, but found `#[{i:#?}...`"),
                        )
                    }
                } else {
                    bail!(tt.span(), format_args!("Expected documentation but found a different attribute: {tt:#?}"))
                }
            } else {
                bail!(g.span(), "Expected documentation but found nothing")
            }
            if let Some(TokenTree::Punct(p)) = docs.next() {
                if p.as_char() != '=' {
                    bail!(p.span(), "Docs don't seem to have any content")
                }
            } else {
                bail!(g.span(), "Docs don't seem to have any content")
            }
            if let Some(TokenTree::Literal(lit)) = docs.next() {
                let docstring = lit.to_string();
                if !docstring.starts_with("\" ") {
                    bail!(lit.span(), "Add a space after `///`")
                }
                if !docstring.starts_with("\" Encodes ") {
                    bail!(lit.span(), "Please lead with what the macro encodes. For example, it's commont to say that `Maybe` encodes the possibility of failure, lists encode nondeterminism, etc. Start with \"Encodes ...\", then, of course, the rest of however many lines can be whatever you want.")
                }
            } else {
                bail!(g.span(), "Docs don't seem to have any content")
            }
            g.to_tokens(&mut out);
        },
        Some(x) => bail!(
            x.span(),
            format_args!("Expected documentation, which Rust would automatically translate to `#[doc...`, but the token after `#` was {x:#?}")
        ),
        None => bail!(tt.span(), "Expected documentation, which Rust would automatically translate to `#[doc...`, but found nothing")
    }
    // Continue parsing docs
    let mut doc = TokenTree::Ident(ident!(parse_error));
    'docs: loop {
        // Read the `#` in front of an attribute; safely break otherwise
        doc = if let Some(tt) = tokens.next() {
            tt
        } else {
            bail!(
                doc.span(),
                "Add a data structure definition after your docs"
            )
        };
        if let TokenTree::Punct(p) = &doc {
            if p.as_char() != '#' {
                break 'docs;
            }
            p.to_tokens(&mut out);
        } else {
            break 'docs;
        }

        // Now we *must* have a doc: otherwise, raise a compilation error.
        match tokens.next(){
            Some(TokenTree::Group(g)) => {
                if g.delimiter() != Delimiter::Bracket {
                    bail!(
                        g.span(),
                        format_args!(
                            "Expected documentation, which Rust would automatically translate to `#[doc...`, but the delimiter after `#` was {:#?}",
                            g.delimiter(),
                        ),
                    );
                }
                match g.stream().into_iter().next() {
                    Some(TokenTree::Ident(i)) => {
                        if i != "doc" {
                            bail!(
                                i.span(),
                                "All attributes except docs are automatically derived; please remove these",
                            )
                        }
                    },
                    _ => bail!(
                        g.span(),
                        "All attributes except docs are automatically derived; please remove these",
                    )
                };
                g.to_tokens(&mut out);
            },
            x => bail!(x.span(), format_args!("Expected documentation, which Rust would automatically translate to `#[doc...`, but the group after `#` was {x:#?}")),
        };
    }

    // Shut the fuck up, Clippy
    punct!('#').to_tokens(&mut out);
    Group::new(Delimiter::Bracket, {
        let mut allow = TokenStream::new();
        ident!(allow).to_tokens(&mut allow);
        Group::new(Delimiter::Parenthesis, {
            let mut lints = TokenStream::new();
            ident!(clippy).to_tokens(&mut lints);
            pathsep!(&mut lints);
            ident!(exhaustive_enums).to_tokens(&mut lints);
            punct!(',').to_tokens(&mut lints);
            ident!(clippy).to_tokens(&mut lints);
            pathsep!(&mut lints);
            ident!(exhaustive_structs).to_tokens(&mut lints);
            lints
        })
        .to_tokens(&mut allow);
        allow
    })
    .to_tokens(&mut out);

    // Add derive attributes
    punct!('#').to_tokens(&mut out);
    Group::new(Delimiter::Bracket, {
        let mut derive = TokenStream::new();
        ident!(derive).to_tokens(&mut derive);
        Group::new(Delimiter::Parenthesis, {
            let mut traits = TokenStream::new();
            ident!(Clone).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(Debug).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(Default).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(Eq).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(Hash).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(Ord).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(PartialEq).to_tokens(&mut traits);
            punct!(',').to_tokens(&mut traits);
            ident!(PartialOrd).to_tokens(&mut traits);
            traits
        })
        .to_tokens(&mut derive);
        derive
    })
    .to_tokens(&mut out);

    // Parse `pub` keyword
    if let TokenTree::Ident(i) = doc {
        if i != "pub" {
            bail!(i.span(), "Expected `pub`")
        }
        i.to_tokens(&mut out);
    } else {
        bail!(doc.span(), "Expected `pub`")
    }

    let name = parse_data_structure_def(&mut tokens, &mut out)?;

    parse_bind(&mut tokens, &mut out, &name)?;

    write_shr(&mut out, &name);

    if let Some(extra) = tokens.next() {
        bail!(extra.span(), "Macro should have ended before this token");
    }

    // Return the finished product
    Ok(out)
}

/// Parse everything after `pub`.
/// Takes `enum`, `struct`, & `union`.
fn parse_data_structure_def(
    ts: &mut proc_macro2::token_stream::IntoIter,
    out: &mut TokenStream,
) -> Result<Ident, syn::Error> {
    let structure = ts.next();

    // Get the type's name
    let Some(tt) = ts.next() else {
        bail!(structure.span(), "Expected a name but found the end of the macro")
    };
    let TokenTree::Ident(ref name) = tt else {
        bail!(tt.span(), "Expected a name")
    };

    // Parse the definition
    let Some(tt) = ts.next() else {
        bail!(tt.span(), "Expected a definition in braces, e.g. `{ Nothing, Just(A) }`")
    };
    let TokenTree::Group(ref group) = tt else {
        bail!(tt.span(), "Expected a definition in braces, e.g. `{ Nothing, Just(A) }`")
    };

    let Some(tt) = structure else {
        bail!(tt.span(), "Expected a data structure definition like a `struct`, `enum`, or similar");
    };
    if let TokenTree::Ident(i) = tt {
        if i == "enum" {
            parse_enum(&name, &group, out);
        } else if i == "struct" {
            let semicolon = (group.delimiter() == Delimiter::Parenthesis).then(|| ts.next());
            parse_struct(&name, &group, out);
            if let Some(maybe) = semicolon {
                if let Some(TokenTree::Punct(p)) = maybe {
                    if p.as_char() == ';' {
                        p.to_tokens(out);
                    } else {
                        bail!(p.span(), "Expected a semicolon")
                    }
                } else {
                    bail!(maybe.span(), "Expected a semicolon")
                }
            }
        } else if i == "union" {
            parse_union(&name, &group, out);
        } else {
            bail!(
                i.span(),
                "Expected a data structure definition like a `struct`, `enum`, or similar"
            );
        }
    } else {
        bail!(
            tt.span(),
            "Expected a data structure definition like a `struct`, `enum`, or similar"
        );
    }

    Ok(name.clone())
}

/// Parse an enum, adding `<A>`.
fn parse_enum(name: &Ident, group: &Group, out: &mut TokenStream) {
    // Structure and name
    ident!(enum).to_tokens(out);
    name.to_tokens(out);

    // Generic argument <A>
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);

    // Paste the definition
    group.to_tokens(out);

    ident!(pub).to_tokens(out);
    ident!(use).to_tokens(out);
    name.to_tokens(out);
    pathsep!(out);
    Group::new(Delimiter::Brace, {
        let mut ctors = TokenStream::new();
        for tt in group.stream() {
            if let TokenTree::Ident(i) = tt {
                i.to_tokens(&mut ctors);
                punct!(',').to_tokens(&mut ctors); // trailing doesn't matter
            }
        }
        ctors
    })
    .to_tokens(out);
    punct!(';').to_tokens(out);
}

/// Parse a struct, adding `<A>`.
fn parse_struct(name: &Ident, group: &Group, out: &mut TokenStream) {
    // Structure and name
    ident!(struct).to_tokens(out);
    name.to_tokens(out);

    // Generic argument <A>
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);

    // Paste the definition
    group.to_tokens(out);
}

/// Parse a union, adding `<A>`.
fn parse_union(name: &Ident, group: &Group, out: &mut TokenStream) {
    // Structure and name
    ident!(union).to_tokens(out);
    name.to_tokens(out);

    // Generic argument <A>
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);

    // Paste the definition
    group.to_tokens(out);
}

/// Parse `fn bind(...` and write the `impl` block around it.
#[allow(clippy::too_many_lines)]
fn parse_bind(
    ts: &mut proc_macro2::token_stream::IntoIter,
    out: &mut TokenStream,
    name: &Ident,
) -> Result<(), syn::Error> {
    #![allow(clippy::shadow_unrelated)]

    // Write the `impl` block heading from thin air
    ident!(impl).to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);
    // Punct::new(':', Spacing::Joint).to_tokens(out);
    // punct!(':').to_tokens(out);
    ident!(rsmonad).to_tokens(out);
    pathsep!(out);
    ident!(Monad).to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);
    ident!(for).to_tokens(out);
    name.to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);

    Group::new(Delimiter::Brace, {
        let mut impl_block = TokenStream::new();

        // Add a type constructor from thin air
        ident!(type).to_tokens(&mut impl_block);
        ident!(Constructor).to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(B).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!('=').to_tokens(&mut impl_block);
        name.to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(B).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!(';').to_tokens(&mut impl_block);

        // Parse the `fn` keyword
        let Some(tt) = ts.next() else {
            bail_na!("Expected `fn bind(...` after the data structure definition");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn bind(...` after the data structure definition");
        };
        if i != "fn" {
            bail!(
                i.span(),
                "Expected `fn bind(...` after the data structure definition"
            );
        }
        inline_always!(&mut impl_block);
        i.to_tokens(&mut impl_block);

        // Parse the function name (bind)
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected `fn bind(...` after the data structure definition");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn bind(...` after the data structure definition");
        };
        if i != "bind" {
            bail!(i.span(), "Expected `fn bind(...` after the data structure definition");
        }
        i.to_tokens(&mut impl_block);

        // Add generic arguments out of thin air
        punct!('<').to_tokens(&mut impl_block);
        ident!(B).to_tokens(&mut impl_block);
        punct!(',').to_tokens(&mut impl_block);
        ident!(FromA).to_tokens(&mut impl_block);
        punct!(':').to_tokens(&mut impl_block);
        ident!(From).to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(A).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!(',').to_tokens(&mut impl_block);
        ident!(IntoMonadB).to_tokens(&mut impl_block);
        punct!(':').to_tokens(&mut impl_block);
        ident!(Into).to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        name.to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(B).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!(',').to_tokens(&mut impl_block);
        ident!(F).to_tokens(&mut impl_block);
        punct!(':').to_tokens(&mut impl_block);
        ident!(Fn).to_tokens(&mut impl_block);
        Group::new(Delimiter::Parenthesis, {
            let mut a = TokenStream::new();
            ident!(FromA).to_tokens(&mut a);
            a
        })
        .to_tokens(&mut impl_block);
        Punct::new('-', Spacing::Joint).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        ident!(IntoMonadB).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);

        // Parse arguments
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected `fn bind(...` after the data structure definition");
        };
        let TokenTree::Group(g) = tt else {
            bail!(tt.span(), "Expected `fn bind(...` after the data structure definition");
        };
        if g.delimiter() != Delimiter::Parenthesis {
            bail!(g.span(), "Expected `fn bind(...` after the data structure definition");
        }
        let mut args = g.stream().into_iter();
        let Some(tt) = args.next() else {
            bail!(g.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        if i != "self" {
            bail!(i.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        }
        let Some(tt) = args.next() else {
            bail!(i.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        let TokenTree::Punct(p) = tt else {
            bail!(tt.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        if p.as_char() != ',' {
            bail!(p.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        }
        let Some(tt) = args.next() else {
            bail!(p.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        };
        if i != "f" {
            bail!(i.span(), "Expected `fn bind(self, f) ...` after the data structure definition");
        }
        if let Some(arg) = args.next() {
            bail!(arg.span(), "Argument list should stop at `(self, f)`");
        }
        Group::new(Delimiter::Parenthesis, {
            let mut typed_args = TokenStream::new();
            ident!(self).to_tokens(&mut typed_args);
            punct!(',').to_tokens(&mut typed_args);
            ident!(f).to_tokens(&mut typed_args);
            punct!(':').to_tokens(&mut typed_args);
            ident!(F).to_tokens(&mut typed_args);
            typed_args
        })
        .to_tokens(&mut impl_block);

        // Make sure there's no trailing return type
        let g = match ts.next() {
            None => bail!(i.span(), "Expected a function definition block"),
            Some(TokenTree::Group(g)) => g,
            Some(x) => bail!(x.span(), "Expected a definition block. `bind`'s return type is extremely difficult to write in Rust at the moment, so please let this macro write it for future compatibility.")
        };
        if g.delimiter() != Delimiter::Brace {
            bail!(g.span(), "Expected a function definition block");
        }

        // Add a trailing return type out of thin air
        Punct::new('-', Spacing::Joint).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        name.to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(B).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);

        // Paste the definition verbatim
        g.to_tokens(&mut impl_block);

        // Parse `fn consume(...`
        let Some(tt) = ts.next() else {
            bail!(g.span(), "Expected `fn consume(...` after bind");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn consume(...` after bind");
        };
        if i != "fn" {
            bail!(
                i.span(),
                "Expected `fn consume(...` after bind"
            );
        }
        inline_always!(&mut impl_block);
        i.to_tokens(&mut impl_block);

        // Parse the function name (consume)
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected `fn consume(...` after bind");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn consume(...` after bind");
        };
        if i != "consume" {
            bail!(i.span(), "Expected `fn consume(...` after bind");
        }
        i.to_tokens(&mut impl_block);

        // Parse arguments
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected `fn consume(...` after bind");
        };
        let TokenTree::Group(g) = tt else {
            bail!(tt.span(), "Expected `fn consume(...` after bind");
        };
        if g.delimiter() != Delimiter::Parenthesis {
            bail!(g.span(), "Expected `fn consume(...` after bind");
        }
        let mut args = g.stream().into_iter();
        let Some(tt) = args.next() else {
            bail!(g.span(), "Expected `fn consume(a) -> Self` after bind");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `fn consume(a) -> Self` after bind");
        };
        if i != "a" {
            bail!(i.span(), "Expected `fn consume(a) -> Self` after bind");
        }
        if let Some(a) = args.next() {
            bail!(a.span(), "Argument list should stop at `(a)`");
        }
        punct!('<').to_tokens(&mut impl_block);
        ident!(IntoA).to_tokens(&mut impl_block);
        punct!(':').to_tokens(&mut impl_block);
        ident!(Into).to_tokens(&mut impl_block);
        punct!('<').to_tokens(&mut impl_block);
        ident!(A).to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        punct!('>').to_tokens(&mut impl_block);
        Group::new(Delimiter::Parenthesis, {
            let mut typed_args = TokenStream::new();
            ident!(a).to_tokens(&mut typed_args);
            punct!(':').to_tokens(&mut typed_args);
            ident!(IntoA).to_tokens(&mut typed_args);
            typed_args
        })
        .to_tokens(&mut impl_block);

        // Parse return type (Self)
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        let TokenTree::Punct(p) = tt else {
            bail!(tt.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        if p.as_char() != '-' || p.spacing() != Spacing::Joint {
            bail!(p.span(), "Expected `-> Self` after `fn consume(a)`");
        }
        p.to_tokens(&mut impl_block);
        let Some(tt) = ts.next() else {
            bail!(p.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        let TokenTree::Punct(p) = tt else {
            bail!(tt.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        if p.as_char() != '>' || p.spacing() != Spacing::Alone {
            bail!(p.span(), "Expected `-> Self` after `fn consume(a)`");
        }
        p.to_tokens(&mut impl_block);
        let Some(tt) = ts.next() else {
            bail!(p.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        let TokenTree::Ident(i) = tt else {
            bail!(tt.span(), "Expected `-> Self` after `fn consume(a)`");
        };
        if i != "Self" {
            bail!(i.span(), "Expected `-> Self` after `fn consume(a)`");
        }
        i.to_tokens(&mut impl_block);

        // Parse the definition block and paste it verbatim
        let Some(tt) = ts.next() else {
            bail!(i.span(), "Expected a function definition block after `fn consume(a) -> Self`");
        };
        let TokenTree::Group(g) = tt else {
            bail!(tt.span(), "Expected a function definition block after `fn consume(a) -> Self`");
        };
        g.to_tokens(&mut impl_block);

        impl_block
    }).to_tokens(out);

    Ok(())
}

/// Write a `>>` implementation.
fn write_shr(out: &mut TokenStream, name: &Ident) {
    // `impl<A, B, F: Fn(A) -> B> core::ops::Shr<F> for $name<A>`
    ident!(impl).to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!(',').to_tokens(out);
    ident!(B).to_tokens(out);
    punct!(',').to_tokens(out);
    ident!(F).to_tokens(out);
    punct!(':').to_tokens(out);
    ident!(Fn).to_tokens(out);
    Group::new(Delimiter::Parenthesis, ident!(A).into_token_stream()).to_tokens(out);
    Punct::new('-', Spacing::Joint).to_tokens(out);
    punct!('>').to_tokens(out);
    name.to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(B).to_tokens(out);
    punct!('>').to_tokens(out);
    punct!('>').to_tokens(out);
    ident!(core).to_tokens(out);
    pathsep!(out);
    ident!(ops).to_tokens(out);
    pathsep!(out);
    ident!(Shr).to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(F).to_tokens(out);
    punct!('>').to_tokens(out);
    ident!(for).to_tokens(out);
    name.to_tokens(out);
    punct!('<').to_tokens(out);
    ident!(A).to_tokens(out);
    punct!('>').to_tokens(out);

    // impl block
    Group::new(Delimiter::Brace, {
        let mut ts = TokenStream::new();

        // type Output = $name<B>;
        ident!(type).to_tokens(&mut ts);
        ident!(Output).to_tokens(&mut ts);
        punct!('=').to_tokens(&mut ts);
        name.to_tokens(&mut ts);
        punct!('<').to_tokens(&mut ts);
        ident!(B).to_tokens(&mut ts);
        punct!('>').to_tokens(&mut ts);
        punct!(';').to_tokens(&mut ts);

        // fn shr(...
        ident!(fn).to_tokens(&mut ts);
        ident!(shr).to_tokens(&mut ts);
        Group::new(Delimiter::Parenthesis, {
            let mut args = TokenStream::new();
            ident!(self).to_tokens(&mut args);
            punct!(',').to_tokens(&mut args);
            ident!(f).to_tokens(&mut args);
            punct!(':').to_tokens(&mut args);
            ident!(F).to_tokens(&mut args);
            args
        })
        .to_tokens(&mut ts);
        Punct::new('-', Spacing::Joint).to_tokens(&mut ts);
        punct!('>').to_tokens(&mut ts);
        name.to_tokens(&mut ts);
        punct!('<').to_tokens(&mut ts);
        ident!(B).to_tokens(&mut ts);
        punct!('>').to_tokens(&mut ts);
        Group::new(Delimiter::Brace, {
            let mut body = TokenStream::new();
            ident!(rsmonad).to_tokens(&mut body);
            pathsep!(&mut body);
            ident!(Monad).to_tokens(&mut body);
            pathsep!(&mut body);
            ident!(bind).to_tokens(&mut body);
            Group::new(Delimiter::Parenthesis, {
                let mut args = TokenStream::new();
                ident!(self).to_tokens(&mut args);
                punct!(',').to_tokens(&mut args);
                ident!(f).to_tokens(&mut args);
                args
            })
            .to_tokens(&mut body);
            body
        })
        .to_tokens(&mut ts);

        ts
    })
    .to_tokens(out);
}
