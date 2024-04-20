#![allow(unused_imports)] // Remove later
#![allow(non_upper_case_globals)]

use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

use shapes::{Cube, Camera, Vec3, Shape, Object, Lexip, Lights, Sun};
mod shapes;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const pixel_size: usize = 16;
const DYNAMIC_SIZE: bool = true;
const REFRESH_RATE: u64 = 240;

fn main() {
    let fov: f64 = 90.0;

    let cam: Camera = Camera::new(Vec3 {x:0.0, y:0.0, z:-100.0}, fov, WIDTH, HEIGHT, pixel_size);

    let mut objects: Vec<Shape> = Vec::new();
    objects.push(Shape::Cube(Cube::new(Vec3{x:120.0,y: 50.0,z:100.0}, 80.0)));
    // objects.push(Shape::Cube(Cube::new(Vec3{x:-120.0,y: 50.0,z:-100.0}, 80.0)));
    // objects.push(Shape::Cube(Cube::new(Vec3{x:120.0,y: 50.0,z:-100.0}, 80.0)));
    // objects.push(Shape::Cube(Cube::new(Vec3{x:-120.0,y: 50.0,z:100.0}, 80.0)));
    let mut lights: Vec<Lights> = Vec::new();
    
    // build screen
    const fn scale_to_int(scale: minifb::Scale) -> u8 {
        use minifb::Scale::*;
        match scale {
            X1 => 1,
            X2 => 2,
            X4 => 4,
            X8 => 8,
            X16 => 16,
            X32 => 32,
            FitScreen => 1,
        }
    }
    const SCALE: minifb::Scale = minifb::Scale::X1;
    
    let mut window = Window::new(
        "renderer",
        WIDTH*pixel_size,
        HEIGHT*pixel_size,
        WindowOptions {
            // borderless: true,
            title: true,
            resize: DYNAMIC_SIZE,
            scale: SCALE,
            // transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_nanos(1_000_000_000 / REFRESH_RATE)));
    // window.limit_update_rate(60);

    // objects[0].rotateZ(0.05, Vec3{x:0.0,y:0.0,z:0.0});
    // objects[0].rotateY(-0.01, Vec3{x:20.0,y:0.0,z:150.0});
    // objects[0].rotateX(0.1, Vec3{x:20.0,y:0.0,z:150.0});
    // objects[1].rotateZ(-0.05, Vec3{x:0.0,y:0.0,z:0.0});
    // objects[1].rotateY(0.01, Vec3{x:20.0,y:0.0,z:150.0});
    // objects[1].rotateX(-0.1, Vec3{x:20.0,y:0.0,z:150.0});


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = vec![0u32; WIDTH * HEIGHT];
        // println!("{}", buffer.len());

        let mut collision_map: Vec<Lexip> = Vec::new();
        window.set_background_color(0, 0, 0);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let p: Lexip = cam.get_pixel(x, y, &objects, &lights);
                // canvas.set_draw_color(Color::RGB(
                //     p.red, 
                //     p.green,
                //     p.blue
                // ));
                // let _ = canvas.fill_rect(Rect::new(x as i32 * pixel_size as i32, y as i32 * pixel_size as i32, pixel_size, pixel_size));
                collision_map.push(p);
                if y+x*WIDTH < buffer.len() && y+x*WIDTH > 0 {
                    buffer[y+x*WIDTH] = clampi((collision_map[y+x*WIDTH].blue as i32)/2, 0, 255) as u32;
                }
            }
        }
        for i in 0..collision_map.clone().into_iter().len() {// if collision_map[i].collision_object != Shape::None {
            let x = (i / WIDTH as usize) as i32;
            let y = (i % HEIGHT as usize) as i32;
            for n in [(0,-1),(1,0)] {
                if ((y+n.0) + (x+n.1) * WIDTH as i32) < (WIDTH*HEIGHT) as i32 && ((y+n.0) + (x+n.1) * WIDTH as i32) > 0 &&
                        collision_map[((y+n.0) + (x+n.1) * WIDTH as i32) as usize].collision_object != collision_map[i].collision_object 
                {
                    buffer[i] = 0x255;
                    // canvas.set_draw_color(Color::RGB(
                    //     clampi((collision_map[i].red as i32)/2, 0, 255) as u8, 
                    //     clampi((collision_map[i].green as i32)/2, 0, 255) as u8, 
                    //     clampi((collision_map[i].blue as i32)/2, 0, 255) as u8, 
                    // ));
                    // let _ = canvas.fill_rect(Rect::new(x * pixel_size as i32, y * pixel_size as i32, pixel_size, pixel_size));
                    // window
                    // .update_with_buffer(buf.as_rgba(), buf.width(), buf.height())
                    // .unwrap();
                    
                }
            }
        }//}
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // objects[0].rotate_xy(0.05, Vec3{x:0.0,y:0.0,z:0.0});
        objects[0].rotateZ(-0.007, Vec3{x:20.0,y:0.0,z:200.0});
        // objects[0].translate(Vec3{x:0.0,y:0.0,z:-1.0});
        // lights[0].direction.rotateX(-0.007, Vec3{x:0.0,y:0.0,z:0.0});
        // objects[1].rotateZ(0.005, Vec3{x:20.0,y:0.0,z:200.0});
        // objects[0].rotate_yz(0.001, Vec3{x:20.0,y:0.0,z:150.0});

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    // Ok(())
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