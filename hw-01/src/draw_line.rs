#[derive(Copy, Clone)]
struct ColorInfo{
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl ColorInfo{
    fn new() -> ColorInfo{
        ColorInfo{r: 0, g:0, b:0}
    }

    fn to_string(&self) -> String{
        return format!("{} {} {}", self.r, self.g, self.b);
    }
    
    fn plotColor(&self, newColor: ColorInfo){
        self.r = newColor.r;
        self.g = newColor.g;
        self.b = newColor.b;
    }
}

struct Screen{
    pub screen: Vec<Vec<ColorInfo>>,
}

impl Screen{
    fn new(x: usize, y: usize) -> Screen{
        Screen{screen: vec![vec![ColorInfo::new(); x]; y]}
    }

    fn plot(&self, x: i32, y: i32, color: ColorInfo){
        if x >= 0 && y >= 0{
            self.screen[x as usize][y as usize].plotColor(color);
        }
    }
}
fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, screen: Screen, color: ColorInfo){
    // if x0 < x1{
    //     let tmp = x0;
    //     x0 = x1;
    //     x1 = tmp;
    //     tmp = y0;
    //     y0 = y1;
    //     y1 = tmp;
    // }
    let x = x0;
    let y = y0;
    let a = 2*(y1-y0);
    let b = -2*(x1-x0);
    let d = a + 1/2*b;
    while x <= x1{
        screen.plot(x, y, color);
        if d < 0{
            y += 1;
            d += b;
        }
        x += 1;
        d += a;
    }
}