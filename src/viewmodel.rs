use std::collections::HashMap;
use std::sync::Arc;

use gloo::file::File;
use gloo_file::callbacks::FileReader;
use yew::Callback;

use crate::{FileDetails, Interactor, Msg};
use crate::converter::Args;

pub struct MainScreenViewModel<'a> {
    pub state: State,
    pub interactor: Box<dyn for <'b> Interactor<'a, Args<'b>, String>>,
}
#[derive(Default)]
pub struct State {
    readers: HashMap<String, FileReader>,
    pub files: Vec<FileDetails>,
}

impl <'a> MainScreenViewModel<'a> {
    pub(crate) fn push(&mut self, file_name: String, file_type: String, data: Vec<u8>) {
        let data = self.interactor.execute(Args{ file_type: &file_type, data: &data });
        self.state.files.push(FileDetails {
            data,
            file_type,
            name: file_name.clone(),
        });
        self.state.readers.remove(&file_name);
    }

    pub(crate) fn update(&mut self, files: Vec<File>, callback: Callback<Msg>) {
        for file in files.into_iter() {
            let file_name = file.name();
            let file_type = file.raw_mime_type();

            let task = {
                let file_name = file_name.clone();

                let callback = callback.clone();
                gloo::file::callbacks::read_as_bytes(&file, move |res| {
                    callback.emit(Msg::Loaded(
                        file_name,
                        file_type,
                        res.expect("failed to read file"),
                    ))
                })
            };
            self.state.readers.insert(file_name, task);
        }
    }
}
