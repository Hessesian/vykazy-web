use web_sys::DragEvent;
use web_sys::Event;
use web_sys::HtmlInputElement;
use yew::Callback;
use yew::Context;
use yew::html;
use yew::Html;
use yew::html::Scope;
use yew::TargetCast;

use crate::FileDetails;

use super::MainScreen;

pub fn view<'a, 'b>(app: &'a MainScreen, ctx: Scope<MainScreen<'static>>) -> Html {
    return yew_template::template_html! {"templates/MainScreen.html",
                    ondrop={ctx.callback(|event: DragEvent| {
                        event.prevent_default();
                        let files = event.data_transfer().unwrap().files();
                        MainScreen::upload_files(files)
                    })},
                    ondragover={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })},
                    ondragenter={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })},
                    onchange={ctx.callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        MainScreen::upload_files(input.files())
                    })},
                    files= { app.view_model.state.files.iter().map(MainScreen::view_file).collect::<Html>() }

    }
}

pub fn view_file(file: &FileDetails) -> Html {
    html! {
        <>
        <div class="preview-tile">
            <p class="preview-name">{ format!("{}", file.name) }</p>
            <div class="preview-media">
                if file.file_type.contains("image") {
                    <img src={format!("data:{};base64,{}", file.file_type, file.data)} />
                } else if file.file_type.contains("video") {
                    <video controls={true}>
                        <source src={format!("data:{};base64,{}", file.file_type, file.data)} type={file.file_type.clone()}/>
                    </video>
                } else if file.file_type.contains("pdf") {
            <a href ={format!("data:{};base64,{}", "text/csv", file.data)}>
            {"Download"}
            </a>
        }
            </div>
        </div>
        </>
    }
}
