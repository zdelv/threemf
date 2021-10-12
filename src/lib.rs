pub mod error;
pub mod mesh;
pub mod model;
pub mod triangle;
pub mod units;
pub mod vertex;
pub mod xml_parse;

pub use crate::mesh::Mesh;
pub use crate::model::Model;
pub use crate::triangle::Triangle;
pub use crate::units::Units;
pub use crate::vertex::Vertex;

#[cfg(test)]
mod tests {
    use crate::model::Model;

    fn run_model_test(data: &'static [u8]) {
        match Model::from_raw_data(data) {
            Ok(d) => {
                println!("Objects: {}", d.num_objects());
                println!("Vertices: {}", d.num_vertices());
                println!("Triangles: {}", d.num_triangles());
                assert!(true)
            }
            Err(e) => {
                println!("{:#?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    // Very simple single part 3MF from Shapr3D
    fn test_shapr_single() {
        let data: &'static [u8] = include_bytes!("../data/test.3mf");
        run_model_test(data)
    }

    #[test]
    // Complicated single part 3MF from Fusion 360 (NEMA motor frontplate)
    fn test_fusion_single() {
        let data: &'static [u8] = include_bytes!("../data/Frontplate.3mf");
        run_model_test(data)
    }

    #[test]
    // Very large multi part 3MF from Fusion 360 (Full frame assembly, 80 objects)
    fn test_fusion_multi() {
        let data: &'static [u8] = include_bytes!("../data/test_fusion_multi.3mf");
        run_model_test(data)
    }

    #[test]
    fn test_from_file() {
        let path = "data/test.3mf";
        match Model::from_file(path) {
            Ok(d) => {
                println!("Objects: {}", d.num_objects());
                println!("Vertices: {}", d.num_vertices());
                println!("Triangles: {}", d.num_triangles());
                assert!(true)
            }
            Err(e) => {
                println!("{:#?}", e);
                assert!(false)
            }
        }
    }
}
