use tui_renderer::terminal::canvas::Canvas;

#[test]
fn test_canvas_get() {
    let canvas = Canvas::new(50, 10, '.');

    for i in 0..10 {
        for j in 0..50 {
            assert_eq!('.', canvas.get((j, i)))
        }
    }
}

#[test]
fn test_canvas_set() {
    let mut canvas = Canvas::new(50, 10, '.');

    for i in 0..10 {
        for j in 0..50 {
            let pos = (j, i);
            assert_eq!('.', canvas.get(pos));
            canvas.set(pos, '$');
            assert_eq!('$', canvas.get(pos));
        }
    }
}
