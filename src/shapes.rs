#![allow(dead_code)]
#![allow(non_snake_case)]


pub struct Vec2 {
    x: f32,
    y: f32
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Ray {
    pub point: Vec3,
    pub vector: Vec3
}


pub struct Camera {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub pixel_size: i32
}

pub struct Triangle {
    pub A: Vec3, // Point A
    pub B: Vec3, // Point B
    pub C: Vec3, // Point C
    pub N: Vec3,  // Normal Vector
    pub center: Vec3
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
        (self.x*self.x+self.y*self.y+self.z*self.z)*0.5 // Return the length of vector
    }
    pub fn normalize(&mut self) {
        self.x = self.x/self.len();
        self.y = self.y/self.len();
        self.z = self.z/self.len();
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
}

impl Triangle { // A, B, C, Normal, Center
    pub fn new(A: Vec3, B : Vec3, C: Vec3) -> Triangle {
        let center = Vec3 {   x: (A.x + B.x + C.x)/3.0,
                                    y: (A.y + B.y + C.y)/3.0,
                                    z: (A.z + B.z + C.z)/3.0};
        let mut normal: Vec3 = Vec3 {   x: A.y*B.z-A.z*B.y,
                                    y: A.z*B.x-A.x*B.z,
                                    z: A.x*B.y-A.y*B.x};
        normal.normalize();

        Triangle {
            A: A, B: B, C: C,
            N: normal,
            center: center
        }
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        // point used for formula will be self.A
        // Normal.x(x - A.x) + Normal.y(y - A.y) + Normal.z(z - A.z) = 0
        // Normal.x(-A.x) + Normal.y(-A.y) + Normal.z(-A.z) + Normal.x(ray.point.x) + Normal.y(ray.point.y) + Normal.z(ray.point.z) = 
        // t = (Nx(X-Xi)+Ny(Y-Yi)+Nz(Z-Zi))/(Nx*Ax+Ny*Ay+Nz*Az)
        let denominator = Vec3::dot(&self.N, &ray.vector);
        println!("Normal {} {} {}", self.N.x, self.N.y, self.N.z);
        println!("Den {}", denominator);
        if denominator == 0.0 {
            return false;
        }
        let t: f32 = Vec3::dot(&self.N, &Vec3::sub(&self.A, &ray.point))/denominator;

        let intercept: Vec3 = Vec3 {
            x: ray.point.x + (ray.vector.x*t),
            y: ray.point.y + (ray.vector.y*t),
            z: ray.point.z + (ray.vector.z*t)
        };
        let AV:Vec3 = Vec3 {
            x: (self.B.x+self.C.x)/2.0,
            y: (self.B.y+self.C.y)/2.0,
            z: (self.B.z+self.C.z)/2.0};
        let BV:Vec3 = Vec3 {
            x: (self.A.x+self.C.x)/2.0,
            y: (self.A.y+self.C.y)/2.0,
            z: (self.A.z+self.C.z)/2.0};
        let CV:Vec3 = Vec3 {
            x: (self.A.x+self.B.x)/2.0,
            y: (self.A.y+self.B.y)/2.0,
            z: (self.A.z+self.B.z)/2.0};

        let a: f32 = Vec3::dot( // Calculate a
            &AV, 
            &Vec3 {
            x: intercept.x - self.A.x,
            y: intercept.y - self.A.y,
            z: intercept.z - self.A.z}
        )/Vec3::dot(
            &AV,
            &Vec3 {
                x: self.B.x - self.A.x,
                y: self.B.y - self.A.y,
                z: self.B.z - self.A.z
            }
        );
        let b: f32 = Vec3::dot( // Calculate b
            &BV, 
            &Vec3 {
            x: intercept.x - self.B.x,
            y: intercept.y - self.B.y,
            z: intercept.z - self.B.z}
        )/Vec3::dot(
            &BV,
            &Vec3 {
                x: self.C.x - self.B.x,
                y: self.C.y - self.B.y,
                z: self.C.z - self.B.z
            }
        );
        let c: f32 = Vec3::dot( // Calculate c
            &CV, 
            &Vec3 {
            x: intercept.x - self.C.x,
            y: intercept.y - self.C.y,
            z: intercept.z - self.C.z})/Vec3::dot(
            &CV,
            &Vec3 {
                x: self.A.x - self.C.x,
                y: self.A.y - self.C.y,
                z: self.A.z - self.C.z
            }
        );
        println!("a {}",a);
        if a >= 0.0 && b >= 0.0 && c >= 0.0 && a+b+c < 1.0 { // determine if it is inside the triangle using barycentric coordinates
            return true;
        }
        false
    }
}


