use syn::Ident;
use quote::quote;

pub fn register_ident_to_quote(register: Ident) -> proc_macro2::TokenStream {

    let register_name = register.to_string();
    match register_name.as_str() {

        // RETURN REGISTER

        "ret" => {
            quote! {
                RocCPURegister::ReturnValue
            }
        },

        // GENERAL PURPOSE REGISTERS

        "ax" => {
            quote! {
                RocCPURegister::GeneralPurposeA
            }
        },
        "bx" => {
            quote! {
                RocCPURegister::GeneralPurposeB
            }
        },
        "cx" => {
            quote! {
                RocCPURegister::GeneralPurposeC
            }
        },
        "dx" => {
            quote! {
                RocCPURegister::GeneralPurposeD
            }
        },

        // FUNCTION ARGUMENT REGISTERS

        "f1" => {
            quote! {
                RocCPURegister::FunctionParameter1
            }
        },
        "f2" => {
            quote! {
                RocCPURegister::FunctionParameter2
            }
        },
        "f3" => {
            quote! {
                RocCPURegister::FunctionParameter3
            }
        },
        "f4" => {
            quote! {
                RocCPURegister::FunctionParameter4
            }
        },

        "fret" => {
            quote! {
                RocCPURegister::FunctionReturn
            }
        },


        _ => {
            panic!("Register \"${}\" is not a valid register.", register_name);
        }

    }

}