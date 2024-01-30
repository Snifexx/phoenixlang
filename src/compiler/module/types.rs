

#[derive(Clone, Copy)]
pub enum Type {
    Void, Bool, Dec, Int, Str
}

impl Type {
    pub fn bytes() -> Vec<u8> {
        // TODO
        vec![]
    }
}
