use crate::mesh::Mesh;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertices<T> {
    pub vertex: T,
}

// TODO: Put this into a wrapper macro
impl<T> Vertices<T> {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let wrapper = <Self as Deserialize>::deserialize(deserializer)?;
        Ok(wrapper.vertex)
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Triangles<T> {
    pub triangle: T,
}

// TODO: Put this into a wrapper macro
impl<T> Triangles<T> {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let wrapper = <Self as Deserialize>::deserialize(deserializer)?;
        Ok(wrapper.triangle)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type", default)]
    pub otype: String,
    pub mesh: Mesh,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resources<T> {
    pub object: T,
}

// TODO: Put this into a wrapper macro
// Model is defined outside of this file, so we need to pub deserialize
impl<T> Resources<T> {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let wrapper = <Self as Deserialize>::deserialize(deserializer)?;
        Ok(wrapper.object)
    }
}
