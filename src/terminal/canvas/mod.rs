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

    pub fn get(&self, pos: (usize, usize)) -> char {
        let row_len = self.width() + 2;
        let index = pos.1 * row_len + pos.0;

        self.string.char_indices().nth(index).unwrap().1
    }

    pub fn set(&mut self, pos: (usize, usize), value: char) {
        let row_len = self.width() + 2;
        let index = pos.1 * row_len + pos.0;

        let mut chars: Vec<char> = self.string.chars().collect();
        chars[index] = value;

        self.string = chars.iter().collect();
    }

    pub fn width(&self) -> usize {
        self._width
    }

    pub fn height(&self) -> usize {
        self._height
    }

    pub fn chars(&self) -> Vec<char> {
        let mut v = vec![];

        for i in 0..self.height() {
            for j in 0..self.width() {
                v.push(self.get((j, i)))
            }
        }

        v
    }

    pub fn render(self, pos: (isize, isize), source: &Canvas) -> Canvas {
        let mut target = self;

        for i in 0..source.height() {
            for j in 0..source.width() {
                let x = pos.0 + j as isize;
                let y = pos.1 + i as isize;

                if x < target.width() as isize
                && y < target.height() as isize
                && x >= 0 && y >= 0 {
                    target.set((x as usize, y as usize), source.get((j, i)))
                }
            }
        }

        target
    }
}

impl From::<&str> for Canvas {
    fn from(value: &str) -> Self {
        let line_count = value.lines().count();
        let mut max_char_count = 0;
        value.lines().for_each(|line| {
            max_char_count = max_char_count.max(line.chars().count());
        });

        let mut canvas = Canvas::new(max_char_count, line_count, ' ');
        value.lines().enumerate().for_each(|(i, s)| {
            s.chars().enumerate().for_each(|(j, c)| {
                canvas.set((j, i), c);
            });
        });

        canvas
    }
}

impl From::<String> for Canvas {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
