#![allow(unused_imports)] // Remove later

use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowBuildError, WindowContext};

use sdl2::{EventPump, VideoSubsystem};
use shapes::{Triangle, Camera, Vec3, Shape};
mod shapes;

fn main() -> Result<(), String>{
    // Renderer stuff

    let width: u32 = 64;
    let height: u32 = 64;
    let pixel_size: u32 = 8;
    let fov: f32 = 90.0;

    let cam: Camera = Camera::new(Vec3 {x:0.0, y:0.0, z:0.0}, fov, width, height, pixel_size);

    let mut objects: Vec<Shape> = Vec::new();
    objects.push(Shape::Triangle(Triangle::new(
        Vec3{x:20.0,y:0.0,z:20.0}, 
        Vec3{x:20.0,y:20.0,z:20.0},
        Vec3{x:0.0,y:20.0,z:20.0}
    )));


    // Build screen
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_subsystem
        .window("rust-sdl2 demo: Video", width*pixel_size, height*pixel_size)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e: WindowBuildError| e.to_string())?;
    let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump: EventPump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255,255,255));
        // let _ = canvas.fill_rect(Rect::new(10, 10, 10, 10));

        for x in 0..width {
            for y in 0..height {
                let p = cam.get_pixel(x, y, &objects);
                if p.value {
                    canvas.fill_rect(Rect::new(p.position.x as i32 * pixel_size as i32, p.position.y as i32 * pixel_size as i32, pixel_size, pixel_size));
                }
            }
        }


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    Ok(())

}
