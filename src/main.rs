use tui_renderer::terminal::canvas::Canvas;

fn main() {
    let canvas = Canvas::new(5, 4);

    println!("{}", canvas);
}
