use std::borrow::{Cow, Borrow};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::str::FromStr;
use crate::vm::value::Pointer;
use crate::vm::{value, Stack};
use crate::{op_codes, vm::{Vm, value::Value}, compiler::chunk::Const};

op_codes! {
    #[derive(PartialEq, Eq)]
    pub enum FBOpCode {
        OpReturn = 0 => 1,
        OpConstant => 4, OpTrue => 1, OpFalse => 1,
        OpPop => 1,
        OpAdd => 1, OpSub => 1, OpMul => 1, OpDiv => 1, OpNeg => 1,
        OpPrint => 1,
        OpGlobSet => 4, OpGlobGet => 4, OpGlobClone => 4,
        OpLocSet => 4, OpLocGet => 4, OpLocClone => 4,
    }
}

pub fn debug(i: u64, slice: &[u8]) {
    macro_rules! oper { ($op:literal $($arg:tt)*) => {{print!("{}", $op); println!($($arg)*);}}; }

    print!("\t{:#010X}\t\t", i);
    match FBOpCode::from(slice[0]) {
        FBOpCode::OpReturn => oper!("OpReturn"),
        FBOpCode::OpConstant => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpConstant" "\t#{}", a);
        }
        FBOpCode::OpPop =>oper!("OpPop"), 
        FBOpCode::OpTrue => oper!("OpTrue"), FBOpCode::OpFalse => oper!("OpFalse"),
        FBOpCode::OpAdd => oper!("OpAdd"), FBOpCode::OpSub => oper!("OpSub"),
        FBOpCode::OpMul => oper!("OpMul"), FBOpCode::OpDiv => oper!("OpDiv"),
        FBOpCode::OpNeg => oper!("OpNeg"),
        FBOpCode::OpPrint => oper!("OpPrint"),
        FBOpCode::OpGlobSet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpGlobSet\t->" "\t#{}", a);
        }
        FBOpCode::OpGlobGet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpGlobGet\t<-" "\t#{}", a);
        } 
        FBOpCode::OpGlobClone => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpGlobClone\t<~" "\t#{}", a);
        } 
        FBOpCode::OpLocSet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpLocSet\t->" "\t#{}", a);
        }
        FBOpCode::OpLocGet => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpLocGet\t<-" "\t#{}", a);
        } 
        FBOpCode::OpLocClone => {
            let a = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}); //u24
            oper!("OpLocClone\t<~" "\t#{}", a);
        } 
    } 
}

