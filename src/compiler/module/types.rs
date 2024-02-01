use std::fmt::Display;



#[derive(Clone, Copy, Debug)]
pub enum Type {
    Void, Bool, Dec, Int, Str
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Type {
    pub fn bytes() -> Vec<u8> {
        // TODO
        vec![]
    }
}
