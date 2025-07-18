use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ProjectCardProps {
    name: String,
    description: String,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <div class="rounded-md w-55 p-4 bg-white hover:z-10 transition filter drop-shadow-lg hover:drop-shadow-2xl hover:scale-105">
            <h1 class="text-center font-medium text-lg">{ &props.name }</h1>
            <hr />
            <p class="text-left">{ &props.description }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <>
        <h1 class="text-3xl text-center font-medium font-mono">{ "adamprojects.net" }</h1>
        <hr class="larger"/>
        <div class="flex space-x-4">
            <ProjectCard name="Project" description="Lorem ipsum" />
            <ProjectCard name="Longer Project" description="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." />
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
