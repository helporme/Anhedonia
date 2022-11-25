extern crate proc_macro;

use proc_macro::{TokenStream};

use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt, Ident};
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma};

struct ImplWithIdents {
    macro_ident: Ident,
    custom_idents: Vec<Ident>,
    start: usize,
    end: usize,
}

impl Parse for ImplWithIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_ident = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let start = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        let end = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;

        let mut custom_idents = vec![input.parse::<Ident>()?];
        while input.parse::<Comma>().is_ok() {
            custom_idents.push(input.parse::<Ident>()?);
        }

        return Ok(Self { macro_ident, custom_idents, start, end })
    }
}

/// Call macro with given idents repeated from `start` to `end` times.
///
/// # Example:
/// ```rust
/// use macros::impl_with_idents;
///
/// // This
/// impl_with_idents!(my_macro, 0, 3, T, U);
///
/// // will be converted to the
/// my_macro!();
/// my_macro!(T0, U0);
/// my_macro!(T0, U0, T1, U1);
/// ```
#[proc_macro]
pub fn impl_with_idents(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplWithIdents);
    let custom_ident_count = input.custom_idents.len();

    let mut all_idents: Vec<Ident> = Vec::with_capacity(
        custom_ident_count * (input.end - input.start));

    for i in input.start..input.end {
        for custom_ident in input.custom_idents.iter() {
            all_idents.push(format_ident!("{}{}", custom_ident, i));
        }
    }

    let macro_ident = &input.macro_ident;
    let invocations = (input.start..input.end).map(|i| {
        let custom_idents = &all_idents[(0..i*custom_ident_count)];
        quote! {
            #macro_ident!(#(#custom_idents),*);
        }
    });

    TokenStream::from(quote! { #(#invocations)*})
}
