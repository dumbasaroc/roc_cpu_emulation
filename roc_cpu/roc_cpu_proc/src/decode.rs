use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::*;


pub fn create_decoder_derive_additions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        ident: enum_ident,
        data,
        .. } = input;

    let data = match data {
        Data::Enum(e) => e,
        _ => {
            let err = quote_spanned! {enum_ident.span()=>
                compile_error!("ProgramDecodable can only be derived on an Enum.");
            };
            return TokenStream::from(err);
        }
    };

    let rebuilt = quote! {
        impl ProgramDecodable for #enum_ident {
            fn decode(value: u8) -> Self {
                panic!("Oh no.");
            }
        }
    };

    TokenStream::from(rebuilt)
}