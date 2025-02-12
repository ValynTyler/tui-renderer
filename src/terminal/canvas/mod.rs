use std::fmt::Display;

pub struct Canvas{
    string: String,
    _width: usize,
    _height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            string: ('.'.to_string().repeat(width) + "\r\n").repeat(height - 1) + &'.'.to_string().repeat(width),
            _width: width,
            _height: height,
        }
    }

    pub fn width(&self) -> usize {
        self._width
    }

    pub fn height(&self) -> usize {
        self._height
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
