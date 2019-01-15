

use ray_tracer as rt;

fn main() {
    let cam =  rt::CamBuilder::new().build();
    let world = rt::World::new();
    let p = cam.take_pic(&world);
    p.save("test.png").unwrap();
    println!("Finish");
}
