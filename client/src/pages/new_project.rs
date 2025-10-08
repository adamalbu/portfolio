use gloo_net::http::Request;
use web_sys::wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

use crate::Route;

#[function_component(NewProject)]
pub fn new_project() -> Html {
    let navigator = use_navigator().unwrap();

    let add_project_on_click = Callback::from(move |_: MouseEvent| {
        let document = web_sys::window().unwrap().document().unwrap();
        let title = document
            .get_element_by_id("title")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        let description = document
            .get_element_by_id("description")
            .unwrap()
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .unwrap()
            .value();

        let navigator = navigator.clone();
        spawn_local(async move {
            let project = portfolio_common::Project::new(title, description);

            let response = Request::post("/api/new_project")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&project).unwrap())
                .unwrap()
                .send()
                .await
                .unwrap();

            if response.status() == 200 {
                navigator.push(&Route::Home);
            }
        });
    });

    html! {
        <div class="rounded-sm items-center flex flex-col space-y-3 absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-fit p-10 shadow-xl/20 bg-bg-secondary text-text-dark">
            <div class="space-x-3">
                <label for="title">{ "Project title" }</label>
                <input type="text" id="title" name="title" required=true />
            </div>
            <div class="space-x-3 flex flex-row items-center">
                <label for="description">{ "Project description" }</label>
                <textarea id="description" name="description" required=true />
            </div>

            <button onclick={add_project_on_click} type="submit">{ "Add project" }</button>
        </div>
    }
}
