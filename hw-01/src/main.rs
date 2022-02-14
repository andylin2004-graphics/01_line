use draw_line::Image;
use draw_line::Color;
mod draw_line;

fn main() {
    const XRES: i32 = 500;
    const YRES: i32 = 500;
    let mut s = Image::new(XRES as usize, YRES as usize);
    let mut c = Color::new_color(0, 255, 0);

    // octants 1 and 5
    s.draw_line(0, 0, XRES-1, YRES-1, c);
    s.draw_line(0, 0, XRES-1, YRES / 2, c);
    s.draw_line(XRES -1, YRES -1, 0, YRES / 2, c);

    // octants 8 and 4
    c.b = 255;
    s.draw_line(0, YRES-1, XRES-1, 0, c);  
    s.draw_line(0, YRES-1, XRES-1, YRES/2, c);
    s.draw_line(XRES-1, 0, 0, YRES/2, c);

    // octants 2 and 6
    c.r = 255;
    c.g = 0;
    c.b = 0;
    s.draw_line(0, 0, XRES/2, YRES-1, c);
    s.draw_line(XRES-1, YRES-1, XRES/2, 0, c);

    // octants 7 and 3
    c.b = 255;
    s.draw_line(0, YRES-1, XRES/2, 0, c);
    s.draw_line(XRES-1, 0, XRES/2, YRES-1, c);

    // horizontal and vertical
    c.b = 0;
    c.g = 255;
    s.draw_line(0, YRES/2, XRES-1, YRES/2, c);
    s.draw_line(XRES/2, 0, XRES/2, YRES-1, c);

    s.create_file("imageFile.ppm".to_owned());

    s = Image::new(XRES as usize, YRES as usize);

    s.draw_line(XRES/2, 0, XRES - 1, YRES/2, c);
    s.draw_line(XRES - 1, YRES/2, XRES/2, YRES - 1, c);
    s.draw_line(XRES/2, YRES - 1, 0, YRES/2, c);
    s.draw_line(0, YRES/2, XRES/2, 0, c);

    s.create_file("rickRoll.ppm".to_owned());
}
