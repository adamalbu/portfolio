use yew::prelude::*;

mod components;
use components::project_cards::{ Project, ProjectsList };
use components::line_input::LineInput;

#[function_component(App)]
fn app() -> Html {
    let projects: Vec<Project> = vec![
        Project::new("Project".into(), "Lorem ipsum".into()),
        Project::new("Longer Project".into(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".into()),
    ];
    
    let filter = use_state(String::new);

    let filter_projects = {
        let filter = filter.clone();
        Callback::from(move |text: String| {
            filter.set(text);
        })
    };

    html!{
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
        <ProjectsList projects={ projects } filter={ (*filter).clone() } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
