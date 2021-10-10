use std::io::{Cursor, Read};

use zip::ZipArchive;
use quick_xml::de::{from_str};
use serde::Deserialize;

use crate::error::Error;
use crate::units::Units;
use crate::xml_parse::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Model {
    resources: Resources,
    pub unit: Units
}

impl Model {
    #[allow(dead_code)]
    fn name(&self) -> &str {
        return self.resources.objects[0].name.as_str()
    }

    #[allow(dead_code)]
    fn vertices(&self) -> &[Vertex] {
        return self.resources.objects[0].mesh.vertices.data.as_slice()
    }

    #[allow(dead_code)]
    fn triangles(&self) -> &[Triangle] {
        return self.resources.objects[0].mesh.triangles.data.as_slice()
    }
}

impl Model {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Model { resources: Resources::default(), unit: Units::Unknown }
    }

    #[allow(dead_code)]
    pub fn from_raw_data(data: &[u8]) -> Result<Self, Error> {
        let curs = Cursor::new(data);
        let mut archive = ZipArchive::new(curs)?;

        // Go through the files within the archive and grab the first .model
        let mut model_file_name: String = String::new();

        for name in archive.file_names() {
            if name.contains(".model") {
                model_file_name = String::from(name);
            }
        }

        if model_file_name.is_empty() {
            return Err(Error::EmptyModelError)
        }

        // Grab our zip file
        let mut zip_file = archive.by_name(model_file_name.as_str())?;

        let mut string_data: String = String::new();
        zip_file.read_to_string(&mut string_data)?;


        let model: Model = from_str(string_data.as_str())?;

        println!("{:?}", model);

        Ok(model)
    }
}

#[test]
fn create_model() {
    let model = Model::empty();
    assert_eq!(Model { resources: Resources::default(), unit: Units::Unknown }, model)
}