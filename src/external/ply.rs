use std::sync::Arc;
use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, Property};
use std::fs::File;
use std::io::BufReader;
use indicatif::{ProgressBar, ProgressStyle};
use crate::materials::Material;
use crate::primitives::*;
use crate::hittable::hittable_list::HittableList;

pub fn load_ply(file_path: &str, mat: Arc<Material>) -> HittableList {
    let mut world = HittableList::new();

    let file = File::open(file_path).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let parser = Parser::<DefaultElement>::new();
    let ply = parser.read_ply(&mut buf_reader).expect("Failed to read PLY file");
    let vertex_elements = &ply.payload["vertex"];
    let face_elements = &ply.payload["face"];

    let mut vertices = Vec::new();
    let vertex_bar = ProgressBar::new(vertex_elements.len() as u64);
    vertex_bar.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("=>-"));

    for vertex in vertex_elements {
        let x = match vertex.get("x").unwrap() {
            Property::Float(val) => *val as f64,
            Property::Double(val) => *val,
            _ => panic!("Unexpected type for vertex x coordinate"),
        };
        let y = match vertex.get("y").unwrap() {
            Property::Float(val) => *val as f64,
            Property::Double(val) => *val,
            _ => panic!("Unexpected type for vertex y coordinate"),
        };
        let z = match vertex.get("z").unwrap() {
            Property::Float(val) => *val as f64,
            Property::Double(val) => *val,
            _ => panic!("Unexpected type for vertex z coordinate"),
        };
        vertices.push(Point3::new(x, y, z));
        vertex_bar.inc(1);
    }
    vertex_bar.finish_with_message("Vertices processed");

    let face_bar = ProgressBar::new(face_elements.len() as u64);
    face_bar.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{wide_bar:.green/yellow}] {pos}/{len} ({eta})")
        .progress_chars("=>-"));

    for face in face_elements {
        let vertex_indices = match face.get("vertex_indices").unwrap() {
            Property::ListInt(val) => val.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            Property::ListUInt(val) => val.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            _ => panic!("Unexpected type for face vertex indices"),
        };
        let i1 = vertex_indices[0];
        let i2 = vertex_indices[1];
        let i3 = vertex_indices[2];

        let v0 = vertices[i1];
        let v1 = vertices[i2];
        let v2 = vertices[i3];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        world.add(Arc::new(Triangle::new(v0, edge1, edge2, mat.clone())));
        face_bar.inc(1);
    }
    face_bar.finish_with_message("Faces processed");

    world
}
