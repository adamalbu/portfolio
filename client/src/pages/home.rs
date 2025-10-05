use crate::Route;
use crate::components::{LineInput, ProjectsList};

use gloo_net::http::Request;
use portfolio_common::Project;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

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

    let navigator = use_navigator().unwrap();
    let go_to_login = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&Route::Login))
    };

    let logout_on_click = {
        Callback::from(move |_: MouseEvent| {
            spawn_local(async move {
                let _ = Request::post("/api/logout").send().await;

                web_sys::window().unwrap().location().reload().unwrap();
            });
        })
    };

    let is_admin = use_state(|| false);
    {
        let is_admin = is_admin.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let response = Request::get("/api/auth-status").send().await;
                if let Ok(resp) = response {
                    is_admin.set(resp.ok());
                };
            });
        })
    }

    html! {
        <>
            <div class="flex flex-row place-content-between w-[calc(100%-4rem)] mx-auto">
                <h1 class="text-3xl text-center font-medium font-mono">{ TITLE_NAME }</h1>
                if !*is_admin {
                <button
                    onclick={go_to_login} >
                    { "Log in" }
                </button>
                }
                if *is_admin {
                    <button onclick={logout_on_click}>{ "Log out" }</button>
                }
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
