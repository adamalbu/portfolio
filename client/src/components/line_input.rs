use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LineInputProps {
    pub oninput: Callback<String>,
}

#[function_component(LineInput)]
pub fn line_input(props: &LineInputProps) -> Html {
    let oninput = {
        let parent_oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input_element = e.target_dyn_into::<web_sys::HtmlElement>();

            if let Some(element) = input_element {
                let content = element.inner_text();

                if content.contains("\n") {
                    element.set_inner_text(&content.replace("\n", ""));
                }
                parent_oninput.emit(content)
            };
        })
    };

    html! {
        <span class="min-w-4 inline-block align-bottom" contenteditable=true {oninput}></span>
    }
}
