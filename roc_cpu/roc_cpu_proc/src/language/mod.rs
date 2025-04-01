mod types;
mod util;

pub use types::*;
use quote::quote;
use std::collections::HashMap;


pub fn translate_asm_to_opcode(operation: Operation, labels: &HashMap<String, usize>) -> Option<proc_macro2::TokenStream> {
    let tokens = match operation {

        // ------- LABEL ----------------------//

        Operation::LabelOperation { label } => {
            match label {
                RocCPULiteral::Label(_) => {
                    // We just ignore labels, since we don't
                    // translate them to anything in particular.
                    // They get compiled down to line indexes
                    // at compiletime.
                    return None;
                },
                _ => {
                    // Literally anything else is an error
                    panic!( "Labels can only be strings, not any other literal." );
                }
            }
        },

        // ------- NO ARGUMENT OPCODES ------- //

        Operation::OperationNoArgs { op_name } => {
            let op_name_str = format!("{}", op_name);
            match op_name_str.as_str() {
                "EXIT" => {
                    quote! {
                        RocCPUInstruction::Exit
                    }
                },
                "RENDER" => {
                    quote! {
                        RocCPUInstruction::Render
                    }
                },
                "RETURN" => {
                    quote! {
                        RocCPUInstruction::Return
                    }
                },
                _ => {
                    panic!("{} is not a valid opcode.", op_name);
                }
            }
        },


        // ------- ONE ARGUMENT OPCODES ------- //

        Operation::OperationOneArg { op_name, value_arg1: arg1 } => {
            let op_name_str = format!("{}", op_name);
            match op_name_str.as_str() {
                "CALL" => {
                    match arg1 {
                        RocCPULiteral::Label(lbl) => {
                            if let Some(loc) = labels.get(&lbl) {
                                let loc = *loc as u16;
                                let lo = loc as u8;
                                let hi = (loc >> 8) as u8;

                                quote! {
                                    RocCPUInstruction::Call(#hi, #lo)
                                }
                            } else {
                                panic!( "Label \"{}\" is not defined in this program.", lbl );
                            }
                        },
                        _ => {
                            panic!( "Only labels can be jumped to with one argument." );
                        }
                    }
                },
                "JUMP" => {
                    match arg1 {
                        RocCPULiteral::Label(lbl) => {
                            if let Some(loc) = labels.get(&lbl) {
                                let loc = *loc as u16;
                                let lo = loc as u8;
                                let hi = (loc >> 8) as u8;

                                quote! {
                                    RocCPUInstruction::Jump(#hi, #lo)
                                }
                            } else {
                                panic!( "Label \"{}\" is not defined in this program.", lbl );
                            }
                        },
                        _ => {
                            panic!( "Only labels can be jumped to with one argument." );
                        }
                    }
                },
                "JZ" => {
                    match arg1 {
                        RocCPULiteral::Label(lbl) => {
                            if let Some(loc) = labels.get(&lbl) {
                                let loc = *loc as u16;
                                let lo = loc as u8;
                                let hi = (loc >> 8) as u8;

                                quote! {
                                    RocCPUInstruction::JumpIfZero(#hi, #lo)
                                }
                            } else {
                                panic!( "Label \"{}\" is not defined in this program.", lbl );
                            }
                        },
                        _ => {
                            panic!( "Only labels can be jumped to with one argument." );
                        }
                    }
                },
                "POP" => {
                    quote! {
                        RocCPUInstruction::Pop(#arg1)
                    }
                }
                "PUSH" => {
                    quote! {
                        RocCPUInstruction::Push(#arg1)
                    }
                }
                "SETRET" => {
                    quote! {
                        RocCPUInstruction::SetRet(#arg1)
                    }
                },
                "WAIT" => {
                    quote! {
                        RocCPUInstruction::Wait(#arg1)
                    }
                },
                _ => {
                    panic!("{} is not a valid opcode.", op_name);
                }
            }
        },


        // ------- TWO ARGUMENT OPCODES ------- //

        Operation::OperationTwoArg { op_name, value_arg1: arg1 , value_arg2: arg2 } => {
            let op_name_str = format!("{}", op_name);
            match op_name_str.as_str() {
                "ADD" => {
                    quote! {
                        RocCPUInstruction::Add(#arg1, #arg2)
                    }
                },
                "CALL" => {
                    quote! {
                        RocCPUInstruction::Call(#arg1, #arg2);
                    }
                },
                "CMP" => {
                    quote! {
                        RocCPUInstruction::Cmp(#arg1, #arg2)
                    }
                },
                "JUMP" => {
                    quote! {
                        RocCPUInstruction::Jump(#arg1, #arg2)
                    }
                },
                "JZ" => {
                    quote! {
                        RocCPUInstruction::JumpIfZero(#arg1, #arg2)
                    }
                },
                "MOV" => {
                    quote! {
                        RocCPUInstruction::Mov(#arg1, #arg2)
                    }
                },
                "PUT" => {
                    quote! {
                        RocCPUInstruction::Put(#arg1, #arg2)
                    }
                },
                "SUB" => {
                    quote! {
                        RocCPUInstruction::Sub(#arg1, #arg2)
                    }
                },
                _ => {
                    panic!("{} is not a valid opcode.", op_name);
                }
            }
        },


        // ------- THREE ARGUMENT OPCODES ------- //

        Operation::OperationThreeArg {
            op_name,
            value_arg1: arg1,
            value_arg2: arg2,
            value_arg3: arg3
        } => {
            let op_name_str = op_name.to_string();
            match op_name_str.as_str() {
                "PUTMEM" => {
                    quote! {
                        RocCPUInstruction::PutMem(#arg1, #arg2, #arg3)
                    }
                },
                _ => {
                    panic!("{} is not a valid opcode.", op_name);
                }
            }
        }
    };

    Some(tokens)
}
