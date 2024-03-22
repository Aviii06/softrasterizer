#[derive(Clone)]
pub struct Pixel {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub fn print(&self) {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;

        println!("{r}, {g}, {b}, {a}")
    }
}
