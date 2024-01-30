use crate::vm::Stacktrait;
use crate::{op_codes, vm::{Vm, value::Value}, compiler::chunk::Const};

op_codes! {
    pub enum FBOpCode {
        OpReturn = 0 => 1,
        OpConstant => 4, OpTrue => 1, OpFalse => 1,
    }
}

pub fn debug(i: u64, slice: &[u8]) {
    macro_rules! oper { ($op:literal $($arg:tt)*) => {{print!("{}", $op); println!($($arg)*);}}; }

    print!("\t{:#010X}\t\t", i);
    match FBOpCode::from(slice[0]) {
        FBOpCode::OpReturn => oper!("OpReturn"),
        FBOpCode::OpConstant => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a});
            oper!("OpConstant" "\t#{}", a);
        }
        FBOpCode::OpTrue => oper!("OpTrue"), FBOpCode::OpFalse => oper!("OpFalse"),
    } 
}

pub fn run(vm: &mut Vm, size: usize) -> Option<u8> {
    let slice = &vm.chunk.code[vm.pc as usize..vm.pc as usize + size];

    match FBOpCode::from(slice[0]) {
        FBOpCode::OpReturn => return Some(0),
        FBOpCode::OpConstant => {
            let value = match &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize] {
                Const::Int(v) => Value::Int(*v),
                Const::Dec(v) => Value::Dec(*v as f64),
                Const::String(v) => Value::Str(Box::new(v.clone()))
            }; vm.stack.push_val(value)
        }
        FBOpCode::OpTrue => vm.stack.push_val(Value::Bool(true)), FBOpCode::OpFalse => vm.stack.push_val(Value::Bool(false)),
    }
    None
}








