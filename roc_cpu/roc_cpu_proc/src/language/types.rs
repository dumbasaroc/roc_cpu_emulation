use syn::*;
use std::collections::HashMap;

// OVERALL TYPE //

#[derive(Clone)]
pub struct Program {
    pub operations: Vec<Operation>,
    pub labels: HashMap<String, usize>,
}

impl syn::parse::Parse for Program {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut prgm = Self { operations: vec![], labels: HashMap::new() };
        
        while !input.is_empty() {
            let op: Operation = input.parse()?;
            match op {
                Operation::LabelOperation { label } => {
                    match label {
                        RocCPULiteral::Label(lbl) => {
                            prgm.labels.insert(lbl, prgm.operations.len());
                        },
                        _ => {
                            panic!( "Invalid label detected in compilation." );
                        }
                    }
                },
                _ => {
                    prgm.operations.push(op);
                }
            }
        }

        Ok(prgm)
    }
}


// OPERATION TYPE //


#[derive(Clone)]
pub enum Operation {
    LabelOperation { label: RocCPULiteral },
    OperationNoArgs { op_name: Ident },
    OperationOneArg { op_name: Ident, value_arg1: RocCPULiteral },
    OperationTwoArg { op_name: Ident, value_arg1: RocCPULiteral, value_arg2: RocCPULiteral },
    OperationThreeArg { op_name: Ident, value_arg1: RocCPULiteral, value_arg2: RocCPULiteral, value_arg3: RocCPULiteral },
}


impl syn::parse::Parse for Operation {
    fn parse(input: parse::ParseStream) -> Result<Self> {

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![@]) {
            let label: RocCPULiteral = input.parse()?;
            return Ok(Self::LabelOperation { label });
        }

        let op_name: Ident = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            return Ok(Self::OperationNoArgs { op_name });
        }
        
        // At least one argument
        let value_arg1: RocCPULiteral = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            return Ok(Self::OperationOneArg { op_name, value_arg1 });
        }

        // At least two arguments
        input.parse::<Token![,]>()?;
        let value_arg2: RocCPULiteral = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            return Ok(Self::OperationTwoArg { op_name, value_arg1, value_arg2 });
        }

        // Three arguments
        input.parse::<Token![,]>()?;
        let value_arg3: RocCPULiteral = input.parse()?;
        input.parse::<Token![;]>()?;

        Ok( Self::OperationThreeArg { op_name, value_arg1, value_arg2, value_arg3 } )
    }
}

// LITERAL TYPE (DATA AND REGISTERS)

#[derive(Clone)]
pub enum RocCPULiteral {
    Number(u8),
    Register(proc_macro2::TokenStream),
    Label(String),
}

impl syn::parse::Parse for RocCPULiteral {

    fn parse(input: parse::ParseStream) -> Result<Self> {
        
        use crate::language::util::register_ident_to_quote;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![$]) {
            // This is a register
            input.parse::<Token![$]>()?;
            let reg_ident: Ident = input.parse()?;
            
            return Ok(RocCPULiteral::Register(
                register_ident_to_quote(reg_ident)
            ));
        }

        if lookahead.peek(Token![@]) {
            // This is a label
            input.parse::<Token![@]>()?;
            let label: Ident = input.parse()?;

            return Ok(RocCPULiteral::Label(
                label.to_string()
            ));
        }

        let num_lit: LitInt = input.parse()?;
        Ok(Self::Number(num_lit.base10_parse::<u8>().unwrap()))
    }
}

impl quote::ToTokens for RocCPULiteral {

    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let extra_tokens = match self {
            Self::Register(reg) => reg.clone(),
            Self::Label(lab) => quote::quote!( #lab ),
            Self::Number(n) => quote::quote!( #n )
        };
        
        tokens.extend(extra_tokens);
    }
}
