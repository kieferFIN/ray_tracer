

use ray_tracer as rt;

struct Ball {
    c:rt::Color,
    r: f64,
    o: rt::Vector
}

impl rt::Entity for Ball{
    fn hit(&self, ray: &rt::Ray) -> Option<(f64,rt::Color)> {
        let a = ray.orig - self.o;
        let b = 2.0*ray.dir.dot(&a);
        let d = b.powi(2) - 4.0 * (a.dot(&a)-self.r.powi(2)) ;
        if d > 0.0 {
            let t = 0.0_f64.max((-b +d)*0.5).min(0.0_f64.max((-b -d)*0.5));
            if t>0.0{
                return Some((t ,self.c));
            }
        }
        None

    }
}

fn main() {
    //let mut camConf = rt::CameraBuilder::new();
    let cam =  rt::CameraBuilder::new().build();
    let mut wb = rt::WorldBuilder::new();
    let ball = Ball{c:rt::Color{data:[200,0,0]},r:2.0, o: rt::Vector::new(1.0, 0.0, -8.0)};
    let ball2 = Ball{c:rt::Color{data:[0,0,200]},r:2.0, o: rt::Vector::new(-0.5, 0.0, -8.0)};
    wb.entities.push(Box::new(ball));
    wb.entities.push(Box::new(ball2));
    let world = wb.build();
    let p = cam.take_pic(&world);
    p.save("test.png").unwrap();
    println!("Finish");
}
