use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertices {
    #[serde(rename="vertex", default)]
    pub data: Vec<Vertex>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Triangles {
    #[serde(rename="triangle", default)]
    pub data: Vec<Triangle>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    pub vertices: Vertices,
    pub triangles: Triangles
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
    pub id: usize,
    pub name: String,
    #[serde(rename="type", default)]
    pub otype: String,
    pub mesh: Mesh
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Resources {
    #[serde(rename="object", default)]
    pub objects: Vec<Object>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct XMLModel {
    pub resources: Resources,
    pub unit: String
}