extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, LitInt};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;

struct ImplWithIdents {
    macro_ident: Ident,
    custom_ident: Ident,
    start: usize,
    end: usize,
}

impl Parse for ImplWithIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_ident = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let custom_ident = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let start = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        let end = input.parse::<LitInt>()?.base10_parse()?;

        return Ok(Self { macro_ident, custom_ident, start, end })
    }
}

/// Call macro with given ident repeated from `start` to `end` times.
///
/// # Example:
/// ```rust
/// use macros::impl_with_idents;
///
/// // This
/// impl_with_idents!(my_macro, T, 0, 3);
///
/// // will be converted to the
/// my_macro!();
/// my_macro!(T0);
/// my_macro!(T0, T1);
/// ```
#[proc_macro]
pub fn impl_with_idents(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplWithIdents);

    let custom_idents = (input.start..input.end)
        .map(|i| format_ident!("{}{}", &input.custom_ident, i))
        .collect::<Vec<_>>();

    let macro_ident = &input.macro_ident;
    let invocations = (input.start..input.end).map(|i| {
        let custom_idents = &custom_idents[0..i];
        quote! {
            #macro_ident!(#(#custom_idents),*);
        }
    });

    TokenStream::from(quote! { #(#invocations)*})
}
