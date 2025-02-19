#![feature(test)]

extern crate test;
use test::Bencher;
use tui_renderer::terminal::canvas::Canvas;

#[bench]
fn bench_canvas_new(b: &mut Bencher) {
    b.iter(|| {
        let _ = Canvas::new((50, 10), '.');
    });
}

#[bench]
fn bench_canvas_draw(b: &mut Bencher) {
    let target = Canvas::new((50, 10), '.');
    let source = Canvas::new((5, 5), 'x');
    b.iter(|| {
        let t = target.clone();
        let _ = t.draw((0, 0), &source);
    });
}
