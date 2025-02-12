use std::fmt::Display;

pub struct Canvas{
    string: String,
    _width: usize,
    _height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize, fill: char) -> Self {
        Canvas {
            string: (fill.to_string().repeat(width) + "\r\n").repeat(height - 1) + &fill.to_string().repeat(width),
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

    pub fn get(&self, x: usize, y: usize) -> char {
        let row_len = self.width() + 2;
        let index = y * row_len + x;

        self.string.char_indices().nth(index).unwrap().1
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        let row_len = self.width() + 2;
        let index = y * row_len + x;

        let mut chars: Vec<char> = self.string.chars().collect();
        chars[index] = value;

        self.string = chars.iter().collect();
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
