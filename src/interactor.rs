use base64::encode;
use vykazy::{parse_text, InputType};

#[derive(Default)]
pub struct ConverterInteractor {}

impl ConverterInteractor {
    pub fn convert(&self, file_type: &str, data: &[u8]) -> String {
        match file_type.contains("pdf") {
            true => encode(parse_text(InputType::MEM(data)).unwrap()),
            false => encode(&data),
        }
    }
}
