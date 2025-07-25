use gloo_net::http::Request;
use yew::platform::spawn_local;
use yew::prelude::*;

use components::line_input::LineInput;
use components::project_cards::ProjectsList;
use portfolio_common::Project;

mod components;

#[function_component(App)]
fn app() -> Html {
    let projects = use_state(|| vec![]);

    {
        let projects = projects.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let fetched_projects: Vec<Project> = Request::get("/api/projects")
                    .send()
                    .await
                    .expect("Failed to fetch projects")
                    .json()
                    .await
                    .expect("Failed to parse JSON");
                projects.set(fetched_projects);
            });
        });
    }

    let filter = use_state(String::new);

    let filter_projects = {
        let filter = filter.clone();
        Callback::from(move |text: String| {
            filter.set(text);
        })
    };

    html! {
        <>
        <h1 class="text-3xl text-center font-medium font-mono">{ "adamprojects.net" }</h1>
        <hr class="larger"/>
        <div class="font-mono mb-2">
            <span class="text-green-500">{ "adamalbu@adamprojects" }</span>
            { ":" }
            <span class="text-blue-500">{ "projects" }</span>
            { "$ ls | grep -F \"" }
            <LineInput oninput={filter_projects} />
            {"\""}
        </div>
        <ProjectsList projects={ (*projects).clone() } filter={ (*filter).clone() } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
