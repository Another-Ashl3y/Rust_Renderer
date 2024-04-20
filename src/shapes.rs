#![allow(dead_code)]
#![allow(non_snake_case)]

use std::io::Read;
use std::mem::discriminant;
use std::ops::Index;
use std::fs::File;

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
    Object(Object),
    None
}

impl Shape {
    pub fn intersects(&self, ray:&Ray) -> Lexip {
        match self {
            Shape::Triangle(x) => {return x.intersects(ray)},
            Shape::Cube(x) => {return x.intersects(ray)},
            Shape::Object(x) => {return x.intersects(ray)},
            #[allow(unreachable_patterns)]
            _ => println!("A shape is not yet implemented")
        }
        Lexip::empty()
    }
    pub fn rotateZ(&mut self, angle:f64, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotateZ(angle, origin),
            Shape::Cube(x) => x.rotateZ(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotateY(&mut self, angle:f64, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotateY(angle, origin),
            Shape::Cube(x) => x.rotateY(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotateX(&mut self, angle:f64, origin:Vec3) {        
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
}

pub enum Lights {
    Sun(Sun)
}


pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

#[derive(PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct Pixel {
    pub position: Vec2,
    pub value: f64
}

pub struct Ray {
    pub point: Vec3,
    pub vector: Vec3
}

pub struct Camera {
    position: Vec3,
    screen: Surface,
    FOV: f64,
    pixel_size: usize
}

pub struct Surface {
    top_left: Vec3,
    top_right: Vec3,
    bottom_left: Vec3,
    bottom_right: Vec3
}
#[derive(Clone)]
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
    size:f64
}

impl Ray {
    pub fn get_point(&self, t:f64) -> Vec3 {
        Vec3 {x: self.point.x + (self.vector.x*t), y: self.point.y + (self.vector.y*t), z: self.point.z + (self.vector.z*t)}
    }
}

impl Vec3 {
    pub fn as_Vec2 (&self) -> Vec2 {
        Vec2{
            x: self.x,
            y: self.y
        }
    }
    pub fn as_string(&self) -> String {
        format!("({}, {}, {})",self.x,self.y,self.z).to_string()
    }
    pub fn rotateZ(&mut self, angle:f64, origin: Vec3) {        
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.y-origin.y)*-angle.sin())+origin.x,
            y: ((self.x-origin.x)*angle.sin()+(self.y-origin.y)*angle.cos())+origin.y,
            ..self.clone()
        };
    }
    pub fn rotateY(&mut self, angle:f64, origin: Vec3) {
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.x,
            z: ((self.x-origin.x)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z,
            ..self.clone()
        }
    }
    pub fn rotateX(&mut self, angle:f64, origin: Vec3) {
        *self = Self {
            y: ((self.y-origin.y)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.y,
            z: ((self.y-origin.y)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z,
            ..self.clone()
        }
    }
    pub fn len(&self) -> f64 {
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).powf(0.5) // Return the length of vector
    }
    pub fn normalize(&self) -> Vec3{
        let length:f64 = self.len();
        Vec3{
            x: self.x/length,
            y: self.y/length,
            z: self.z/length
        }
    }
    pub fn dot(A:&Vec3, B:&Vec3) -> f64 {
        A.x*B.x+A.y*B.y+A.z*B.z
    }
    pub fn sub(A:&Vec3, B:&Vec3) -> Vec3 {
        Vec3 {
            x: A.x - B.x,
            y: A.y - B.y,
            z: A.z - B.z
        }
    }
    pub fn add(A:&Vec3, B:&Vec3) -> Vec3 {
        Vec3 {
            x: A.x+B.x,
            y: A.y+B.y,
            z: A.z+B.z
        }
    }
    pub fn mul(A:&Vec3, B:&Vec3) -> Vec3 {
        Vec3 {
            x: A.x * B.x,
            y: A.y * B.y,
            z: A.z * B.z
        }
    }
    pub fn mul_f(A:&Vec3, B:f64) -> Vec3 {
        Vec3 {
            x: A.x * B,
            y: A.y * B,
            z: A.z * B
        }
    }
    pub fn x_to_midpoint(A: &Vec3, B: &Vec3, C:&Vec3) -> Vec3 {
        let mid = Vec3::midpoint(B, C);
        Vec3::sub(&mid, A)
    }
    pub fn midpoint(A: &Vec3, B: &Vec3) -> Vec3{
        Vec3{x: (A.x+B.x)/2.0, y: (A.y+B.y)/2.0, z: (A.z+B.z)/2.0}
    }
    pub fn proj(A: &Vec3, B:&Vec3) -> Vec3 {
        Vec3::mul_f(A, Vec3::dot(A, B)/Vec3::dot(A, A))
    }
    pub fn bary(AB: &Vec3, CB:&Vec3, AI:&Vec3) -> f64 {
        let AV: Vec3 = Vec3::sub(AB, &Vec3::proj(CB, AB));

        1.0 - Vec3::dot(&AV, AI)/Vec3::dot(&AV, AB)
    }

}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3{x:self.x, y:self.y, z:self.z}
    }
}

