use std::io::{BufReader, Cursor, Read, Seek};
use std::path::Path;

use quick_xml::de::from_reader;
use serde::Deserialize;
use zip::ZipArchive;

use crate::error::Error;
use crate::units::Units;
use crate::xml_parse::Object;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Model {
    pub unit: Units,
    #[serde(rename = "resources", with = "crate::xml_parse::Resources", default)]
    pub objects: Vec<Object>,
}

impl Model {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        return self.objects[0].name.as_str();
    }

    #[allow(dead_code)]
    pub fn num_objects(&self) -> usize {
        return self.objects.len();
    }

    #[allow(dead_code)]
    pub fn num_vertices(&self) -> usize {
        return self.objects.iter().map(|d| d.mesh.vertices.len()).sum();
    }

    #[allow(dead_code)]
    pub fn num_triangles(&self) -> usize {
        return self.objects.iter().map(|d| d.mesh.triangles.len()).sum();
    }
}

impl Model {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Model {
            objects: Vec::new(),
            unit: Units::Unknown,
        }
    }

    #[allow(dead_code)]
    fn new<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Self, Error> {
        // Go through the files within the archive and grab the first .model
        // TODO: There is a better way to do this
        let mut model_file_name: String = String::new();
        for name in archive.file_names() {
            if name.contains(".model") {
                model_file_name = String::from(name);
            }
        }

        // If we didn't find a model, return an error
        if model_file_name.is_empty() {
            return Err(Error::EmptyModelError);
        }

        // Grab our zip file
        let zip_file = archive.by_name(model_file_name.as_str())?;

        // Parse the zipfile (using Serde and quick_xml)
        let model: Model = from_reader(BufReader::new(zip_file))?;

        Ok(model)
    }

    #[allow(dead_code)]
    pub fn from_raw_data(data: &[u8]) -> Result<Self, Error> {
        let curs = Cursor::new(data);
        let archive = ZipArchive::new(curs)?;

        Ok(Self::new(archive)?)
    }

    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = std::fs::File::open(path)?;

        let archive = ZipArchive::new(file)?;

        Ok(Self::new(archive)?)
    }
}

#[test]
fn create_model() {
    let model = Model::empty();
    assert_eq!(
        Model {
            objects: Vec::new(),
            unit: Units::Unknown
        },
        model
    )
}
