mod model;
mod error;
mod units;
mod xml_parse;

#[cfg(test)]
mod tests {
    use crate::model::Model;

    #[test]
    fn test_shapr_single() {
        let data: &'static [u8] = include_bytes!("test.3mf");
        match Model::from_raw_data(data) {
            Ok(d) => {
                println!("{:#?}", d);
                assert!(true)
            }
            Err(e) => {
                println!("{:#?}",e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_fusion_single() {
        let data: &'static [u8] = include_bytes!("Frontplate.3mf");
        match Model::from_raw_data(data) {
            Ok(d) => {
                println!("{:#?}", d);
                assert!(true)
            }
            Err(e) => {
                println!("{:#?}",e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_fusion_multi() {
        let data: &'static [u8] = include_bytes!("test_fusion_multi.3mf");
        match Model::from_raw_data(data) {
            Ok(d) => {
                //println!("{:#?}", d);
                assert!(true)
            }
            Err(e) => {
                println!("{:#?}",e);
                assert!(false)
            }
        }
    }
}