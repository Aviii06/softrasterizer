use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::slice;
use std::str;

pub struct WindowManager {
    pub screen_width: u32,
    pub screen_height: u32,
    // pub title: Box<str>,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    running: bool,
    pub event_queue: sdl2::EventPump,
}

impl WindowManager {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("hello", screen_width, screen_height)
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        let running = true;
        let mut event_queue = sdl_context.event_pump().unwrap();


        Self {
            screen_width,
            screen_height,
            // title,
            canvas,
            running,
            event_queue,
        }
    }

    pub fn draw(&mut self, mut fb: &crate::frameBuffer::FrameBuffer) {
        for event in self.event_queue.poll_iter() {

            match event {
                Event::Quit {..} => {
                    self.running = false;
                },

                Event::MouseMotion {
                    x, y, xrel, yrel, .. } => {

                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                },

                _ => {}
            }
        }

        for y in 0..fb.height {
            for x in 0..fb.width {
                let px = fb.getPixel(x, y);
                self.canvas.set_draw_color(Color::RGBA(px.r.try_into().unwrap(), px.g.try_into().unwrap(), px.b.try_into().unwrap(), px.a.try_into().unwrap()));
                self.canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
            }
        }

        self.canvas.present();
    }

    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn isRunning(&self) -> bool {
        self.running
    }
}

pub fn setup() -> Result<(), String> {

    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let clear_color = Color::RGB(64, 192, 255);
    canvas.set_draw_color(clear_color);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {

        for event in event_queue.poll_iter() {

            match event {
                Event::Quit {..} => {
                    running = false;
                },

                Event::MouseMotion {
                    x, y, xrel, yrel, .. } => {

                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                },

                _ => {}
            }
        }

        canvas.fill_rect(screen_area);
        canvas.present();
    }

    Ok(())
}