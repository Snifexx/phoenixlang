use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::vm::{value, Stack};
use crate::{op_codes, vm::{Vm, value::Value}, compiler::chunk::Const};

op_codes! {
    pub enum FBOpCode {
        OpReturn = 0 => 1,
        OpConstant => 4, OpTrue => 1, OpFalse => 1,
        OpPop => 1,
        OpAdd => 1, OpSub => 1, OpMul => 1, OpDiv => 1, OpNeg => 1,
        OpPrint => 1,
        OpGlobSet => 4, OpGlobGet => 4,
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
        FBOpCode::OpPop =>oper!("OpPop"), 
        FBOpCode::OpTrue => oper!("OpTrue"), FBOpCode::OpFalse => oper!("OpFalse"),
        FBOpCode::OpAdd => oper!("OpAdd"), FBOpCode::OpSub => oper!("OpSub"),
        FBOpCode::OpMul => oper!("OpMul"), FBOpCode::OpDiv => oper!("OpDiv"),
        FBOpCode::OpNeg => oper!("OpNeg"),
        FBOpCode::OpPrint => oper!("OpPrint"),
        FBOpCode::OpGlobSet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a});
            oper!("OpGlobSet\t->" "\t#{}", a);
        }
        FBOpCode::OpGlobGet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a});
            oper!("OpGlobGet\t<-" "\t#{}", a);
        } 
    } 
}

pub fn run(vm: &mut Vm, size: usize) -> Option<u8> {

    let slice = &vm.chunk.code[vm.pc as usize..vm.pc as usize + size];

    match FBOpCode::from(slice[0]) {
        FBOpCode::OpReturn => return Some(0),
        FBOpCode::OpConstant => {
            let value = match &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize] {
                Const::Int(v) => Value::Int(*v),
                Const::Dec(v) => Value::Dec(f64::from_bits(*v)),
                Const::String(v) => {
                    let v = vm.strings.intern_str(v);
                    Value::Str(v)
                },
                Const::Char(c) => Value::Char(*c),
            }; vm.stack.push(value) }
        FBOpCode::OpTrue => vm.stack.push(Value::Bool(true)), FBOpCode::OpFalse => vm.stack.push(Value::Bool(false)),
        FBOpCode::OpPop => { vm.stack.pop(); }
        FBOpCode::OpAdd => {
            let second = vm.stack.pop();
            let first = vm.stack.pop();
            
            match (first, second) {
                (Value::Int(int_f), Value::Int(int_s)) => vm.stack.push(Value::Int(int_f + int_s)),
                (Value::Dec(dec_f), Value::Dec(dec_s)) => vm.stack.push(Value::Dec(dec_f + dec_s)),
                (Value::Str(str), Value::Str(to_concat)) => {
                    let mut new_str = String::from(&*str); new_str.push_str(&*to_concat);
                    let v = vm.strings.intern_str(&*new_str);
                    vm.stack.push(Value::Str(v));
                }
                (Value::Str(str), Value::Char(to_concat)) => {
                    let mut new_str = String::from(&*str); new_str.push(to_concat);
                    let v = vm.strings.intern_str(&*new_str);
                    vm.stack.push(Value::Str(v));
                }
                (_, _) => unreachable!()
            };
        }
        FBOpCode::OpSub => {
            let second = vm.stack.pop();
            let first = vm.stack.pop();
            
            match (first, second) {
                (Value::Int(int_f), Value::Int(int_s)) => vm.stack.push(Value::Int(int_f - int_s)),
                (Value::Dec(dec_f), Value::Dec(dec_s)) => vm.stack.push(Value::Dec(dec_f - dec_s)),
                (_, _) => unreachable!()
            };
        }
        FBOpCode::OpMul => {
            let second = vm.stack.pop();
            let first = vm.stack.pop();
            
            match (first, second) {
                (Value::Int(int_f), Value::Int(int_s)) => vm.stack.push(Value::Int(int_f * int_s)),
                (Value::Dec(dec_f), Value::Dec(dec_s)) => vm.stack.push(Value::Dec(dec_f * dec_s)),
                (_, _) => unreachable!()
            };
        }
        FBOpCode::OpDiv => {
            let second = vm.stack.pop();
            let first = vm.stack.pop();
            
            match (first, second) {
                (Value::Int(int_f), Value::Int(int_s)) => vm.stack.push(Value::Int(int_f / int_s)),
                (Value::Dec(dec_f), Value::Dec(dec_s)) => vm.stack.push(Value::Dec(dec_f / dec_s)),
                (_, _) => unreachable!()
            };
        }
        FBOpCode::OpNeg => {
            let value = vm.stack.pop();

            match value {
                Value::Int(value) => vm.stack.push(Value::Int(-value)),
                Value::Dec(value) => vm.stack.push(Value::Dec(-value)),
                _ => unreachable!(),
            }
        }
        FBOpCode::OpPrint => { print!("{}", vm.stack.pop()) }
        FBOpCode::OpGlobSet => {
            let name = &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize];
            let name = if let Const::String(str) = name { str } else { unreachable!() };
            let name = vm.strings.intern_str(name);
            let value = vm.stack.pop();
            vm.globals.insert(name, value);
        }
        FBOpCode::OpGlobGet => {
            let name = &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize];
            let name = if let Const::String(str) = name { str } else { unreachable!() };
            let name = vm.strings.intern_str(name);
            vm.stack.push(vm.globals[&name].clone());
        }
    }
    None
}








