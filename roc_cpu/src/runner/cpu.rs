use crate::types::*;
use crate::runner::display::*;

pub struct RocCPURunner {
    display: RocCPUDisplay,

    registers: [u8; 10],
    program: Option<Vec<RocCPUInstruction>>,
    memory: [u8; 0x10000],
    stack: [u8; 0xFF],

    // State things
    should_continue: bool,
    program_counter: usize,

    /// This points at the value one idx ahead
    /// of the most recently populated stack val
    stack_pointer: usize,
    pc_manually_set: bool,

    // Flags
    zero_flag: bool,
}

impl Default for RocCPURunner {
    fn default() -> Self {
        Self {
            display: RocCPUDisplay::new(),

            registers: [0; 10],
            program: None,
            memory: [0; 0x10000],
            stack: [0; 0xFF],

            should_continue: true,
            program_counter: 0,
            stack_pointer: 0,
            pc_manually_set: false,

            zero_flag: false,
        }
    }
}

// Public API Implementations
impl RocCPURunner {

    pub fn new(program: Option<&Vec<RocCPUInstruction>>) -> Self {

        let mut s = Self {
            ..Default::default()
        };

        if program.is_some() {
            let newprogram = program.unwrap().clone();
            s.program = Some(newprogram);
        }

        s
    }


    pub fn load_program(&mut self, program: &Vec<RocCPUInstruction>) {
        self.program = Some(program.clone());
    }

    pub fn unload_program(&mut self) {
        self.program = None;
    }

    pub fn execute(&mut self) -> u8 {
        self.reset_execution_stuff();

        if self.program.is_none() {
            return 0;
        }

        self.execution_mainloop();
        self.get_register_value(RocCPURegister::ReturnValue)
    }
}


// Execution Mainloop

impl RocCPURunner {

    fn execution_mainloop(&mut self) {
        
        'execution_loop: loop {

            if self.program.as_ref().unwrap().len() <= self.program_counter {
                self.set_register_value(RocCPURegister::ReturnValue, 255);
                break 'execution_loop;
            }

            let opcode = self.program.as_ref().unwrap()[self.program_counter];
            self.execute_opcode(opcode);

            if !self.should_continue {
                break 'execution_loop;
            }

            if !self.pc_manually_set {
                self.program_counter += 1;
            }
            self.pc_manually_set = false;
        }
    }

    fn push_value_to_stack(&mut self, val: u8) {
        if self.stack_pointer == self.stack.len() {
            panic!( "Tried to push a value onto a full stack." );
        }

        self.stack[self.stack_pointer] = val;
        self.stack_pointer += 1;
    }

    fn pop_value_from_stack(&mut self) -> u8 {
        if self.stack_pointer == 0 {
            panic!( "Tried to pop a value from an empty stack." );
        }

        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }


    fn execute_opcode(&mut self, opcode: RocCPUInstruction) {

        use crate::RocCPUInstruction::*;
        use crate::RocCPURegister::*;

        match opcode {

            // Arithmetic

            Add(dst, src) => {
                let dst_val = self.get_register_value(dst);
                let src_val = self.get_register_value(src);
                let retval = match dst_val.checked_add(src_val) {
                    Some(val) => val,
                    None => { dst_val.wrapping_add(src_val) }
                };

                self.set_register_value(dst, retval);

                self.zero_flag = retval == 0;
            },

            Sub(dst, src) => {
                let dst_val = self.get_register_value(dst);
                let src_val = self.get_register_value(src);
                let retval = match dst_val.checked_sub(src_val) {
                    Some(val) => val,
                    None => { dst_val.wrapping_sub(src_val) }
                };
                self.set_register_value(dst, retval);

                self.zero_flag = retval == 0;
            },

            // Setting registers
                       
            SetRet(val) => {
                self.set_register_value(ReturnValue, val);
            },

            Put(reg, val) => {
                self.set_register_value(reg, val);
            },

            Mov(dst, src) => {
                let val = self.get_register_value(src);
                self.set_register_value(dst, val);
            },

            // Execution Control

            Exit => {
                self.should_continue = false;
            },
            Nop => { /* Literally do nothing */ },
            Cmp(reg1, reg2) => {
                let val1 = self.get_register_value(reg1);
                let val2 = self.get_register_value(reg2);

                match val1.checked_sub(val2) {
                    Some(res) => {
                        if res == 0 { self.zero_flag = true; }
                        else { self.zero_flag = false; }
                    },
                    None => { self.zero_flag = false; }
                }
            },


            // Testing things
            
            PutMem(hi, lo, val) => {
                let hi = (hi as usize) << 8;
                let address: usize = hi + lo as usize;
                self.memory[address] = val;
            },

            Push(reg) => {
                self.push_value_to_stack(self.get_register_value(reg));
            },

            Pop(reg) => {
                let val = self.pop_value_from_stack();
                self.set_register_value(reg, val);
            }

            Render => {
                self.display.render_current(
                    &self.memory[DISPLAY_MEMORY_START..DISPLAY_MEMORY_END]
                );
            },

            Wait(secs) => {
                std::thread::sleep(std::time::Duration::from_secs(secs as u64));
            },

            Jump(hi, lo) => {
                let hi = (hi as usize) << 8;
                let address: usize = hi + lo as usize;
                self.program_counter = address;
                self.pc_manually_set = true;
            },
            JumpIfZero(hi, lo) => {
                if self.zero_flag {
                    let hi = (hi as usize) << 8;
                    let address: usize = hi + lo as usize;
                    self.program_counter = address;
                    self.pc_manually_set = true;
                }
            },

            Call(hi, lo) => {
                let hi = (hi as usize) << 8;
                let address: usize = hi + lo as usize;

                // First, push current PC to stack
                // (Bottom of stack) [ .., lobytes, hibytes, .. ] (Top of Stack)
                let to_return_to = self.program_counter + 1;
                let pc_lo = to_return_to as u8;
                let pc_hi = (to_return_to >> 8) as u8;
                self.push_value_to_stack(pc_lo);
                self.push_value_to_stack(pc_hi);

                self.program_counter = address;
                self.pc_manually_set = true;
            },

            Return => {
                let hi = self.pop_value_from_stack();
                let lo = self.pop_value_from_stack();
                let address = ((hi as usize) << 8) + lo as usize;
                self.program_counter = address;
                self.pc_manually_set = true;
            }

            _ => {}
        }
    }

}


// Private Implementations
impl RocCPURunner {

    fn reset_execution_stuff(&mut self) {
        self.program_counter = 0;
        self.should_continue = true;
        self.stack = [0; 0xFF];
    }

    fn get_register_idx(&self, register: RocCPURegister) -> usize {
        use crate::RocCPURegister::*;
        
        match register {
            GeneralPurposeA => 0,
            GeneralPurposeB => 1,
            GeneralPurposeC => 2,
            GeneralPurposeD => 3,
            ReturnValue => 4,

            FunctionParameter1 => 5,
            FunctionParameter2 => 6,
            FunctionParameter3 => 7,
            FunctionParameter4 => 8,
            FunctionReturn => 9,
        }
    }

    fn get_register_value(&self, register: RocCPURegister) -> u8 {
        let idx = self.get_register_idx(register);
        self.registers[idx]
    }

    fn set_register_value(&mut self, register: RocCPURegister, value: u8) {
        let idx = self.get_register_idx(register);
        self.registers[idx] = value;
    }
}
