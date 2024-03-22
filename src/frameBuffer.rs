pub use crate::pixel::Pixel;

pub struct FrameBuffer {
    width: i32,
    height: i32,
    pixels: Vec<Pixel>,
}

impl FrameBuffer {
    pub fn  new(width: i32, height: i32) -> Self {
        let size = width * height;
        let px = Pixel::new();
        let pxs = vec![px; size as usize];
        // for i in 0..size {
        //     pxs.push(px.clone())
        // }
        Self {
            width,
            height,
            pixels: pxs.clone(),
        }
    }

    pub fn clearBuffer(&mut self, px: Pixel) {
        self.pixels.iter_mut().for_each(|p| {
            p.r = px.r;
            p.g = px.g;
            p.b = px.b;
            p.a = px.a;
        });
    }

    pub fn print(&self) {
        self.pixels.iter().for_each(|p| p.print())
    }
}