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
    let world = config.create_world::<rt::world::BasicCreator<Triangle>>();
    timer.add("World 1 creation");
    let world2 = config.create_world::<rt::world::SimpleBVHCreator<Triangle>>();
    timer.add("World 2 creation");
    let cam = config.create_camera();
    timer.add("Camera creation");
    let p = cam.take_pic(&world);
    timer.add("Pic 1 rendering");
    let p2 = cam.take_pic(&world2);
    timer.add("Pic 2 rendering");
    p.save("test.png").unwrap();
    timer.add("Pic 1 saving");
    p2.save("test2.png").unwrap();
    timer.add("Pic 2 saving");
    println!("Finish");
    timer.print();
}
