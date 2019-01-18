

use ray_tracer as rt;
use nalgebra::Matrix3 as M3;
use nalgebra::Vector3 as V3;



fn main() {
    //test();
    let t =rt::read_obj_file("cornell.obj").unwrap();
    println!("file read.");
    let mut cam_conf = rt::CameraBuilder::new();
    cam_conf.orig = rt::Vector::new(0.0,0.0,-3.0);
    cam_conf.look_at(rt::Vector::new(0.0,0.0,0.0));
    //cam_conf.size=(100,100);
    let cam =  cam_conf.build();
    println!("Camera created.");
    let mut wb = rt::WorldBuilder::new();
    /*let ball = rt::Ball{c:rt::Color{r:1.0, g:0.0,b:0.0},r:2.0, o: rt::Vector::new(1.0, 0.0, -7.0)};
    let ball2 = rt::Ball{c:rt::Color{r:0.0,b:1.0,g:0.0},r:2.0, o: rt::Vector::new(-0.5, 0.0, -8.0)};
    wb.entities.push(Box::new(ball));
    wb.entities.push(Box::new(ball2));*/
    for tri in t {
        wb.entities.push(Box::new(tri));
    }
    let world = wb.build();
    println!("World created");
    let p = cam.take_pic(&world);
    println!("Picture taken.");
    p.save("test.png").unwrap();
    println!("Finish");
}

fn test(){
    let m1 = M3::new(1,2,3,1,2,3,1,2,3);
    let v1 = V3::new(1,1,1);
    let v2 = V3::new(2,2,2);
    let v3 = V3::new(3,3,3);
    let m2 = M3::from_columns(&[v1,v2,v3]);

    let a1= m1*v1;
    let a2= m2*v1;
    println!("{:?} {:?}",a1,a2);
}