impl Triangle {
    pub fn new(A: Vec3, B : Vec3, C: Vec3) -> Triangle {
        Triangle {
            A, B,  C
        }
    }
    pub fn intersects(&self, ray: &Ray) -> Lexip {

        // let accuracy: u32 = 16;

        if self.facing(&ray.point) { // Check if the plane is facing the camera
            return Lexip::empty();
        }

        let denominator = Vec3::dot(&self.get_normal(), &ray.vector);
        // println!("Den {}", denominator);
        // println!("Normal {} {} {}", self.N.x, self.N.y, self.N.z);
        if denominator == 0.0 {return Lexip::empty();}

        let t: f64 = Vec3::dot(&self.get_normal(), &Vec3::sub(&self.A, &ray.point))/denominator;

        let intercept: Vec3 = ray.get_point(t);

        // let v:Vec3 = Vec3::x_to_midpoint(&self.A, &self.B, &self.C);

        let AB: Vec3 = Vec3::sub(&self.B, &self.A);
        let CB: Vec3 = Vec3::sub(&self.B, &self.C);
        let AI: Vec3 = Vec3::sub(&intercept, &self.A);
        let a: f64 = Vec3::bary(&AB, &CB, &AI);

        let BC: Vec3 = Vec3::sub(&self.C, &self.B);
        let AC: Vec3 = Vec3::sub(&self.C, &self.A);
        let BI: Vec3 = Vec3::sub(&intercept, &self.B);
        let b: f64 = Vec3::bary(&BC, &AC, &BI);

        let CA: Vec3 = Vec3::sub(&self.A, &self.C);
        let BA: Vec3 = Vec3::sub(&self.A, &self.B);
        let CI: Vec3 = Vec3::sub(&intercept, &self.C);
        let c: f64 = Vec3::bary(&CA, &BA, &CI);

        // println!("{} {} {} {}", a,b,c,a+b+c);
        // let angle = Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point));
        if a >= -0.01 && b >= -0.01 && c >= -0.01 && a+b+c <= 1.01 { // determine if it is inside the triangle using barycentric coordinates
            let distance = Vec3::sub(&ray.point, &intercept).len() as f32;
            return Lexip{
                red: (255.0 * (5000.0/distance.powf(2.0))) as u8,
                green: (255.0 * (5000.0/distance.powf(2.0))) as u8,
                blue: (255.0 * (5000.0/distance.powf(2.0))) as u8,
                distance,
                collision_object: Shape::Triangle(self.Copy()),
                collision_normal: self.get_normal().clone()
            };
        }
        Lexip::empty()
    }
    pub fn rotateZ(&mut self, angle:f64, origin:Vec3) {
        self.A.rotateZ(angle, origin.clone());
        self.B.rotateZ(angle, origin.clone());
        self.C.rotateZ(angle, origin.clone());
    }
    pub fn rotateY(&mut self, angle:f64, origin:Vec3) {
        self.A.rotateY(angle, origin.clone());
        self.B.rotateY(angle, origin.clone());
        self.C.rotateY(angle, origin.clone());
    }
    pub fn rotateX(&mut self, angle:f64, origin:Vec3) {
        self.A.rotateX(angle, origin.clone());
        self.B.rotateX(angle, origin.clone());
        self.C.rotateX(angle, origin.clone());
    }
    pub fn translate(&mut self, translation:Vec3) {
        self.A = Vec3::add(&self.A, &translation);
        self.B = Vec3::add(&self.B, &translation);
        self.C = Vec3::add(&self.C, &translation);
    }
    pub fn get_normal(&self) -> Vec3 {
        let a: Vec3 = Vec3::sub(&self.B, &self.A);
        let b: Vec3 = Vec3::sub(&self.C, &self.A);
        let normal: Vec3 = Vec3 {   
            x: a.y*b.z-a.z*b.y,
            y: a.z*b.x-a.x*b.z,
            z: a.x*b.y-a.y*b.x};
        normal.normalize()
    }
    pub fn Copy(&self) -> Triangle {
        Triangle {
            A: self.A.clone(),
            B: self.B.clone(),
            C: self.C.clone()
        }
    }
    pub fn facing(&self, ray: &Vec3) -> bool {
        Vec3::dot(&self.get_normal(), &Vec3::sub(&self.A, &ray)) > 0.0
    }
}

