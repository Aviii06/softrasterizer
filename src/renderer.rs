use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw(&mut self,fb :crate::frameBuffer::FrameBuffer) {
        for y in 0..fb.height {
            for x in 0..fb.width {
                let px = fb.getPixel(x, y);
                self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(px.r.try_into().unwrap(), px.g.try_into().unwrap(), px.b.try_into().unwrap(), px.a.try_into().unwrap()));
                self.canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
            }
        }

        self.present();
    }
}