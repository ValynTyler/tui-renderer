use std::fmt::Display;

pub trait Position<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl Position<usize> for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

impl Position<isize> for (isize, isize) {
    fn x(&self) -> isize {
        self.0
    }

    fn y(&self) -> isize {
        self.1
    }
}

pub trait Size<T> {
    fn width(&self) -> T;
    fn height(&self) -> T;
}

impl Size<usize> for (usize, usize) {
    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1
    }
}

impl Size<isize> for (isize, isize) {
    fn width(&self) -> isize {
        self.0
    }

    fn height(&self) -> isize {
        self.1
    }
}

#[derive(Clone)]
pub struct Canvas(Box<[Box<[char]>]>);

impl Canvas {
    pub fn new(size: impl Size<usize>, fill: char) -> Self {
        Canvas(vec![vec![fill; size.width()].into(); size.height()].into())
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn center(&self) -> (usize, usize) {
        (self.width() / 2, self.height() / 2)
    }

    pub fn row(&self, index: usize) -> &[char] {
        &self.0[index]
    }

    pub fn get(&self, pos: impl Position<usize>) -> char {
        self.0[pos.y()][pos.x()]
    }

    pub fn set(&mut self, pos: impl Position<usize>, value: char) {
        self.0[pos.y()][pos.x()] = value
    }

    pub fn draw(self, pos: impl Position<isize>, source: &Canvas) -> Canvas {
        let mut target = self;

        for i in 0..source.height() {
            for j in 0..source.width() {
                let x = pos.x() + j as isize;
                let y = pos.y() + i as isize;

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

        let mut canvas = Canvas::new((max_char_count, line_count), ' ');
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
        for i in 0..self.height() - 1 {
            for j in 0..self.width() {
                write!(f, "{}", self.get((j, i)))?;
            }
            write!(f, "\r\n")?;
        }

        for c in self.row(self.height() - 1) {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}
