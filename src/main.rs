

use ray_tracer as rt;


fn main() {
    let t =rt::read_obj_file("cornell.obj").unwrap();
    println!("file read.");
    let wb = rt::WorldBuilder::from_entities(t);
    /*let mut wb = rt::WorldBuilder::new();
    let ball = rt::TestBall{c:rt::Color{r:1.0, g:0.0,b:0.0},r:1.2, o: rt::Vector::new(0.5, 0.0, 0.0)};
    let ball2 = rt::TestBall{c:rt::Color{r:0.0,b:1.0,g:0.0},r:1.0, o: rt::Vector::new(-0.5, 0.0, 0.0)};
    wb.add_entity(ball);
    wb.add_entity(ball2);*/
    let world = wb.build();
    println!("World created");
    let mut cam_conf = rt::CameraBuilder::new();
    cam_conf.orig = rt::Vector::new(0.0,0.0,3.0);
    cam_conf.look_at(rt::Vector::new(0.0,0.0,0.0));
    let cam =  cam_conf.build();
    println!("Camera created.");
    let p = cam.take_pic(&world);
    println!("Picture taken.");
    p.save("test.png").unwrap();
    println!("Finish");
}
