use std::fmt::Display;

use super::render::surface::RenderSurface;
use super::render::element::RenderElement;

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

    // pub fn chars(&self) -> Vec<char> {
    //     let mut v = vec![];

    //     for i in 0..self.height() {
    //         for j in 0..self.width() {
    //             v.push(self.get(j, i))
    //         }
    //     }

    //     v
    // }
}

impl RenderElement for Canvas {
    fn get(&self, pos: (usize, usize)) -> char {
        let row_len = self.width() + 2;
        let index = pos.1 * row_len + pos.0;

        self.string.char_indices().nth(index).unwrap().1
    }

    fn render<T>(&self, pos: (usize, usize), surface: &mut T)
    where T: RenderSurface {
        for i in 0..self.height() {
            for j in 0..self.width() {
                surface.set((j + pos.0, i + pos.1), self.get((j, i)))
            }
        }
    }
}

impl RenderSurface for Canvas {
    fn set(&mut self, pos: (usize, usize), value: char) {
        let row_len = self.width() + 2;
        let index = pos.1 * row_len + pos.0;

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
