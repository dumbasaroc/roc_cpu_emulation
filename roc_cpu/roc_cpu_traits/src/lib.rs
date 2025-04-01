pub trait ProgramEncodable where
    Self: Sized
{
    /// Encodes this value into a matching
    /// u8 value, such that we can encode it
    /// into a "binary" of sorts.
    fn encode(&self) -> Vec<u8>;
}


impl ProgramEncodable for u8 {
    fn encode(&self) -> Vec<u8> {
        vec![ *self ]
    }
}


pub trait ProgramDecodable where
    Self: Sized
{
    fn decode(value: u8) -> Self;
}

impl ProgramDecodable for u8 {
    fn decode(value: u8) -> Self {
        value
    }
}


pub trait CPUProgramEncode where Self: Sized {
    type Instruction where Self::Instruction: Sized + ProgramEncodable;
    
    fn variants_to_string(variant: Self) -> String;
    fn encode_to_program(instructions: Vec<Self::Instruction>) -> Vec<u8> where
        Self::Instruction: Sized + ProgramEncodable;
}

pub trait CPUProgramDecode {}
