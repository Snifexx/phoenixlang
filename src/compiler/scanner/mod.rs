pub mod token;

pub struct Scanner {
    src: String,
    i: usize,
    line: u16, 
}

impl Scanner {
    pub fn new(src: String) -> Self { Self { src, i: 0, line: 0 }}

    pub fn scanToken(mut self) -> 
}




