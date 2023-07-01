use ray_tracer as rt;
use ray_tracer::world::entities::Triangle;
use ray_tracer::Config;
use std::fs;

fn main() {
    let mut timer = rt::Timer::new();
    timer.add("");
    let config_str = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    timer.add("Config");
    let world = config.create_world::<rt::world::SimpleBVHCreator<Triangle>>();
    timer.add("World creation");
    let cam = config.create_camera();
    timer.add("Camera creation");
    let p = cam.take_pic(&world);
    timer.add("Pic rendering");
    p.save("test.png").unwrap();
    timer.add("Pic saving");
    println!("Finish");
    timer.print();
}
