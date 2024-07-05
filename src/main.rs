#![allow(unused_imports)] // Remove later
#![allow(non_upper_case_globals)]

use std::{ops::Not, time::{Duration, Instant}};

use minifb::{Key, Window, WindowOptions, MouseMode};
use noise::Turbulence;
use rayon::prelude::*;

use shapes::{Camera, Cube, Lexip, Lights, Shape, Sun, Vec2, Vec3};
mod shapes;


const size: f32 = 5.0;
const WIDTH: usize = (size*16.0) as usize;
const HEIGHT: usize = (size*9.0) as usize;
const pixel_size: usize = 6;

const MOVE_SPEED: f32 = 300.0;
const TURN_SPEED: f32 = 5.0;

fn main() {
    println!("Initialized");
    let fov: f32 = 70.0;

    let mut cam: Camera = Camera::new(Vec3 {x:0.0, y:0.0, z:-200.0}, Vec3 {x:0.0, y: 0.0, z: 0.0}, fov, WIDTH, HEIGHT, pixel_size);

    let mut objects: Vec<Shape> = Vec::new();
    // objects.push(Shape::Cube(Cube::new(Vec3{x:120.0,y: 50.0,z:100.0}, 80.0)));
    objects.push(Shape::Cube(Cube::new(Vec3{x:120.0,y: 50.0,z:0.0}, 120.0)));
    // objects.push(Shape::Cube(Cube::new(Vec3{x:0.0,y: 100.0,z:0.0}, 10.0)));
    // objects.push(Shape::Cube(Cube::new(Vec3{x:0.0,y: 50.0,z:100.0}, 50.0)));

    

    let mut lights: Vec<Lights> = Vec::new();
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 }, colour: Vec3 {x: 140.0, y: 140.0, z: 140.0} }));
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, colour: Vec3 {x: 140.0, y: 0.0, z: 140.0} }));
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 }, colour: Vec3 {x: 230.0, y: 140.0, z: 140.0} }));
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: -1.0, y: 0.0, z: 0.0 }, colour: Vec3 {x: 140.0, y: 140.0, z: 0.0} }));
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: 0.0, y: -1.0, z: 0.0 }, colour: Vec3 {x: 0.0, y: 140.0, z: 140.0} }));
    lights.push(Lights::Sun(Sun { direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, colour: Vec3 {x: 140.0, y: 255.0, z: 140.0} }));
    
    const SCALE: minifb::Scale = minifb::Scale::X1;
    
    let mut window = Window::new(
        "renderer",
        WIDTH*pixel_size,
        HEIGHT*pixel_size,
        WindowOptions {
            resize: true,
            scale: SCALE,
            ..WindowOptions::default()
        },
    )
    .unwrap();


    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.set_background_color(0, 0, 0);

        let start = Instant::now();

        let buffer: Vec<u32> = (0..(WIDTH*HEIGHT)).into_iter().map(|i| 
            {
                let point = dimension_expansion(i);
                if (8.0*point.x-point.y + 8.0) < 0.0 || (-8.0*point.x-point.y+WIDTH as f32*8.0) < 0.0{
                    return from_u8_rgb(140, 140, 140);
                }
                else if point.y == (WIDTH as f32*0.75).round() {
                    return from_u8_rgb(20, 20, 20);
                }
                else if point.y > (WIDTH as f32 * 0.75).round() {
                    return from_u8_rgb(50, 50, 60);
                }
                let p: Lexip = cam.get_pixel(point.x as usize, point.y as usize, &objects, &lights);
                from_u8_rgb(p.red, p.green, p.blue)
            }
        ).collect();
        

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        let took = start.elapsed();
        let delta = took.as_secs_f32();

        println!("{} fps", 1.0/delta);
        
        // println!("{}, {}, {}", 
        // (cam.rotation.x.to_degrees() % 360.0).abs(),
        // cam.rotation.y.to_degrees(),
        // cam.rotation.z.to_degrees());

        if window.is_key_down(Key::W) {
            cam.move_forward(Vec3 {x: 0.0, y: 0.0, z: MOVE_SPEED*delta});
        }
        if window.is_key_down(Key::S) {
            cam.move_forward(Vec3 {x: 0.0, y: 0.0, z: -MOVE_SPEED*delta});
        }
        if window.is_key_down(Key::A) {
            // cam.rotate(Vec3 {x: 0.0, y: 0.0, z: -TURN_SPEED*delta});
            cam.move_forward(Vec3 {x: -MOVE_SPEED*delta, y: 0.0, z: 0.0});
        }
        if window.is_key_down(Key::D) {
            // cam.rotate(Vec3 {x: 0.0, y: 0.0, z: TURN_SPEED*delta});
            cam.move_forward(Vec3 {x: MOVE_SPEED*delta, y: 0.0, z: 0.0});
        }
        if window.is_key_down(Key::Up) {
            cam.rotate(Vec3 { x: TURN_SPEED*delta, y: 0.0, z: 0.0 });
        }
        if window.is_key_down(Key::Down) {
            cam.rotate(Vec3 { x: -TURN_SPEED*delta, y: 0.0, z: 0.0 });
        }
        if window.is_key_down(Key::Left) {
            cam.rotate(Vec3 { x: 0.0, y: TURN_SPEED*delta, z: 0.0 });
        }
        if window.is_key_down(Key::Right) {
            cam.rotate(Vec3 { x: 0.0, y: -TURN_SPEED*delta, z: 0.0 });
        }

    }
}


// fn clampi(n: i32, low: i32, high: i32) -> i32 {
//     if n > high {
//         return high;
//     }
//     if n < low {
//         return low;
//     }
//     n
// }

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn dimension_expansion(i: usize) -> Vec2 {
    Vec2 {
        x: (i % WIDTH) as f32,
        y: (i / HEIGHT) as f32 
    }
}
// fn dimension_shrink(i: Vec2) -> usize {
//     (WIDTH as f32 * i.x + i.y) as usize
// }
