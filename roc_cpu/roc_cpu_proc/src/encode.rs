use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::*;

pub fn create_encoder_derive_additions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enumeration
    let DeriveInput{
        ident: enum_ident,
        data: derive_data,
        ..
    } = input;

    let enum_data = match derive_data {
        Data::Enum(e) => e,
        _ => {
            let err = quote_spanned! {enum_ident.span()=>
                compile_error!("ProgramEncodable can only be derived on an Enum.");
            };
            return TokenStream::from(err);
        }
    };

    let output_vector = quote!{ output_vec };
    let variants = enum_data.clone().variants.into_iter().map(|v| {
        v.ident
    });
    let values = enum_data.clone().variants.into_iter().map(|v| {
        v.discriminant.unwrap().1
    });

    let append_lines = enum_data.clone().variants.into_iter().map(|v| {
        let lines = (0..v.fields.len()).into_iter().map(
            |item| {
                let letter_param = (b'a' + item as u8) as char;
                let letter_param = format_ident!("{}", letter_param);
                quote!{
                    #output_vector.append(&mut #letter_param.encode());
                }
            }
        );
        quote!{#(#lines)*}
    });

    let parenthesis_notation = enum_data.clone().variants.into_iter().map(|v| {
        if v.fields.len() == 0 { return quote!(); }
        let params = (0..v.fields.len()).into_iter().map(|i| {
            let letter_param = (b'a' + i as u8) as char;
            let letter_param = format_ident!("{}", letter_param);
            quote!(#letter_param)
        });
        quote!{ (#(#params),*) }
    });

    // Rebuild everything
    let rebuilt = quote! {
        impl ProgramEncodable for #enum_ident {
            fn encode(&self) -> Vec<u8> {
                let mut #output_vector: Vec<u8> = vec![];
                let val = match self {
                    #(#enum_ident::#variants #parenthesis_notation => {
                        #output_vector.push(#values);
                        #append_lines
                    }),*
                };
                #output_vector
            }
        }
    };

    TokenStream::from(rebuilt)
}