pub fn square(top_left: Vec3, top_right: Vec3, bottom_left: Vec3, bottom_right: Vec3) -> [Triangle; 2] {
    [
        Triangle::new(top_left.clone(), top_right, bottom_right.clone()),
        Triangle::new(top_left, bottom_right, bottom_left)
    ]
}


impl Cube {
    pub fn new(position: Vec3, size: f64) -> Cube {
        Cube {
            size,
            rotation: Vec3{x:0.0,y:0.0,z:0.0},
            position: position.clone()
        }
    }
    fn get_triangles(&self) -> Vec<Triangle> {
        let faces: [[Triangle; 2]; 6] = [
            square(
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:-self.size}, &self.position),
            Vec3::add(&Vec3{x:self.size,y:self.size,z:-self.size}, &self.position), 
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:-self.size}, &self.position),
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:-self.size}, &self.position), 
        ),
            square(
            Vec3::add(&Vec3{x:self.size,y:self.size,z:self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:self.size}, &self.position), 
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:self.size}, &self.position), 
        ),
            square(
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:-self.size}, &self.position), 
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:-self.size}, &self.position), 
        ),
            square(
            Vec3::add(&Vec3{x:self.size,y:self.size,z:-self.size}, &self.position),
            Vec3::add(&Vec3{x:self.size,y:self.size,z:self.size}, &self.position), 
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:-self.size}, &self.position),
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:self.size}, &self.position), 
        ),
            square(
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:self.size}, &self.position), //     - + +
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:self.size}, &self.position), //     + + +
            Vec3::add(&Vec3{x:self.size,y:-self.size,z:-self.size}, &self.position), // - + - 
            Vec3::add(&Vec3{x:-self.size,y:-self.size,z:-self.size}, &self.position), // + + -
        ),
            square(
            Vec3::add(&Vec3{x:self.size,y:self.size,z:-self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:-self.size}, &self.position), 
            Vec3::add(&Vec3{x:self.size,y:self.size,z:self.size}, &self.position),
            Vec3::add(&Vec3{x:-self.size,y:self.size,z:self.size}, &self.position), 
        )];
        let mut triangles: Vec<Triangle> = Vec::new();
        for i in faces {
            let mut tri = i[0].Copy();
            let mut tri2 = i[1].Copy();
            tri.rotateZ(self.rotation.z, self.position.clone());
            tri2.rotateZ(self.rotation.z, self.position.clone());
            tri.rotateX(self.rotation.x, self.position.clone());
            tri2.rotateX(self.rotation.x, self.position.clone());
            tri.rotateY(self.rotation.y, self.position.clone());
            tri2.rotateY(self.rotation.y, self.position.clone());
            triangles.push(tri);
            triangles.push(tri2);
        };
        triangles
        
    }
    pub fn intersects(&self, ray:&Ray) -> Lexip {
        let mut z: Lexip = Lexip::empty();
        for i in self.get_triangles() {
            let c = i.intersects(ray);
            if c.distance < z.distance && c.distance != -1.0 || z.distance == -1.0 {
                z = c;
            }
        }
        if z.collision_object != Shape::None {z.collision_object = Shape::Cube(self.clone())}
        z
    }
    pub fn rotateZ(&mut self, angle:f64, origin:Vec3) {
        self.position.rotateZ(angle, origin.clone());
        self.rotation.rotateZ(angle, origin);
    }
    pub fn rotateY(&mut self, angle:f64, origin:Vec3) {
        self.position.rotateY(angle, origin.clone());
        self.rotation.rotateY(angle, origin);
    }
    pub fn rotateX(&mut self, angle:f64, origin:Vec3) {
        self.position.rotateX(angle, origin.clone());
        self.rotation.rotateX(angle, origin);
    }
    pub fn translate(&mut self, translation:Vec3) {
        self.position = Vec3::add(&self.position, &translation);
    }
}

