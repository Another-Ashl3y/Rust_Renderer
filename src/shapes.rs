#![allow(dead_code)]
#![allow(non_snake_case)]

use std::ops::Index;


pub enum Color {
    White,
    Grey,
    Black
}

pub enum Shape {
    Triangle(Triangle),
    Cube(Box<Cube>)
}

impl Shape {
    pub fn intersects(&self, ray:&Ray) -> f64 {
        match self {
            Shape::Triangle(x) => {return x.intersects(ray)},
            Shape::Cube(x) => {return x.intersects(ray)},
            #[allow(unreachable_patterns)]
            _ => println!("A shape is not yet implemented")
        }
        -1.0
    }
    pub fn rotate_xy(&mut self, angle:f64, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_xy(angle, origin),
            Shape::Cube(x) => x.rotate_xy(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotate_xz(&mut self, angle:f64, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_xz(angle, origin),
            Shape::Cube(x) => x.rotate_xz(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotate_yz(&mut self, angle:f64, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_yz(angle, origin),
            Shape::Cube(x) => x.rotate_yz(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

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
    pixel_size: u32
}

pub struct Surface {
    top_left: Vec3,
    top_right: Vec3,
    bottom_left: Vec3,
    bottom_right: Vec3
}

pub struct Triangle {
    pub A: Vec3, // Point A
    pub B: Vec3, // Point B
    pub C: Vec3, // Point C
}

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
    pub fn rotate_xy(&mut self, angle:f64, origin: Vec3) {        
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.y-origin.y)*-angle.sin())+origin.x,
            y: ((self.x-origin.x)*angle.sin()+(self.y-origin.y)*angle.cos())+origin.y,
            ..self.clone()
        };
    }
    pub fn rotate_xz(&mut self, angle:f64, origin: Vec3) {
        *self = Self {
            x: ((self.x-origin.x)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.x,
            z: ((self.x-origin.x)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z,
            ..self.clone()
        }
    }
    pub fn rotate_yz(&mut self, angle:f64, origin: Vec3) {
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
    pub fn intersects(&self, ray: &Ray) -> f64 {

        // let accuracy: u32 = 16;

        if Vec3::dot(&self.get_normal(), &Vec3::sub(&self.A, &ray.point)) > 0.0 { // Check if the plane is facing the camera
            return -1.0;
        }

        let denominator = Vec3::dot(&self.get_normal(), &ray.vector);
        // println!("Den {}", denominator);
        // println!("Normal {} {} {}", self.N.x, self.N.y, self.N.z);
        if denominator == 0.0 {return -1.0}

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
            // println!("{}",Vec3::sub(&ray.point, &intercept).len());
            return Vec3::sub(&ray.point, &intercept).len();
        }
        -1.0
    }
    pub fn rotate_xy(&mut self, angle:f64, origin:Vec3) {
        self.A.rotate_xy(angle, origin.clone());
        self.B.rotate_xy(angle, origin.clone());
        self.C.rotate_xy(angle, origin.clone());
    }
    pub fn rotate_xz(&mut self, angle:f64, origin:Vec3) {
        self.A.rotate_xz(angle, origin.clone());
        self.B.rotate_xz(angle, origin.clone());
        self.C.rotate_xz(angle, origin.clone());
    }
    pub fn rotate_yz(&mut self, angle:f64, origin:Vec3) {
        self.A.rotate_yz(angle, origin.clone());
        self.B.rotate_yz(angle, origin.clone());
        self.C.rotate_yz(angle, origin.clone());
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
            tri.rotate_xy(self.rotation.z, self.position.clone());
            tri2.rotate_xy(self.rotation.z, self.position.clone());
            tri.rotate_yz(self.rotation.x, self.position.clone());
            tri2.rotate_yz(self.rotation.x, self.position.clone());
            tri.rotate_xz(self.rotation.y, self.position.clone());
            tri2.rotate_xz(self.rotation.y, self.position.clone());
            triangles.push(tri);
            triangles.push(tri2);
        };
        triangles
        
    }
    pub fn intersects(&self, ray:&Ray) -> f64 {
        let mut z: f64 = -1.0;
        for i in self.get_triangles() {
            let c = i.intersects(ray);
            if c < z && c != -1.0 || z == -1.0 {
                z = c;
            }
        }
        z
    }
    pub fn rotate_xy(&mut self, angle:f64, origin:Vec3) {
        self.position.rotate_xy(angle, origin.clone());
        self.rotation.rotate_xy(angle, origin);
    }
    pub fn rotate_xz(&mut self, angle:f64, origin:Vec3) {
        self.position.rotate_xz(angle, origin.clone());
        self.rotation.rotate_xz(angle, origin);
    }
    pub fn rotate_yz(&mut self, angle:f64, origin:Vec3) {
        self.position.rotate_yz(angle, origin.clone());
        self.rotation.rotate_yz(angle, origin);
    }
}

impl Camera {
    pub fn new(position: Vec3, FOV: f64, width: u32, height: u32, pixel_size: u32) -> Camera {
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
    pub fn get_pixel(&self, ox: u32, oy: u32, shapes: &[Shape]) -> Pixel {
        let x: f64 = ox as f64 + self.screen.bottom_left.x;
        let y: f64 = oy as f64 + self.screen.bottom_left.y;
        
        let ray: Ray = Ray {
            point: Vec3{x,y,z: self.screen.bottom_left.z},
            vector: Vec3{x: x-self.position.x,y: y-self.position.y,z: self.screen.bottom_left.z-self.position.z}
        };
        let mut current_z: f64 = -1.0;
        for i in shapes.iter() {
            let c: f64 = i.intersects(&ray);
            if (c < current_z && c != -1.0) || current_z == -1.0 {current_z = c;}
        }
        // if current_z > 0.0 { println!("{}",current_z); }
        // if current_z > 60.0 {return Pixel { position: Vec2{x:ox as f64,y:oy as f64}, value: Color::GREY };}
        // if current_z > 0.0 {return Pixel { position: Vec2{x:ox as f64,y:oy as f64}, value: Color::WHITE };}
        Pixel { position: Vec2{x:ox as f64,y:oy as f64}, value: current_z }
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
