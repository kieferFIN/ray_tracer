use nalgebra::Vector3 as V3;

use ray_tracer as rt;
fn main() {
    let cam =  rt::CamBuilder::new().size((300,300)).orig(V3::new(2.4,4.6,5.5)).build();
    let world = rt::World::new();
    let p = cam.take_pic(&world);
    p.save("test.png").unwrap();
    println!("Finish");
}
