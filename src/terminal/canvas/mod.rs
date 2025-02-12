use std::fmt::Display;

pub struct Canvas{
    string: String,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            string: ('.'.to_string().repeat(width) + "\r\n").repeat(height)
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