impl Camera {
    pub fn new(position: Vec3, FOV: f64, width: usize, height: usize, pixel_size: usize) -> Camera {
        let z: f64 = position.z+((width as f64/2.0)/((FOV/2.0).tan()));
        Camera {
            position: position.clone(),
            FOV,
            pixel_size,
            screen: Surface {
                top_left: Vec3{x:position.x-(width as f64/2.0),y:position.y+(height as f64/2.0),z},
                top_right: Vec3{x:position.x+(width as f64/2.0),y:position.y+(height as f64/2.0),z},
                bottom_left: Vec3{x:position.x-(width as f64/2.0),y:position.y-(height as f64/2.0),z},
                bottom_right: Vec3{x:position.x+(width as f64/2.0),y:position.y-(height as f64/2.0),z}
            }
        }
    }
    pub fn get_pixel(&self, ox: usize, oy: usize, shapes: &[Shape], lights: &[Lights]) -> Lexip {
        let x: f64 = ox as f64 + self.screen.bottom_left.x;
        let y: f64 = oy as f64 + self.screen.bottom_left.y;
        
        let ray: Ray = Ray {
            point: Vec3{x,y,z: self.screen.bottom_left.z},
            vector: Vec3{x: x-self.position.x,y: y-self.position.y,z: self.screen.bottom_left.z-self.position.z}
        };
        let mut current_z: Lexip = Lexip::empty();
        for i in shapes.iter() {
            let c: Lexip = i.intersects(&ray);
            if (c.distance < current_z.distance && c.distance != -1.0) || current_z.distance == -1.0 {current_z = c;}
        }
        // if current_z > 0.0 { println!("{}",current_z); }
        // if current_z > 60.0 {return Pixel { position: Vec2{x:ox as f64,y:oy as f64}, value: Color::GREY };}
        // if current_z > 0.0 {return Pixel { position: Vec2{x:ox as f64,y:oy as f64}, value: Color::WHITE };}
        if current_z.collision_object != Shape::None {
        for i in lights.iter() {
            match i {
                Lights::Sun(x) => {
                    if Vec3::dot(&current_z.collision_normal, &x.direction) < 0.0 {
                        current_z.red = ((current_z.red as f64 + x.colour.x)/2.0) as u8;
                        current_z.green = ((current_z.green as f64 + x.colour.y)/2.0) as u8;
                        current_z.blue = ((current_z.blue as f64 + x.colour.z)/2.0) as u8;
                    }
                },
                _=> {}
            }
        }}
        current_z
    }
}

