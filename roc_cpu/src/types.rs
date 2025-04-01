use roc_cpu_proc::*;
use roc_cpu_traits::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, ProgramEncodable, ProgramDecodable)]
pub enum RocCPURegister {
    GeneralPurposeA = 0x1,
    GeneralPurposeB = 0x2,
    GeneralPurposeC = 0x3,
    GeneralPurposeD = 0x4,

    ReturnValue = 0x10,

    FunctionParameter1 = 0x20,
    FunctionParameter2 = 0x21,
    FunctionParameter3 = 0x22,
    FunctionParameter4 = 0x23,

    FunctionReturn = 0x30,
}


#[repr(u8)]
#[derive(Clone, Copy, Debug, ProgramEncodable, ProgramDecodable)]
pub enum RocCPUInstruction {
    Add(RocCPURegister, RocCPURegister) = 0x1,
    AddI(RocCPURegister, u8) = 0x2,
    Sub(RocCPURegister, RocCPURegister) = 0x3,
    SubI(RocCPURegister, u8) = 0x4,
    Mul(RocCPURegister, RocCPURegister) = 0x5,
    MulI(RocCPURegister, u8) = 0x6,
    Div(RocCPURegister, RocCPURegister) = 0x7,
    DivI(RocCPURegister, u8) = 0x8,

    SetRet(u8) = 0x20,
    Put(RocCPURegister, u8) = 0x21,
    Mov(RocCPURegister, RocCPURegister) = 0x22,

    // Puts value (arg3) into memory at 0x<arg1><arg2>
    PutMem(u8, u8, u8) = 0x40,
    Push(RocCPURegister) = 0x41,
    Pop(RocCPURegister) = 0x42,

    Exit = 0x80,
    Nop = 0x81,
    Cmp(RocCPURegister, RocCPURegister) = 0x82,

    Jump(u8, u8) = 0xA0,
    JumpIfZero(u8, u8) = 0xA1,

    Call(u8, u8) = 0xB0,
    Return = 0xB1,

    Render = 0xF0,
    Wait(u8) = 0xF1,
}
