use crate::triangle::Triangle;
use crate::vertex::Vertex;
use crate::xml_parse::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    #[serde(rename = "vertices", with = "Vertices", default)]
    pub vertices: Vec<Vertex>,
    #[serde(rename = "triangles", with = "Triangles", default)]
    pub triangles: Vec<Triangle>,
}
