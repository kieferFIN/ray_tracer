

use ray_tracer as rt;

fn main() {
    let mut gonf = rt::CameraBuilder::new();
    gonf.size = (200,300);
    gonf.dir.normalize_mut();
    let cam =  rt::CameraBuilder::new().build();
    let world = rt::WorldBuilder::new().build();
    let p = cam.take_pic(&world);
    p.save("test.png").unwrap();
    println!("Finish");
}
