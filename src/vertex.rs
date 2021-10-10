use serde::Deserialize;
use std::ops;

// Can also describe a vector
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Vertex {
        self / self.len()
    }
}

impl ops::Add<Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f32) -> Self::Output {
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vertex {
    type Output = Vertex;

    fn div(self, rhs: f32) -> Self::Output {
        Vertex {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<f32> for &Vertex {
    type Output = Vertex;

    fn div(self, rhs: f32) -> Self::Output {
        Vertex {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
