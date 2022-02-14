use draw_line::Image;
use draw_line::Color;
mod draw_line;

impl Image{
    fn square_portal(&mut self, mut XRES: i32, mut YRES: i32, mut shift_by: i32, counter: u32, mut c: Color){
        if counter % 2 == 0 {
            self.draw_line(XRES/2 + shift_by, shift_by, XRES - 1 + shift_by, YRES/2 + shift_by, c);
            self.draw_line(XRES - 1 + shift_by, YRES/2 + shift_by, XRES/2 + shift_by, YRES - 1 + shift_by, c);
            self.draw_line(XRES/2 + shift_by, YRES - 1 + shift_by, shift_by, YRES/2 + shift_by, c);
            self.draw_line(shift_by, YRES/2 + shift_by, XRES/2 + shift_by, shift_by, c);
            XRES /= 2;
            YRES /= 2;
            shift_by += XRES / 2;
        }else{
            self.draw_line(shift_by, shift_by, XRES + shift_by, shift_by, c);
            self.draw_line(XRES + shift_by, shift_by, XRES + shift_by, YRES + shift_by, c);
            self.draw_line(XRES + shift_by, YRES + shift_by, shift_by, YRES + shift_by, c);
            self.draw_line(shift_by, YRES + shift_by, shift_by, shift_by, c);
        }
        c.g += 20;
        if XRES >= 1 && YRES >= 1 && shift_by >= 1{
            self.square_portal(XRES, YRES, shift_by, counter + 1, c);
        }
    }
}

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

    c.r = 0;
    c.g = 255/2;
    c.b = 255/2;

    s.create_file("imageFile.ppm".to_owned());

    s = Image::new(XRES as usize, YRES as usize);

    s.square_portal(XRES, YRES, 0, 0, c);
    s.create_file("arts.ppm".to_owned());
}