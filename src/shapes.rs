#![allow(dead_code)]
#![allow(non_snake_case)]

pub enum Shape {
    Triangle(Triangle),
    Square(Square),
    Cube(Cube)
}

impl Shape {
    pub fn intersects(&self, ray:&Ray) -> bool {
        match self {
            Shape::Triangle(x) => {return x.intersects(ray)},
            Shape::Cube(x) => {return x.intersects(ray)},
            Shape::Square(x) => {return x.intersects(ray)},
            _ => println!("A shape is not yet implemented")
        }
        false
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
    pub value: bool
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
    pub fn x_to_midpoint(A: &Vec3, B: &Vec3, C:&Vec3) -> Vec3 {
        let mid = Vec3::midpoint(B, C);
        Vec3::sub(&mid, A)
    }
    pub fn midpoint(A: &Vec3, B: &Vec3) -> Vec3{
        Vec3{x: (A.x+B.x)/2.0, y: (A.y+B.y)/2.0, z: (A.z+B.z)/2.0}
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
    pub fn intersects(&self, ray: &Ray) -> bool {

        if Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point)) < 0.0 {
            return false;
        }

        let denominator = Vec3::dot(&self.N, &ray.vector);
        // println!("Den {}", denominator);
        // println!("Normal {} {} {}", self.N.x, self.N.y, self.N.z);
        if denominator == 0.0 {return false;}

        let t: f32 = Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point))/denominator;

        let intercept: Vec3 = ray.get_point(t);

        let AV:Vec3 = Vec3::x_to_midpoint(&self.A, &self.B, &self.C);
        // println!("V: {} {} {}", AV.x, AV.y, AV.z);

        let a =    Vec3::dot(&AV, &Vec3::sub(&intercept, &self.A))
                        /
                        Vec3::dot(&AV, &Vec3::sub(&self.B, &self.A));

        if a >= 0.0 && a < 1.0 { // determine if it is inside the triangle using barycentric coordinates
            return true;
        }
        false
    }
}


impl Square {
    pub fn new(top_left: Vec3, top_right: Vec3, bottom_left: Vec3, bottom_right: Vec3) -> Square {
        Square {
            A: Triangle::new(top_left.clone(), top_right, bottom_right.clone()),
            B: Triangle::new(top_left, bottom_right, bottom_left)
        }
    }
    fn intersects(&self, ray:&Ray) -> bool {
        if self.A.intersects(&ray) || self.B.intersects(&ray) {
            return true;
        }
        false
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
                Vec3::add(&Vec3{x:-size,y:size,z:size}, &position),
                Vec3::add(&Vec3{x:size,y:size,z:size}, &position), 
                Vec3::add(&Vec3{x:-size,y:-size,z:size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position), 
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
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:-size}, &position), 
            ),
            top: Square::new(
                Vec3::add(&Vec3{x:-size,y:size,z:size}, &position),
                Vec3::add(&Vec3{x:size,y:size,z:size}, &position), 
                Vec3::add(&Vec3{x:-size,y:size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:size,z:-size}, &position), 
            ),
            bottom: Square::new(
                Vec3::add(&Vec3{x:-size,y:-size,z:-size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:-size}, &position), 
                Vec3::add(&Vec3{x:-size,y:-size,z:size}, &position),
                Vec3::add(&Vec3{x:size,y:-size,z:size}, &position), 
            ),
        }
    }
    pub fn get_faces(&self) -> [&Square; 6] {
        [&self.front, &self.back, &self.left, &self.right, &self.top, &self.bottom]
    }    
    fn intersects(&self, ray:&Ray) -> bool {
        for i in self.get_faces() {
            if i.intersects(&ray) {
                return true;
            }
        }
        false
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

        for i in shapes.iter() {
            if i.intersects(&ray) {
                return Pixel {
                    position: Vec2 {x: ox as f32, y: oy as f32},
                    value: true
                }
            }
        }
        
        Pixel { position: Vec2{x:ox as f32,y:oy as f32}, value: false }
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


