#![allow(dead_code)]
#![allow(non_snake_case)]

use std::ffi::FromVecWithNulError;
use std::io::Read;
use std::ops::{Add, Index, Sub};
use std::fs::File;
use std::io::BufReader;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub enum Color {
    White,
    Grey,
    Black
}
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Shape {
    Triangle(Triangle),
    Cube(Cube),
    None
}

impl Shape {
    pub fn intersects(&self, ray:&Ray) -> Lexip {
        match self {
            Shape::Triangle(x) => {return x.intersects(ray)},
            Shape::Cube(x) => {return x.intersects(ray)},
            #[allow(unreachable_patterns)]
            _ => println!("A shape is not yet implemented")
        }
        Lexip::empty()
    }
    pub fn rotateZ(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotateZ(angle, origin),
            Shape::Cube(x) => x.rotateZ(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotateY(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotateY(angle, origin),
            Shape::Cube(x) => x.rotateY(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotateX(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotateX(angle, origin),
            Shape::Cube(x) => x.rotateX(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn translate(&mut self, translation: Vec3) {
        match self {
            Shape::Triangle(x) => x.translate(translation),
            Shape::Cube(x) => x.translate(translation),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn get_position(&mut self) -> Vec3 {
        match self {
            Shape::Triangle(x) => x.A.clone(),
            Shape::Cube(x) => x.position.clone(),
            #[allow(unreachable_patterns)]
            _ => {println!("A shapes rotation is not yet implemented"); Vec3{x: -1.0, y: -1.0, z:-1.0}}
        }
    }
}

pub enum Lights {
    Sun(Sun)
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

pub struct Pixel {
    pub position: Vec2,
    pub value: f32
}

pub struct Ray {
    pub point: Vec3,
    pub vector: Vec3
}

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    width: usize,
    height: usize,
    FOV: f32,
    pixel_size: usize
}

pub struct Surface {
    pub top_left: Vec3,
    pub top_right: Vec3,
    pub bottom_left: Vec3,
    pub bottom_right: Vec3
}
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Triangle {
    pub A: Vec3, // Point A
    pub B: Vec3, // Point B
    pub C: Vec3, // Point C
}
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Cube {
    position:Vec3,
    rotation:Vec3,
    size:f32
}

impl Ray {
    pub fn get_point(&self, t:f32) -> Vec3 {
        Vec3 {x: self.point.x + (self.vector.x*t), y: self.point.y + (self.vector.y*t), z: self.point.z + (self.vector.z*t)}
    }
}

impl Vec3 {
    pub fn new(x:f32, y:f32,z:f32) -> Self {
        Self { x, y, z }
    }
    pub fn as_Vec2 (&self) -> Vec2 {
        Vec2{
            x: self.x,
            y: self.y
        }
    }
    pub fn as_string(&self) -> String {
        format!("({}, {}, {})",self.x,self.y,self.z).to_string()
    }
    pub fn rotateZ(&mut self, angle:f32, origin: Vec3) {        
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.y-origin.y)*-angle.sin())+origin.x,
            y: ((self.x-origin.x)*angle.sin()+(self.y-origin.y)*angle.cos())+origin.y,
            ..self.clone()
        };
    }
    pub fn rotateY(&mut self, angle:f32, origin: Vec3) {
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.x,
            z: ((self.x-origin.x)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z,
            ..self.clone()
        }
    }
    pub fn rotateX(&mut self, angle:f32, origin: Vec3) {
        *self = Self {
            y: ((self.y-origin.y)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.y,
            z: ((self.y-origin.y)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z,
            ..self.clone()
        }
    }
    pub fn len(&self) -> f32 {
        (self.x*self.x+self.y*self.y+self.z*self.z).powf(0.5) // Return the length of vector
    }
    pub fn normalize(&self) -> Vec3{
        let length:f32 = self.len();
        Vec3{
            x: self.x/length,
            y: self.y/length,
            z: self.z/length
        }
    }
    pub fn dot(A:Vec3, B:Vec3) -> f32 {
        A.x*B.x+A.y*B.y+A.z*B.z
    }
    pub fn mul(A:Vec3, B:Vec3) -> Vec3 {
        Vec3 {
            x: A.x * B.x,
            y: A.y * B.y,
            z: A.z * B.z
        }
    }
    pub fn mul_f(A:Vec3, B:f32) -> Vec3 {
        Vec3 {
            x: A.x * B,
            y: A.y * B,
            z: A.z * B
        }
    }
    pub fn x_to_midpoint(A: Vec3, B: Vec3, C:Vec3) -> Vec3 {
        let mid = Vec3::midpoint(B, C);
        mid - A
    }
    pub fn midpoint(A: Vec3, B: Vec3) -> Vec3{
        Vec3{x: (A.x+B.x)/2.0, y: (A.y+B.y)/2.0, z: (A.z+B.z)/2.0}
    }
    pub fn proj(A: Vec3, B:Vec3) -> Vec3 {
        Vec3::mul_f(A, Vec3::dot(A, B)/Vec3::dot(A, A))
    }
    pub fn bary(AB: Vec3, CB:Vec3, AI:Vec3) -> f32 {
        let AV: Vec3 = AB - Vec3::proj(CB, AB);

        1.0 - Vec3::dot(AV, AI)/Vec3::dot(AV, AB)
    }
    pub fn rotate(&mut self, rotation: Vec3, origin: Vec3) {
        self.rotateX(rotation.x, origin);
        self.rotateY(rotation.y, origin);
        self.rotateZ(rotation.z, origin);
    }
    pub fn distance(&self, B: Vec3) -> f32 {
        ((self.x-B.x).powi(2) + (self.y-B.y).powi(2) + (self.y-B.y).powi(2)).powf(0.5) as f32
    }
    pub fn clamp(&self, target_low: Vec3, target_high: Vec3) -> Vec3 {
        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;
        if self.x < target_low.x {x = target_low.x}
        if self.x > target_high.x {x = target_high.x}
        if self.y < target_low.y {y = target_low.y}
        if self.y > target_high.y {y = target_high.y}
        if self.z < target_low.z {z = target_low.z}
        if self.z > target_high.z {z = target_high.z}
        Vec3 {x, y, z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z)
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x+rhs.x,self.y+rhs.y,self.z+rhs.z)
    }
}

impl Triangle {
    pub fn new(A: Vec3, B : Vec3, C: Vec3) -> Triangle {
        Triangle {
            A, B,  C
        }
    }
    pub fn intersects(&self, ray: &Ray) -> Lexip {
        if self.facing(ray.point) { // Check if the plane is facing the camera
            return Lexip::empty();
        }
        let normal = self.get_normal();
        let denominator = Vec3::dot(normal, ray.vector);
        if denominator >= 0.0 {return Lexip::empty();}

        let t: f32 = Vec3::dot(normal, self.A - ray.point)/denominator;

        let intercept: Vec3 = ray.get_point(t);

        let a: f32 = Vec3::bary(self.B - self.A, self.B - self.C, intercept - self.A);
        let b: f32 = Vec3::bary(self.C - self.B, self.C - self.A, intercept - self.B);
        let c: f32 = Vec3::bary(self.A - self.C, self.A - self.B, intercept - self.C);

        if a >= -0.01 && b >= -0.01 && c >= -0.01 && a+b+c <= 1.01 { 
            let distance = (ray.point + intercept).len() as f32;
            return Lexip{
                red: 255 as u8,
                green: 255 as u8,
                blue: 255 as u8,
                distance,
                collision_object: Shape::Triangle(self.clone()),
                collision_normal: self.get_normal().clone()
            };
        }
        Lexip::empty()
    }
    pub fn rotateZ(&mut self, angle:f32, origin:Vec3) {
        self.A.rotateZ(angle, origin.clone());
        self.B.rotateZ(angle, origin.clone());
        self.C.rotateZ(angle, origin.clone());
    }
    pub fn rotateY(&mut self, angle:f32, origin:Vec3) {
        self.A.rotateY(angle, origin.clone());
        self.B.rotateY(angle, origin.clone());
        self.C.rotateY(angle, origin.clone());
    }
    pub fn rotateX(&mut self, angle:f32, origin:Vec3) {
        self.A.rotateX(angle, origin.clone());
        self.B.rotateX(angle, origin.clone());
        self.C.rotateX(angle, origin.clone());
    }
    pub fn translate(&mut self, translation:Vec3) {
        self.A = self.A + translation;
        self.B = self.B + translation;
        self.C = self.C + translation;
    }
    pub fn get_normal(&self) -> Vec3 {
        let a: Vec3 = self.B - self.A;
        let b: Vec3 = self.C - self.A;
        let normal: Vec3 = Vec3 {   
            x: a.y*b.z-a.z*b.y,
            y: a.z*b.x-a.x*b.z,
            z: a.x*b.y-a.y*b.x
        };
        normal
    }
    pub fn facing(&self, ray: Vec3) -> bool {
        Vec3::dot(self.get_normal(), self.A - ray) > 0.0
    }
}

pub fn square(top_left: Vec3, top_right: Vec3, bottom_left: Vec3, bottom_right: Vec3) -> [Triangle; 2] {
    [
        Triangle::new(top_left.clone(), top_right, bottom_right.clone()),
        Triangle::new(top_left, bottom_right, bottom_left)
    ]
}

impl Surface {
    pub fn rotate(&mut self, rotation: Vec3, origin: Vec3) {
        self.bottom_left.rotate(rotation, origin);
        self.bottom_right.rotate(rotation, origin);
        self.top_left.rotate(rotation, origin);
        self.top_right.rotate(rotation, origin);
    }
}

impl Cube {
    pub fn new(position: Vec3, size: f32) -> Cube {
        Cube {
            size,
            rotation: Vec3{x:0.0,y:0.0,z:0.0},
            position: position.clone()
        }
    }
    fn get_triangles(&self) -> [Triangle; 12] {
        let verticies = [
            Vec3::new(-self.size, -self.size, -self.size)    + self.position,   // B Back left      0
            Vec3::new(self.size, -self.size, -self.size)     + self.position,   // B Back right     1
            Vec3::new(self.size, -self.size, self.size)      + self.position,   // B Front right    2
            Vec3::new(-self.size, -self.size, self.size)     + self.position,   // B Front left     3
            Vec3::new(-self.size, self.size, -self.size)     + self.position,   // T Back left      4
            Vec3::new(self.size, self.size, -self.size)      + self.position,   // T Back right     5
            Vec3::new(self.size, self.size, self.size)       + self.position,   // T Front right    6
            Vec3::new(-self.size, self.size, self.size)      + self.position,   // T Front left     7
        ];
        [
            // Bottom
            Triangle::new(verticies[0], verticies[1], verticies[2]), Triangle::new(verticies[0], verticies[2], verticies[3]),
            // Back
            Triangle::new(verticies[5], verticies[1], verticies[0]), Triangle::new(verticies[0], verticies[4], verticies[5]),
            // Left
            Triangle::new(verticies[0], verticies[3], verticies[7]), Triangle::new(verticies[0], verticies[7], verticies[4]),
            // Right
            Triangle::new(verticies[2], verticies[1], verticies[5]), Triangle::new(verticies[2], verticies[5], verticies[6]),
            // Front
            Triangle::new(verticies[3], verticies[2], verticies[6]), Triangle::new(verticies[3], verticies[6], verticies[7]),
            // Top
            Triangle::new(verticies[4], verticies[6], verticies[5]), Triangle::new(verticies[4], verticies[7], verticies[6]),
        ]
    }
    pub fn intersects(&self, ray:&Ray) -> Lexip {
        let mut z: Lexip = Lexip::empty();
        for i in self.get_triangles() {
            let c = i.intersects(ray);
            if c.distance > z.distance && c.distance != -1.0 || z.distance == -1.0 {
                z = c;
            }
        }
        z
    }
    pub fn rotateZ(&mut self, angle:f32, origin:Vec3) {
        self.position.rotateZ(angle, origin.clone());
        self.rotation.rotateZ(angle, origin);
    }
    pub fn rotateY(&mut self, angle:f32, origin:Vec3) {
        self.position.rotateY(angle, origin.clone());
        self.rotation.rotateY(angle, origin);
    }
    pub fn rotateX(&mut self, angle:f32, origin:Vec3) {
        self.position.rotateX(angle, origin.clone());
        self.rotation.rotateX(angle, origin);
    }
    pub fn translate(&mut self, translation:Vec3) {
        self.position = self.position + translation;
    }
}

impl Camera {
    pub fn get_surface(&self) -> Surface {
        let z: f32 = self.position.z+((self.width as f32/2.0)/((self.FOV/2.0).tan()));
        Surface {
            top_left: Vec3{x:self.position.x-(self.width as f32/2.0),y:self.position.y+(self.height as f32/2.0),z},
            top_right: Vec3{x:self.position.x+(self.width as f32/2.0),y:self.position.y+(self.height as f32/2.0),z},
            bottom_left: Vec3{x:self.position.x-(self.width as f32/2.0),y:self.position.y-(self.height as f32/2.0),z},
            bottom_right: Vec3{x:self.position.x+(self.width as f32/2.0),y:self.position.y-(self.height as f32/2.0),z}
        }
    }
    pub fn new(position: Vec3, rotation: Vec3, FOV: f32, width: usize, height: usize, pixel_size: usize) -> Camera {
        Camera {
            position: position.clone(),
            rotation,
            FOV,
            pixel_size,
            width,
            height
        }
    }
    pub fn get_pixel(&self, ox: usize, oy: usize, shapes: &[Shape], lights: &[Lights]) -> Lexip {

        let x: f32 = self.get_surface().bottom_left.x + ox as f32;
        let y: f32 = self.get_surface().bottom_left.y + oy as f32;
        let z: f32 = self.get_surface().bottom_left.z;

        let mut pixel_pos: Vec3 = Vec3::new(x,y,z);

        pixel_pos.rotate(self.rotation, self.position);

        let ray: Ray = Ray {
            point: pixel_pos,
            vector: Vec3::new(pixel_pos.x-self.position.x,pixel_pos.y-self.position.y,pixel_pos.z-self.position.z)
        };

        let mut current_z: Lexip = Lexip::empty();

        // Get shape collision
        for i in shapes.iter() {
            let c: Lexip = i.intersects(&ray);
            if (c.distance > current_z.distance && c.distance != -1.0) || current_z.distance == -1.0 {current_z = c;}
        }
        
        // Add lights
        if current_z.collision_object != Shape::None {
            for i in lights.iter()
            {
                match i {
                    Lights::Sun(x) => {
                        let d: f32 = Vec3::dot(current_z.collision_normal.normalize(), x.direction.normalize());
                        if d < 0.0 {
                            current_z.red = ((current_z.red as f32 + x.colour.x * -d)/2.0) as u8;
                            current_z.green = ((current_z.green as f32 + x.colour.y * -d)/2.0) as u8;
                            current_z.blue = ((current_z.blue as f32 + x.colour.z * -d)/2.0) as u8;
                            // return current_z;
                        }
                    },
                    #[allow(unreachable_patterns)]
                    _=> {}
                }
            }
        }

        current_z
    }
    pub fn translate(&mut self, translation: Vec3) {
        self.position = self.position + translation;
    }
    pub fn move_forward(&mut self, vel: Vec3) {
        let mut vel = vel;
        vel.rotate(self.rotation, Vec3{x: 0.0, y: 0.0, z: 0.0});
        self.translate(vel);
    }
    pub fn rotate(&mut self, rotation: Vec3) {
        let mut rotation = rotation;
        if (rotation.x.to_degrees() % 360.0).abs() < 270.0 && (rotation.x.to_degrees() % 360.0).abs() > 90.0 {
            rotation.y*=-1.0;
        }
        self.rotation = self.rotation + rotation;
    }
}

impl Surface {
    pub fn width(&self) -> f32 {
        self.top_right.x - self.bottom_left.x
    }
    pub fn height(&self) -> f32 {
        self.top_right.y - self.bottom_left.y
    }
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}
#[derive(Clone)]
pub struct Lexip {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub distance: f32,
    pub collision_object: Shape,
    pub collision_normal: Vec3
}

impl Lexip{
    pub fn empty() -> Lexip {
        Lexip {
            red:0,
            green:0,
            blue:0,
            distance:-1.0,
            collision_object:Shape::None,
            collision_normal: Vec3 { x: -1.0, y: -1.0, z: -1.0 }
        }
    }
}

pub struct Sun {
    pub direction: Vec3,
    pub colour: Vec3
}



fn clampf(n: f32, low: f32, high: f32) -> f32 {
    if n > high {
        return high;
    }
    if n < low {
        return low;
    } n
}