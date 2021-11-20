use ray_tracer as rt;
use ray_tracer::Config;
use std::fs;

fn main() {
    let mut timer = rt::Timer::new();
    timer.add();
    let config_str = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    timer.add();
    let world = config.create_world::<rt::world::BasicCreator>();
    timer.add();
    let cam = config.create_camera();
    timer.add();
    let p = cam.take_pic(&world);
    timer.add();
    p.save("test.png").unwrap();
    timer.add();
    println!("Finish");
    timer.print();
}
