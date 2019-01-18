use crate::world::entities::Triangle as Tr;
use crate::world::entities::TriangleBuilder as TrB;
use crate::types::Color;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;

pub fn read_obj_file(file_name: &str)->Result<(Vec<Tr>),Box<dyn Error>>{
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    let mut vertices :Vec<[f64;3]> = Vec::new();
    let mut normals :Vec<[f64;3]> = Vec::new();
    let mut faces: Vec<Tr> = Vec::new();

    let mut cur_color: Color = Color{r:0.3,g:0.3,b:0.3};


    for line_result in reader.lines(){
        let line = line_result.unwrap();
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0]  {
            "v" => vertices.push([tokens[1].parse()?,tokens[2].parse()?,tokens[3].parse()?]),
            "vn" => normals.push([tokens[1].parse()?,tokens[2].parse()?,tokens[3].parse()?]),
            "f" => { let mut b = TrB::new(cur_color);
                for i in 1..4 {
                    let [idv,_,idn] = parse_face(tokens[i]);
                    b.add(vertices[idv],normals[idn]);
                }
                faces.push(b.build())
            },
            "usemtl" => {let t = tokens[1].parse::<u8>().unwrap(); cur_color.set(t);}
            _ => continue
        };
    }
    //println!("{} {}",faces[0].a_b, faces[0].a_c);
    Ok(faces)
}

fn parse_face(tokens: &str)->[usize;3]{
    let a:Vec<usize> = tokens.split("/").map(|s| s.parse().unwrap()).collect();
    [a[0]-1,a[1]-1,a[2]-1]
}