#![feature(test)]

extern crate test;
use test::Bencher;
use tui_renderer::terminal::canvas::Canvas;

#[bench]
fn bench_canvas_new(b: &mut Bencher) {
    b.iter(|| {
        let _ = Canvas::new(50, 10, '.');
    });
}
