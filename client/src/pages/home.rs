use crate::components::{LineInput, ProjectsList};
use gloo_net::http::Request;
use portfolio_common::Project;
use yew::platform::spawn_local;
use yew::prelude::*;

const TITLE_NAME: &'static str = "website.com";

#[function_component(Home)]
pub fn home() -> Html {
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
            <div class="flex flex-row place-content-between w-[calc(100%-4rem)] mx-auto">
                <h1 class="text-3xl text-center font-medium font-mono">{ TITLE_NAME }</h1>
                <button class="rounded-md p-1 bg-linear-to-t from-neutral-300 to-white transition ition hover:from-neutral-400 hover:to-neutral-100 hover:cursor-pointer filter hover:drop-shadow-lg">{ "Log in" }</button>
            </div>
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
