#![allow(unused_imports)] // Remove later
#![allow(non_upper_case_globals)]

use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowBuildError, WindowContext};

use sdl2::{EventPump, VideoSubsystem};
use shapes::{Cube, Camera, Vec3, Shape, Object, Lexip};
mod shapes;

const width: u32 = 64;
const height: u32 = 64;
const pixel_size: u32 = 8;

fn main() -> Result<(), String>{
    // Renderer stuff

    // let width: u32 = 108;
    // let height: u32 = 64;
    // let pixel_size: u32 = 8;
    let fov: f64 = 90.0;

    let cam: Camera = Camera::new(Vec3 {x:0.0, y:0.0, z:-100.0}, fov, width, height, pixel_size);

    let mut objects: Vec<Shape> = Vec::new();
    objects.push(Shape::Cube(Cube::new(Vec3{x:120.0,y: 50.0,z:200.0}, 80.0)));
    objects.push(Shape::Cube(Cube::new(Vec3{x:-120.0,y:50.0,z:200.0}, 80.0)));
    // objects.push(Shape::Object(Object::new("test.obj", Vec3 {x:0.0,y:0.0,z:0.0},Vec3 {x:0.0,y:0.0,z:200.0}, 50.0)));
    // objects.push(Shape::Triangle(Triangle::new(
    //     Vec3{x:20.0,y:0.0,z:20.0}, 
    //     Vec3{x:20.0,y:20.0,z:20.0},
    //     Vec3{x:0.0,y:20.0,z:20.0}
    // )));
    
    
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

    objects[0].rotateZ(0.05, Vec3{x:0.0,y:0.0,z:0.0});
    objects[0].rotateY(-0.01, Vec3{x:20.0,y:0.0,z:150.0});
    objects[0].rotateX(0.1, Vec3{x:20.0,y:0.0,z:150.0});
    objects[1].rotateZ(-0.05, Vec3{x:0.0,y:0.0,z:0.0});
    objects[1].rotateY(0.01, Vec3{x:20.0,y:0.0,z:150.0});
    objects[1].rotateX(-0.1, Vec3{x:20.0,y:0.0,z:150.0});

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

        let mut collision_map: Vec<Lexip> = Vec::new();

        for x in 0..width {
            for y in 0..height {
                let p: Lexip = cam.get_pixel(x, y, &objects);
                canvas.set_draw_color(Color::RGB(
                    p.red, 
                    p.green,
                    p.blue
                ));
                let _ = canvas.fill_rect(Rect::new(x as i32 * pixel_size as i32, y as i32 * pixel_size as i32, pixel_size, pixel_size));
                collision_map.push(p);
            }
        }
        for i in 0..collision_map.clone().into_iter().len() {// if collision_map[i].collision_object != Shape::None {
            let x = (i / width as usize) as i32;
            let y = (i % height as usize) as i32;
            for n in [(0,-1),(1,0)] {
                if ((y+n.0) + (x+n.1) * width as i32) < (width*height) as i32 && ((y+n.0) + (x+n.1) * width as i32) > 0 {
                if collision_map[((y+n.0) + (x+n.1) * width as i32) as usize].collision_object != collision_map[i].collision_object {
                    canvas.set_draw_color(Color::RGB(
                        clampi(((collision_map[i].red as i32+255 as i32)/2 as i32) as i32, 0, 255) as u8, 
                        clampi(((collision_map[i].red as i32+255 as i32)/2 as i32) as i32, 0, 255) as u8, 
                        clampi(((collision_map[i].red as i32+255 as i32)/2 as i32) as i32, 0, 255) as u8, 
                    ));
                    let _ = canvas.fill_rect(Rect::new(x as i32 * pixel_size as i32, y as i32 * pixel_size as i32, pixel_size, pixel_size));
                }}

            }
        }//}

        // objects[0].rotate_xy(0.05, Vec3{x:0.0,y:0.0,z:0.0});
        objects[0].rotateZ(-0.007, Vec3{x:20.0,y:0.0,z:200.0});
        objects[1].rotateZ(0.005, Vec3{x:20.0,y:0.0,z:200.0});
        // objects[0].rotate_yz(0.001, Vec3{x:20.0,y:0.0,z:150.0});

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    Ok(())
}


fn clampi(n: i32, low: i32, high: i32) -> i32 {
    if n > high {
        return high;
    }
    if n < low {
        return low;
    }
    n
}