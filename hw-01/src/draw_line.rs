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

    pub fn newColor(r: i32, g: i32, b: i32) -> Color{
        Color{r: r, g:g, b:b}
    }

    pub fn to_string(&self) -> String{
        return format!("{} {} {}", self.r, self.g, self.b);
    }
    
    pub fn plotColor(&mut self, newColor: Color){
        self.r = newColor.r;
        self.g = newColor.g;
        self.b = newColor.b;
    }
}

pub struct Screen{
    pub screen: Vec<Vec<Color>>,
}

impl Screen{
    pub fn new(x: usize, y: usize) -> Screen{
        Screen{screen: vec![vec![Color::new(); x]; y]}
    }

    pub fn plot(&mut self, x: i32, y: i32, color: Color){
        if x >= 0 && y >= 0{
            self.screen[x as usize][y as usize].plotColor(color);
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color){
        if x0 < x1{
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        if y0 < y1{
            // octant 2
            if (y1-y0)/(x1-x0) > 1{
                let mut x = x0;
                let mut y = y0;
                let a = 2*(y1-y0);
                let b = -2*(x1-x0);
                let mut d = a + 1/2*b;
                while x <= x1{
                    self.plot(x, y, color);
                    if d < 0{
                        y += 1;
                        d += b;
                    }
                    x += 1;
                    d += a;
                }
            }else{
                // octant 1
                let mut x = x0;
                let mut y = y0;
                let a = 2*(y1-y0);
                let b = -2*(x1-x0);
                let mut d = 1/2*a + b;
                while y <= y1{
                    self.plot(x, y, color);
                    if d < 0{
                        x += 1;
                        d += a;
                    }
                    y += 1;
                    d += b;
                }
            }
        }else{
            //octant 7
            if (y1-y0)/(x1-x0) > -1{
                let mut x = x0;
                let mut y = y0;
                let a = 2*(y1-y0);
                let b = -2*(x1-x0);
                let mut d = a + 1/2*b;
                while x <= x1{
                    self.plot(x, y, color);
                    if d < 0{
                        y -= 1;
                        d += b;
                    }
                    x += 1;
                    d += a;
                }
            }else{
                // octant 8
                let mut x = x0;
                let mut y = y0;
                let a = 2*(y1-y0);
                let b = -2*(x1-x0);
                let mut d = 1/2*a + b;
                while y <= y1{
                    self.plot(x, y, color);
                    if d < 0{
                        x -= 1;
                        d += a;
                    }
                    y += 1;
                    d += b;
                }
            }
        }
    }

    fn create_data(&self) -> String{
        let mut result: String = "P3\n500 500\n255\n".to_owned();
     
        for i in 0..self.screen.len(){
            for v in 0..self.screen[i].len(){
                result.push_str(&self.screen[i][v].to_string().to_owned());
                result.push_str("  ");
            }
            result.push_str("\n");
        }
    
        return result;
    }

    pub fn createFile(&self){
        let path = Path::new("imageFile.ppm");

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