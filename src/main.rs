#![allow(unused_imports)] // Remove later

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use shapes::{Ray, Triangle, Vec3};
mod shapes;

fn main() -> Result<(), String>{
    // let face: Triangle = Triangle::new(
    //     Vec3{x:-10.0,y:-10.0,z:1.0},
    //     Vec3{x:-10.0,y:10.0,z:1.0},
    //     Vec3{x:10.0,y:10.0,z:1.0}
    // );
    // let ray: Ray = Ray { point: Vec3{x:0.0,y:0.0,z:0.0}, vector: Vec3{x:0.0,y:0.0,z:1.0}};
    // println!("{}", face.intersects(ray));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 64, 64)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

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
        let _ = canvas.fill_rect(Rect::new(10, 10, 10, 10));

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    Ok(())

}
