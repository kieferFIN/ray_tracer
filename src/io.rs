use crate::world::entities::TriangleBuilder as TrB;
use crate::world::entities::Triangle;
use crate::types::Color;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;

pub fn read_obj_file(file_name: &str) -> Result<Vec<Triangle>, Box<dyn Error>> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Triangle> = Vec::new();

    let mut cur_color: Color = Color { r: 0.3, g: 0.3, b: 0.3 };
    let mut materials = HashMap::new();


    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0] {
            "v" => vertices.push([tokens[1].parse()?, tokens[2].parse()?, tokens[3].parse()?]),
            "vn" => normals.push([tokens[1].parse()?, tokens[2].parse()?, tokens[3].parse()?]),
            "f" => {
                let mut b = TrB::new(cur_color);
                for i in 1..4 {
                    let [idv, _, idn] = parse_face(tokens[i]);
                    b.add(vertices[idv], normals[idn]);
                }
                faces.push(b.build());
            }
            "usemtl" => {
                let material_name = tokens[1];
                cur_color = *materials.get(material_name).unwrap();
            }
            "mtllib" => parse_mtl_file(tokens[1],&mut materials),
            _ => continue
        };
    }
    //println!("{} {}",faces[0].a_b, faces[0].a_c);
    Ok(faces)
}

fn parse_face(tokens: &str) -> [usize; 3] {
    let a: Vec<usize> = tokens.split("/").map(|s| s.parse().unwrap()).collect();
    [a[0] - 1, a[1] - 1, a[2] - 1]
}

fn parse_mtl_file(file_name: &str, materials: &mut HashMap<String,Color>) {
    if let Ok(f) = File::open(file_name) {
        let reader = BufReader::new(f);
        let mut cur_material_name: Option<String> = None;
        let mut cur_material: Color = Color::gray(0.0);

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let tokens: Vec<&str> = line.split(' ').collect();
            match tokens[0] {
                "newmtl" =>
                    {
                        if let Some(name) = cur_material_name {
                            materials.insert(name, cur_material);
                        }
                        cur_material_name = Some(tokens[1].to_string());
                    }
                "Kd" => cur_material = Color { r: tokens[1].parse().unwrap(), g: tokens[2].parse().unwrap(), b: tokens[3].parse().unwrap() },
                _ => ()
            }
        }
        if let Some(name) = cur_material_name {
            materials.insert(name, cur_material);
        }
    }
}