#[macro_export]

macro_rules! op_codes {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)? => $size:literal,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl $name {
            pub fn size() -> Vec<u8> { vec![$($size,)*] }
        }

        impl std::convert::From<u8> for $name {
            fn from(v: u8) -> Self {
                match v {
                    $(x if x == $name::$vname as u8 => $name::$vname,)*
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub struct OwnedChars {
    s: String,
    index: usize,
}

impl OwnedChars {
    pub fn new(s: String) -> Self {
        Self { s, index: 0 }
    }
}

impl Iterator for OwnedChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Slice of leftover characters
        let slice = &self.s[self.index..];

        // Iterator over leftover characters
        let mut chars = slice.chars();

        // Query the next char
        let next_char = chars.next()?;

        // Compute the new index by looking at how many bytes are left
        // after querying the next char
        self.index = self.s.len() - chars.as_str().len();

        // Return next char
        Some(next_char)
    }
}

pub trait StringExt {
    fn into_chars(self) -> OwnedChars;
}
impl StringExt for String {
    fn into_chars(self) -> OwnedChars {
        OwnedChars::new(self)
    }
}
