extern crate piston_window;
extern crate sdl2_window;

fn main() {
    use piston_window::*;
    use sdl2_window::Sdl2Window;

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("SDL Window", (640, 480))
        .fullscreen(false)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear([0.9, 0.1, 0.2, 1.0], g);
        });
    }
}