impl Surface {
    pub fn width(&self) -> f64 {
        self.top_right.x - self.bottom_left.x
    }
    pub fn height(&self) -> f64 {
        self.top_right.y - self.bottom_left.y
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Object {
    position: Vec3,
    rotation: Vec3,
    scale: f64,
    faces: Vec<Triangle>
}

impl Object {
    pub fn new(path: &str, position: Vec3, rotation: Vec3, scale: f64) -> Object {

        let mut file = File::open(path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents = contents.replace("\\\n", " ");

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut faces: Vec<Triangle> = Vec::new();
        for line in contents.lines() {
            if line.starts_with('v') {
                let l:Vec<&str> = line.get(2..).unwrap().split_whitespace().collect();
                if l.len() == 3 {
                    vertices.push(Vec3 { 
                        x: l[0].parse::<f64>().unwrap()*scale, 
                        y: l[1].parse::<f64>().unwrap()*scale, 
                        z: l[2].parse::<f64>().unwrap()*scale
                    });
                }
            }
            else if line.starts_with('f') {
                let l: Vec<&str> = line.get(2..).unwrap().split_whitespace().collect::<Vec<_>>()[0].split('/').collect();
                    faces.push(Triangle::new(
                        vertices[l[0].parse::<usize>().unwrap()-1].clone(), 
                        vertices[l[1].parse::<usize>().unwrap()-1].clone(), 
                        vertices[l[2].parse::<usize>().unwrap()-1].clone()
                    ));
            }
        };
        Object {
            position,
            rotation,
            scale,
            faces
        }
    }
    pub fn intersects (&self, ray: &Ray) -> Lexip {
        let mut z: Lexip = Lexip::empty();
        for i in &self.faces {
            let c: Lexip = i.intersects(ray);
            if c.distance < z.distance && c.distance != -1.0 || z.distance == -1.0 {
                z = c;
            }
        }
        if z.collision_object != Shape::None {z.collision_object = Shape::Object(self.clone())}
        z
    }
    pub fn new_ball(pos: Vec3, rotation: Vec3, radius: f32, res: f32) -> Object {
        if res <= 0.0 {panic!("resolution must be positive")}
        let mut faces: Vec<Triangle> = Vec::new();
        let mut points: Vec<Vec3> = Vec::new();
        let mut flat_points: Vec<Vec2> = Vec::new();
        let mut x: f32 = -radius;
        while x < radius {
            flat_points.push(Vec2 {x: x as f64, y: (radius.powi(2)-x.powi(2)).powf(0.5) as f64});
            flat_points.push(Vec2 {x: x as f64, y: -(radius.powi(2)-x.powi(2)).powf(0.5) as f64});
            x += res;
        }
        for i in flat_points {
            points.push(Vec3{x: i.x + pos.x, y: i.y + pos.y, z: (radius.powi(2) as f64 -(i.x.powi(2) + i.y.powi(2))).powf(0.5) + pos.z});
            points.push(Vec3{x: i.x + pos.x, y: i.y + pos.y, z: -(radius.powi(2) as f64 -(i.x.powi(2) + i.y.powi(2))).powf(0.5) + pos.z});
        }
        for x in &points {
            for y in &points {
                for z in &points {
                    faces.push(Triangle{A:x.clone(), B: y.clone(), C: z.clone()});
                    println!("Faces : {}", faces.len());
                }
            }
        }
        Object {
            position: pos,
            rotation,
            scale: 1.0,
            faces
        }
    }
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
    pub position: Vec3,
    pub colour: Vec3,
    pub intensity: i32
}



