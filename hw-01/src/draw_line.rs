use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Color{
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Color{
    pub fn new() -> Color{
        Color{r: 0, g:0, b:0}
    }

    pub fn new_color(r: i32, g: i32, b: i32) -> Color{
        Color{r: r, g: g, b: b}
    }

    pub fn to_string(&self) -> String{
        return format!("{} {} {}", self.r, self.g, self.b);
    }
    
    pub fn plot_color(&mut self, new_color: Color){
        self.r = new_color.r;
        self.g = new_color.g;
        self.b = new_color.b;
    }
}

pub struct Image{
    pub screen: Vec<Vec<Color>>,
    image_height: usize,
    image_width: usize,
}

impl Image{
    pub fn new(image_width: usize, image_height: usize) -> Image{
        Image{screen: vec![vec![Color::new(); image_width]; image_height], image_width: image_width, image_height: image_height}
    }

    pub fn plot(&mut self, x: i32, y: i32, color: Color){
        if x >= 0 && y >= 0{
            self.screen[(self.image_height - 1) - y as usize][x as usize].plot_color(color);
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color){
        if x0 > x1{
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        let slope: f32 = (y1-y0) as f32 / (x1-x0) as f32;
        if slope > 1.0{
            // octant 2
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling y
            while y <= y1{
                self.plot(x, y, color);
                if d < 0{ // as b dominates a, and we need to hit 0
                    x += 1;
                    d += a;
                }
                y += 1;
                d += b;
            }
        }else if slope >= 0.0{
            // octant 1
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling x
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as a dominates b, and we need to hit 0
                    y += 1;
                    d += b;
                }
                x += 1;
                d += a;
            }
        }else if slope < -1.0{
            // octant 7
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling x
            while y >= y1{
                self.plot(x, y, color);
                if d < 0{ // as a dominates b, and we need to hit 0
                    x += 1;
                    d -= a; // basically adding
                }
                y -= 1;
                d -= b; // basically adding
            }
        }else{
            // octant 8
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling y
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as b dominates a, and we need to hit 0
                    y -= 1;
                    d -= b; // basically adding
                }
                x += 1;
                d -= a; // basically adding
            }
        }
    }

    fn create_data(&self) -> String{
        let mut result: String = format!("P3\n{} {}\n255\n", self.screen.len(), self.screen[0].len());
     
        for i in 0..self.screen.len(){
            for v in 0..self.screen[i].len(){
                result.push_str(&self.screen[i][v].to_string().to_owned());
                result.push_str("  ");
            }
            result.push_str("\n");
        }
    
        return result;
    }

    pub fn create_file(&self, file_name: String){
        let path = Path::new(&file_name);

        let mut file = match File::create(&path){
            Err(error) => panic!("failed to create image file because {}", error),
            Ok(file) => file,
        };

        let result = self.create_data();

        match file.write_all(result.as_bytes()){
            Err(error) => panic!("failed to write image file because {}", error),
            Ok(_) => {},
        };
    }
}