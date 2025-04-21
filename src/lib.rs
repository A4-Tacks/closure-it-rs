#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream,
    TokenTree,
};

fn span_setter(span: Span) -> impl Fn(TokenTree) -> TokenTree {
    move |mut tt| {
        tt.set_span(span);
        tt
    }
}

#[must_use]
fn stream<I>(iter: I) -> TokenStream
where I: IntoIterator<Item = TokenTree>,
{
    TokenStream::from_iter(iter)
}

fn err(msg: &str, span: Span) -> TokenStream {
    let s = span_setter(span);
    stream([
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("core", span).into()),
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("compile_error", span).into()),
        s(Punct::new('!', Joint).into()),
        s(Group::new(Delimiter::Brace, stream([
            s(Literal::string(msg).into()),
        ])).into()),
    ])
}

#[derive(Default, Clone)]
struct Closure<'a> {
    it: Option<TokenTree>,
    catch_it: &'a str,
}
impl Closure<'_> {
    fn make_closure(&mut self) -> TokenStream {
        let Some(it) = self.it.take() else {
            return TokenStream::new();
        };
        let s = span_setter(it.span());
        stream([
            s(Punct::new('|', Joint).into()),
            it,
            s(Punct::new('|', Joint).into()),
        ])
    }

    fn ext_proc_it(&mut self, input: TokenStream) -> TokenStream {
        let ext = &mut Self { catch_it: self.catch_it, ..Default::default() };
        let proc_it = ext.proc_it(input);
        ext.make_closure().into_iter().chain(proc_it).collect()
    }

    fn proc_it(&mut self, input: TokenStream) -> TokenStream {
        let iter = &mut input.into_iter().peekable();
        let mut result = TokenStream::new();

        while let Some(tt) = iter.next() {
            match tt {
                TokenTree::Group(group)
                    if group.delimiter() == Delimiter::Parenthesis =>
                {
                    let grouped = self.ext_proc_it(group.stream());
                    result.extend([
                        Group::new(group.delimiter(), grouped).into(),
                    ] as [TokenTree; 1]);
                },
                TokenTree::Group(group) => {
                    let grouped = self.proc_it(group.stream());
                    result.extend([
                        Group::new(group.delimiter(), grouped).into(),
                    ] as [TokenTree; 1]);
                },
                TokenTree::Ident(ref ident)
                    if ident.to_string() == self.catch_it =>
                {
                    result.extend([self.it.get_or_insert(tt).clone()]);
                },
                TokenTree::Ident(_) | TokenTree::Literal(_) => {
                    result.extend([tt]);
                },
                TokenTree::Punct(ref punct)
                    if matches!(punct.as_char(), ',' | ';') =>
                {
                    result.extend([tt]);
                    result.extend(self.ext_proc_it(iter.collect()));
                },
                TokenTree::Punct(ref punct)
                    if punct.as_char() == '='
                        && punct.spacing() == Joint
                        && iter.peek().is_some_and(|p| {
                            matches!(p, TokenTree::Punct(p)
                                if p.as_char() == '>')
                        }) =>
                {
                    result.extend([tt, iter.next().unwrap()]);
                    result.extend(self.ext_proc_it(iter.collect()));
                },
                TokenTree::Punct(_) => {
                    result.extend([tt]);
                },
            }
        }

        result
    }
}

fn get_catch_it<F>(attr: TokenStream, f: F) -> TokenStream
where F: FnOnce(&str) -> TokenStream,
{
    let iter = &mut attr.into_iter();
    let catch_it = match iter.next() {
        Some(TokenTree::Ident(ident)) => &*ident.to_string(),
        Some(attr) => return err("invalid input", attr.span()),
        _ => "it",
    };
    if let Some(extra) = iter.next() {
        return err("invalid input", extra.span());
    }
    f(catch_it)
}

/// Replace `it` to closure body,
/// expand the closure parameter after `,` `;` `=>` and `(`
///
/// # Examples
/// ```
/// #[closure_it::closure_it]
/// fn main() {
///     assert_eq!([0i32, 1, 2].map(it+2), [2, 3, 4]);
///     assert_eq!([0i32, -1, 2].map(it.abs()), [0, 1, 2]);
///     assert_eq!(Some(2).map_or(3, it*2), 4);
/// }
/// ```
///
/// ```
/// #[closure_it::closure_it(this)]
/// fn main() {
///     assert_eq!([0i32, 1, 2].map(this+2), [2, 3, 4]);
///     assert_eq!([0i32, -1, 2].map(this.abs()), [0, 1, 2]);
///     assert_eq!(Some(2).map_or(3, this*2), 4);
/// }
/// ```
#[proc_macro_attribute]
pub fn closure_it(attr: TokenStream, item: TokenStream) -> TokenStream {
    get_catch_it(attr, |catch_it| {
        Closure { catch_it, ..Default::default() }
            .ext_proc_it(item)
    })
}