pub fn run(vm: &mut Vm, size: usize) -> Option<u8> {

    let slice = &vm.chunk.code[vm.pc as usize..vm.pc as usize + size];

    match FBOpCode::from(slice[0]) {
        FBOpCode::OpReturn => return Some(0),
        FBOpCode::OpConstant => {
            let value = match &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize] { //u24
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
            let val = {
                let second = vm.stack.pop();
                let first = vm.stack.pop().depoint(vm); let first = first.deupvalue(vm);
                let second = second.depoint(vm); let second = second.deupvalue(vm);

                match (&**first, &**second) {
                    (Value::Int(int_f), Value::Int(int_s)) => Value::Int(*int_f + *int_s),
                    (Value::Dec(dec_f), Value::Dec(dec_s)) => Value::Dec(*dec_f + *dec_s),
                    (Value::Str(str), Value::Str(to_concat)) => {
                        let mut new_str = String::from(&**str); new_str.push_str(&**to_concat);
                        drop(first); drop(second);
                        let v = vm.strings.intern_str(&*new_str);
                        Value::Str(v)
                    }
                    (Value::Str(str), Value::Char(to_concat)) => {
                        let mut new_str = String::from(&**str); new_str.push(*to_concat);
                        drop(first); drop(second);
                        let v = vm.strings.intern_str(&*new_str);
                        Value::Str(v)
                    }
                    (_, _) => unreachable!()
                }
            };
            vm.stack.push(val);
        }
        FBOpCode::OpSub => {
            let val = {
                let second = vm.stack.pop();
                let first = vm.stack.pop().depoint(vm); let first = first.deupvalue(vm);
                let second = second.depoint(vm); let second = second.deupvalue(vm);

                match (&**first, &**second) {
                    (Value::Int(int_f), Value::Int(int_s)) => Value::Int(*int_f - *int_s),
                    (Value::Dec(dec_f), Value::Dec(dec_s)) => Value::Dec(*dec_f - *dec_s),
                    (_, _) => unreachable!()
                }
            };
            vm.stack.push(val);
        }
        FBOpCode::OpMul => {
            let val = {
                let second = vm.stack.pop();
                let first = vm.stack.pop().depoint(vm); let first = first.deupvalue(vm);
                let second = second.depoint(vm); let second = second.deupvalue(vm);

                match (&**first, &**second) {
                    (Value::Int(int_f), Value::Int(int_s)) => Value::Int(*int_f * *int_s),
                    (Value::Dec(dec_f), Value::Dec(dec_s)) => Value::Dec(*dec_f * *dec_s),
                    (_, _) => unreachable!()
                }
            };
            vm.stack.push(val);
        }
        FBOpCode::OpDiv => {
            let val = {
                let second = vm.stack.pop();
                let first = vm.stack.pop().depoint(vm); let first = first.deupvalue(vm);
                let second = second.depoint(vm); let second = second.deupvalue(vm);

                match (&**first, &**second) {
                    (Value::Int(int_f), Value::Int(int_s)) => Value::Int(*int_f / *int_s),
                    (Value::Dec(dec_f), Value::Dec(dec_s)) => Value::Dec(*dec_f / *dec_s),
                    (_, _) => unreachable!()
                }
            };
            vm.stack.push(val);
        }
        FBOpCode::OpNeg => {
            let val = {
                let value = vm.stack.pop().depoint(vm); let value = value.deupvalue(vm);

                match &**value {
                    Value::Int(value) => Value::Int(-*value),
                    Value::Dec(value) => Value::Dec(-*value),
                    _ => unreachable!(),
                }
            };
            vm.stack.push(val);
        }
        FBOpCode::OpPrint => { print!("{}", &**vm.stack.pop().depoint(vm).deupvalue(vm)) }
        FBOpCode::OpGlobSet => {
            let name = &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize]; //u24
            let name = if let Const::String(str) = name { str } else { unreachable!() };
            let name = vm.strings.intern_str(name);
            let value = vm.stack.pop().depoint(vm).deupvalue(vm).to_owned();

            match vm.globals.get_mut(&name) {
                Some(Value::Upv(rc)) => *rc.borrow_mut() = value,
                Some(glob) => *glob = value,
                None => { vm.globals.insert(name, value); }
            };
        }
        s @ (FBOpCode::OpGlobGet | FBOpCode::OpGlobClone) => {
            let name = &vm.chunk.consts.as_vm()[u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize]; //u24
            let name = if let Const::String(str) = name { str } else { unreachable!() }; let name = vm.strings.intern_str(name);

            let value = if s == FBOpCode::OpGlobGet {Value::Ptr(Pointer::Global(name))} else {vm.globals[&name].deupvalue(vm).clone()};
            vm.stack.push(value);
        }
        FBOpCode::OpLocSet => {
            let addr = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize; //u24
            let value = vm.stack.pop().depoint(vm).deupvalue(vm).to_owned();

            match &mut vm.stack[addr] {
                Value::Upv(rc) => *rc.borrow_mut() = value,
                _ => vm.with_depnt_upved(addr, |mut val| **val = value)
            };
        }
        s @ (FBOpCode::OpLocGet | FBOpCode::OpLocClone) => {
            let addr = u32::from_le_bytes({let mut a = [0; 4]; a[0..3].copy_from_slice(&slice[1..]); a}) as usize; //u24
            let value = if s == FBOpCode::OpLocGet {vm.denested_pointer(addr)} else {vm.with_depnt_upved(addr, |val| val.clone())};
            vm.stack.push(value);
        }
    }
    None
}








