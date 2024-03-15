use shapes::{Ray, Triangle, Vec3};

mod shapes;

fn main() {
    let face: Triangle = Triangle::new(
        Vec3{x:-10.0,y:-10.0,z:1.0},
        Vec3{x:-10.0,y:10.0,z:1.0},
        Vec3{x:10.0,y:10.0,z:1.0}
    );
    let ray: Ray = Ray { point: Vec3{x:0.0,y:0.0,z:0.0}, vector: Vec3{x:0.0,y:0.0,z:1.0}};
    println!("{}", face.intersects(ray));
}
