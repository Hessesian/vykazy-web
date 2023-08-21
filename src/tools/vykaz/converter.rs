use base64::encode;
use vykazy::{InputType, parse_text};

use crate::tools::vykaz::vykaz_component::Interactor;

#[derive(Default)]
pub struct ConverterInteractor {}

pub struct Args<'a> {
    pub file_type: &'a str,
    pub data: &'a [u8],
}

impl<'args> Interactor<Args<'args>, String> for ConverterInteractor {
    fn execute(&self, args: Args) -> String {
        match args.file_type.contains("pdf") {
            true => encode(parse_text(InputType::MEM(args.data)).unwrap()),
            false => encode(&args.data),
        }
    }
}
