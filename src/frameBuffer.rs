pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<crate::pixel::Pixel>,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let size = width * height;
        let px = crate::pixel::Pixel::new();
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

    pub fn clearBuffer(&mut self, px: crate::pixel::Pixel) {
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

    pub fn getPixel(&self, x: i32, y: i32) -> crate::pixel::Pixel {
        self.pixels[(y * self.width + x) as usize].clone()
    }
}