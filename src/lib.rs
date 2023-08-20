use std::sync::Arc;
use gloo_file::File;
use viewmodel::MainScreenViewModel;
use web_sys::FileList;
use yew::{Component, Context, Html};
use crate::converter::ConverterInteractor;

mod converter;
mod main_screen;
mod viewmodel;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: String,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct MainScreen<'a> {
    view_model: MainScreenViewModel<'a>,
}

pub trait Interactor<'a, Args, Result>{
    fn execute(&self, args: Args) -> Result;
}

impl Component for MainScreen<'static>{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            view_model: MainScreenViewModel{
                state: Default::default(),
                interactor: Box::new(ConverterInteractor{}),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                self.view_model.push(file_name, file_type, data);
                true
            }
            Msg::Files(files) => {
                let link = ctx.link().clone();
                self.view_model.update(files, link.callback(|msg| msg));
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        main_screen::view(self, ctx.link().clone())
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

impl <'b> MainScreen<'b> {
    fn view_file(file: &FileDetails) -> Html {
        main_screen::view_file(file)
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}
