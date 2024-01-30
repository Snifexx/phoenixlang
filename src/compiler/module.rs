use std::string::String;
use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::{error::PhoenixError, debug::debug_chunk};

use self::types::Type;

use crate::FBOpCode::*;
use super::{token::{Token, self, TokenType::*}, chunk::Chunk};

mod types;

pub struct Module {
    id: Rc<String>,
    tokens: Vec<Token>, i: usize,
    imports: FxHashMap<Rc<String>, Rc<String>>,
    items: Vec<Items>,
    // TODO temp
    chunk: Chunk,
}

struct Items {
    name: Rc<String>,
    code: Chunk,
    dependencies: FxHashSet<Rc<String>>,
}

impl Module {
    pub fn new(tokens: Vec<Token>, id: Rc<String>) -> Self { Self { tokens, id, i: 0, imports:  FxHashMap::default(), items: Vec::new(), chunk: Chunk::new() }}
    #[inline(always)]
    pub fn curr_tok(&mut self) -> &mut Token { &mut self.tokens[self.i] }
    
    pub fn compile(mut self, interned_str: &mut FxHashSet<Rc<String>>) -> Result<(), PhoenixError> {
        
        while self.tokens[self.i].ty != Eof {
            self.expression(interned_str)?;
        }
        

        debug_chunk(&self.chunk.build());
        Ok(())
    }

    pub fn expression(&mut self, interned_str: &mut FxHashSet<Rc<String>>) -> Result<Type, PhoenixError> {
        let mut lht = match self.curr_tok().ty {
            True | False => self.bool(),
            _ => unreachable!(),
        };
        self.i += 1;
        Ok(lht)
    }
    
    #[inline(always)]
    fn bool(&mut self) -> Type { let op =if self.curr_tok().ty == True { OpTrue } else { OpFalse }; self.chunk.write_op(op); Type::Bool }
}


#[cfg(test)]
mod test {
    use crate::error::PhoenixError;
    use std::rc::Rc;

    use rustc_hash::FxHashSet;

    use crate::compiler::{Compiler, scanner::Scanner};

    use super::Module;

    #[test]
    fn chunk_print() -> Result<(), PhoenixError> {
        let input = " true false ".to_string();
        Module::new(Scanner::new(input).scan()?, Rc::new("test".to_string())).compile(&mut FxHashSet::default());
        Ok(())
    }
}



