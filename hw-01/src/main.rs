use draw_line::Screen;
use draw_line::Color;
mod draw_line;

fn main() {
    let XRES = 500;
    let YRES = 500;
    let MAX_COLOR = 255;
    let mut s = Screen::new(XRES, YRES);
    let c = Color::new_color(0, 255, 0);

    s.draw_line(0, 0, XRES as i32-1, YRES as i32-1, c);
    s.draw_line(0, 0, XRES as i32-1, YRES as i32 / 2, c);
    s.draw_line(XRES as i32-1, YRES as i32-1, 0, YRES as i32 / 2, c);

    s.create_file();
}
