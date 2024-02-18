use std::{collections::HashSet, hash::BuildHasherDefault, sync::Arc, rc::Rc};

use ahash::AHasher;

pub struct InternStrSync(HashSet<Arc<str>, BuildHasherDefault<AHasher>>);

impl InternStrSync {
    pub fn new() -> Self { InternStrSync(Default::default()) }

    pub fn intern_str(&mut self, str: &str) -> Arc<str> { 
        match self.0.get(str) {
            Some(str) => Arc::clone(str),
            None => { let rc = Arc::<str>::from(str); self.0.insert(rc.clone()); rc }
        }
    }
}

pub struct InternStr(HashSet<Rc<str>, BuildHasherDefault<AHasher>>);

impl InternStr {
    pub fn new() -> Self { InternStr(Default::default()) }

    pub fn intern_str(&mut self, str: &str) -> Rc<str> { 
        match self.0.get(str) {
            Some(str) => Rc::clone(str),
            None => { let rc = Rc::<str>::from(str); self.0.insert(rc.clone()); rc }
        }
    }
}
