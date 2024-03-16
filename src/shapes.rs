#![allow(dead_code)]
#![allow(non_snake_case)]

use std::ops::Index;


pub enum Color {
    WHITE,
    GREY,
    BLACK
}

pub enum Shape {
    Triangle(Triangle),
    Square(Square),
    Cube(Cube)
}

impl Shape {
    pub fn intersects(&self, ray:&Ray) -> f32 {
        match self {
            Shape::Triangle(x) => {return x.intersects(ray)},
            Shape::Cube(x) => {return x.intersects(ray)},
            Shape::Square(x) => {return x.intersects(ray)},
            #[allow(unreachable_patterns)]
            _ => println!("A shape is not yet implemented")
        }
        return -1.0;
    }
    pub fn rotate_xy(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_xy(angle, origin),
            Shape::Cube(x) => x.rotate_xy(angle, origin),
            Shape::Square(x) => x.rotate_xy(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotate_xz(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_xz(angle, origin),
            Shape::Cube(x) => x.rotate_xz(angle, origin),
            Shape::Square(x) => x.rotate_xz(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
    pub fn rotate_yz(&mut self, angle:f32, origin:Vec3) {        
        match self {
            Shape::Triangle(x) => x.rotate_yz(angle, origin),
            Shape::Cube(x) => x.rotate_yz(angle, origin),
            Shape::Square(x) => x.rotate_yz(angle, origin),
            #[allow(unreachable_patterns)]
            _ => println!("A shapes rotation is not yet implemented")
        }
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
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
    position: Vec3,
    screen: Surface,
    FOV: f32,
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
    pub N: Vec3,  // Normal Vector
    pub center: Vec3
}

pub struct Square {
    A: Triangle,
    B: Triangle
}

pub struct Cube {
    position:Vec3,
    size:f32,
    front: Square,
    back: Square,
    left: Square,
    right: Square,
    top: Square,
    bottom: Square
}

impl Ray {
    pub fn get_point(&self, t:f32) -> Vec3 {
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
    pub fn rotate_xy(&mut self, angle:f32, origin: Vec3) {
        self.x = ((self.x-origin.x)*angle.cos()+(self.y-origin.y)*-angle.sin())+origin.x;
        self.y = ((self.x-origin.x)*angle.sin()+(self.y-origin.y)*angle.cos())+origin.y;
    }
    pub fn rotate_xz(&mut self, angle:f32, origin: Vec3) {
        self.x = ((self.x-origin.x)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.x;
        self.z = ((self.x-origin.x)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z;
    }
    pub fn rotate_yz(&mut self, angle:f32, origin: Vec3) {
        self.y = ((self.y-origin.y)*angle.cos()+(self.z-origin.z)*-angle.sin())+origin.y;
        self.z = ((self.y-origin.y)*angle.sin()+(self.z-origin.z)*angle.cos())+origin.z;
    }
    pub fn len(&self) -> f32 {
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).powf(0.5) // Return the length of vector
    }
    pub fn normalize(&self) -> Vec3{
        let length:f32 = self.len();
        Vec3{
            x: self.x/length,
            y: self.y/length,
            z: self.z/length
        }
    }
    pub fn dot(A:&Vec3, B:&Vec3) -> f32 {
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
    pub fn mul_f(A:&Vec3, B:f32) -> Vec3 {
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
    pub fn bary(AB: &Vec3, CB:&Vec3, AI:&Vec3) -> f32 {
        let AV: Vec3 = Vec3::sub(&AB, &Vec3::proj(&CB, &AB));

        let a = 1.0 - Vec3::dot(&AV, &AI)/Vec3::dot(&AV, &AB);
        a
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3{x:self.x, y:self.y, z:self.z}
    }
}

impl Triangle { // A, B, C, Normal, Center
    pub fn new(A: Vec3, B : Vec3, C: Vec3) -> Triangle {
        let center = Vec3 {   x: (A.x + B.x + C.x)/3.0,
                                    y: (A.y + B.y + C.y)/3.0,
                                    z: (A.z + B.z + C.z)/3.0};
        let a = Vec3::sub(&B, &A);
        let b = Vec3::sub(&C, &A);

        let mut normal: Vec3 = Vec3 {   
            x: a.y*b.z-a.z*b.y,
            y: a.z*b.x-a.x*b.z,
            z: a.x*b.y-a.y*b.x};
        normal = normal.normalize();

        Triangle {
            A: A, B: B, C: C,
            N: normal,
            center: center
        }
    }
    pub fn intersects(&self, ray: &Ray) -> f32 {

        let accuracy: u32 = 5;

        if Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point)) > 0.0 { // Check if the plane is facing the camera
            return -1.0;
        }

        let denominator = Vec3::dot(&self.N, &ray.vector);
        // println!("Den {}", denominator);
        // println!("Normal {} {} {}", self.N.x, self.N.y, self.N.z);
        if denominator == 0.0 {return -1.0}

        let t: f32 = Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point))/denominator;

        let intercept: Vec3 = ray.get_point(t);

        // let v:Vec3 = Vec3::x_to_midpoint(&self.A, &self.B, &self.C);

        let AB: Vec3 = Vec3::sub(&self.B, &self.A);
        let CB: Vec3 = Vec3::sub(&self.B, &self.C);
        let AI: Vec3 = Vec3::sub(&intercept, &self.A);
        let a: f32 = round(Vec3::bary(&AB, &CB, &AI), accuracy);

        let BC: Vec3 = Vec3::sub(&self.C, &self.B);
        let AC: Vec3 = Vec3::sub(&self.C, &self.A);
        let BI: Vec3 = Vec3::sub(&intercept, &self.B);
        let b: f32 = round(Vec3::bary(&BC, &AC, &BI), accuracy);

        let CA: Vec3 = Vec3::sub(&self.A, &self.C);
        let BA: Vec3 = Vec3::sub(&self.A, &self.B);
        let CI: Vec3 = Vec3::sub(&intercept, &self.C);
        let c: f32 = round(Vec3::bary(&CA, &BA, &CI), accuracy);

        // println!("{} {} {} {}", a,b,c,a+b+c);
        let angle = Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point));
        if a >= 0.0 && b >= 0.0 && c >= 0.0 && a+b+c <= 1.0 { // determine if it is inside the triangle using barycentric coordinates
            // println!("{}",Vec3::sub(&ray.point, &intercept).len());
            return Vec3::sub(&ray.point, &intercept).len();
        }
        return -1.0;
    }
    pub fn rotate_xy(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_xy(angle, origin.clone());
        self.B.rotate_xy(angle, origin.clone());
        self.C.rotate_xy(angle, origin.clone());
        self.N = self.get_normal();
    }
    pub fn rotate_xz(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_xz(angle, origin.clone());
        self.B.rotate_xz(angle, origin.clone());
        self.C.rotate_xz(angle, origin.clone());
        self.N = self.get_normal();
    }
    pub fn rotate_yz(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_yz(angle, origin.clone());
        self.B.rotate_yz(angle, origin.clone());
        self.C.rotate_yz(angle, origin.clone());
        self.N = self.get_normal();
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
}

impl Square {
    pub fn new(top_left: Vec3, top_right: Vec3, bottom_left: Vec3, bottom_right: Vec3) -> Square {
        Square {
            A: Triangle::new(top_left.clone(), top_right, bottom_right.clone()),
            B: Triangle::new(top_left, bottom_right, bottom_left)
        }
    }
    pub fn intersects(&self, ray:&Ray) -> f32 {
        let a = self.A.intersects(&ray);
        let b = self.B.intersects(&ray);
        return (a+b)/2.0;
    }
    pub fn rotate_xy(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_xy(angle, origin.clone());
        self.B.rotate_xy(angle, origin.clone());
    }
    pub fn rotate_xz(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_xz(angle, origin.clone());
        self.B.rotate_xz(angle, origin.clone());
    }
    pub fn rotate_yz(&mut self, angle:f32, origin:Vec3) {
        self.A.rotate_yz(angle, origin.clone());
        self.B.rotate_yz(angle, origin.clone());
    }
}

impl Cube {
    pub fn new(position: Vec3, size: f32) -> Cube {
        Cube {
            size: size,
            position: position.clone(),
            front: Square::new(
                Vec3::add(&Vec3{x:-size,y:size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:size,z:-size}, &position), 
                Vec3::add(&Vec3{x:-size,y:-size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:-size}, &position), 
            ),
            back: Square::new(
                Vec3::add(&Vec3{x:size,y:size,z:size}, &position),
                Vec3::add(&Vec3{x:-size,y:size,z:size}, &position), 
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position),
                Vec3::add(&Vec3{x:-size,y:-size,z:size}, &position), 
            ),
            left: Square::new(
                Vec3::add(&Vec3{x:-size,y:size,z:size}, &position),
                Vec3::add(&Vec3{x:-size,y:size,z:-size}, &position), 
                Vec3::add(&Vec3{x:-size,y:-size,z:size}, &position),
                Vec3::add(&Vec3{x:-size,y:-size,z:-size}, &position), 
            ),
            right: Square::new(
                Vec3::add(&Vec3{x:size,y:size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:size,z:size}, &position), 
                Vec3::add(&Vec3{x:size,y:-size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position), 
            ),
            bottom: Square::new(
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position), //     - + +
                Vec3::add(&Vec3{x:-size,y:-size,z:size}, &position), //     + + +
                Vec3::add(&Vec3{x:size,y:-size,z:-size}, &position), // - + - 
                Vec3::add(&Vec3{x:-size,y:-size,z:-size}, &position), // + + -
            ),
            top: Square::new(
                Vec3::add(&Vec3{x:size,y:size,z:-size}, &position),
                Vec3::add(&Vec3{x:-size,y:size,z:-size}, &position), 
                Vec3::add(&Vec3{x:size,y:size,z:size}, &position),
                Vec3::add(&Vec3{x:-size,y:size,z:size}, &position), 
            ),
        }
    }
    pub fn get_faces(&self) -> [&Square; 6] {
        [&self.front, &self.back, &self.left, &self.right, &self.top, &self.bottom]
    }    
    pub fn intersects(&self, ray:&Ray) -> f32 {
        let mut z: f32 = -1.0;
        for i in self.get_faces() {
            let c = i.intersects(&ray);
            if c < z && c != -1.0 || z == -1.0 {
                z = c;
            }
        }
        return z;
    }
    pub fn rotate_xy(&mut self, angle:f32, origin:Vec3) {
        self.front.rotate_xy(angle, origin.clone());
        self.back.rotate_xy(angle, origin.clone());
        self.left.rotate_xy(angle, origin.clone());
        self.right.rotate_xy(angle, origin.clone());
        self.top.rotate_xy(angle, origin.clone());
        self.bottom.rotate_xy(angle, origin.clone())
    }
    pub fn rotate_xz(&mut self, angle:f32, origin:Vec3) {
        self.front.rotate_xz(angle, origin.clone());
        self.back.rotate_xz(angle, origin.clone());
        self.left.rotate_xz(angle, origin.clone());
        self.right.rotate_xz(angle, origin.clone());
        self.top.rotate_xz(angle, origin.clone());
        self.bottom.rotate_xz(angle, origin.clone())
    }
    pub fn rotate_yz(&mut self, angle:f32, origin:Vec3) {
        self.front.rotate_yz(angle, origin.clone());
        self.back.rotate_yz(angle, origin.clone());
        self.left.rotate_yz(angle, origin.clone());
        self.right.rotate_yz(angle, origin.clone());
        self.top.rotate_yz(angle, origin.clone());
        self.bottom.rotate_yz(angle, origin.clone())
    }
}

impl Camera {
    pub fn new(position: Vec3, FOV: f32, width: u32, height: u32, pixel_size: u32) -> Camera {
        let z: f32 = position.z+((width as f32/2.0)/((FOV as f32/2.0).tan()));
        Camera {
            position: position.clone(),
            FOV: FOV,
            pixel_size: pixel_size,
            screen: Surface {
                top_left: Vec3{x:position.x-(width as f32/2.0),y:position.y+(height as f32/2.0),z:z},
                top_right: Vec3{x:position.x+(width as f32/2.0),y:position.y+(height as f32/2.0),z:z},
                bottom_left: Vec3{x:position.x-(width as f32/2.0),y:position.y-(height as f32/2.0),z:z},
                bottom_right: Vec3{x:position.x+(width as f32/2.0),y:position.y-(height as f32/2.0),z:z}
            }
        }
    }
    pub fn get_pixel(&self, ox: u32, oy: u32, shapes: &Vec<Shape>) -> Pixel {
        let x: f32 = ox as f32 + self.screen.bottom_left.x;
        let y: f32 = oy as f32 + self.screen.bottom_left.y;
        
        let ray: Ray = Ray {
            point: Vec3{x: x,y: y,z: self.screen.bottom_left.z},
            vector: Vec3{x: x-self.position.x,y: y-self.position.y,z: self.screen.bottom_left.z-self.position.z}
        };
        let mut current_z: f32 = -1.0;
        for i in shapes.iter() {
            let c: f32 = i.intersects(&ray);
            if (c < current_z && c != -1.0) || current_z == -1.0 {current_z = c;}
        }
        // if current_z > 0.0 { println!("{}",current_z); }
        // if current_z > 60.0 {return Pixel { position: Vec2{x:ox as f32,y:oy as f32}, value: Color::GREY };}
        // if current_z > 0.0 {return Pixel { position: Vec2{x:ox as f32,y:oy as f32}, value: Color::WHITE };}
        Pixel { position: Vec2{x:ox as f32,y:oy as f32}, value: current_z }
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
