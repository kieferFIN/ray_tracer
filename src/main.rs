use ray_tracer as rt;

fn main() {
    let mut timer = rt::Timer::new();
    timer.add();
    let t =rt::read_obj_file("cornell.obj").unwrap();
    timer.add();
    let wb = rt::WorldBuilder::from_triangles(t);
    /*let mut wb = rt::WorldBuilder::new();
    let ball = rt::TestBall{c:rt::Color{r:1.0, g:0.0,b:0.0},r:1.2, o: rt::Vector::new(0.5, 0.0, 0.0)};
    let ball2 = rt::TestBall{c:rt::Color{r:0.0,b:1.0,g:0.0},r:1.0, o: rt::Vector::new(-0.5, 0.0, 0.0)};
    wb.add_entity(ball);
    wb.add_entity(ball2);*/
    let world = wb.build();
    timer.add();
    let mut cam_conf = rt::CameraBuilder::new();
    cam_conf.orig = rt::Vector::new(0.0,0.0,-3.0);
    cam_conf.look_at(rt::Vector::new(0.0,0.1,0.0));
    cam_conf.super_sampling_factor=2;
    cam_conf.rays_per_pixel=4;
    let cam =  cam_conf.build();
    timer.add();
    let p = cam.take_pic(&world);
    timer.add();
    p.save("test.png").unwrap();
    timer.add();
    println!("Finish");
    timer.print();
}
