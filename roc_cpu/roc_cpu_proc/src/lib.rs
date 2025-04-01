use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod decode;
mod encode;
mod language;

#[proc_macro_derive(ProgramEncodable)]
pub fn encodable_derive(item: TokenStream) -> TokenStream {
    encode::create_encoder_derive_additions(item)
}

#[proc_macro_derive(ProgramDecodable)]
pub fn decodable_derive(item: TokenStream) -> TokenStream {
    decode::create_decoder_derive_additions(item)
}

#[proc_macro]
pub fn roc_asm(input: TokenStream) -> TokenStream {
    
    let token_data = parse_macro_input!(input as language::Program);
    let labels = token_data.clone().labels;

    
    let final_program = token_data.clone().operations.into_iter().filter_map(
        move |op| {
            language::translate_asm_to_opcode(op, &labels)
        }
    );
    
    let q = quote! {
        vec![
            #(#final_program),*
        ]
    };
    TokenStream::from(q)
}
