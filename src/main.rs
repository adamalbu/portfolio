use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, Clone, PartialEq)]
struct ProjectCardProps {
    name: AttrValue,
    description: AttrValue,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <div class="rounded-md w-55 p-4 bg-white hover:z-10 transition filter drop-shadow-lg hover:drop-shadow-2xl hover:scale-105">
            <h1 class="text-center font-medium text-lg">{ &*props.name }</h1>
            <hr />
            <p class="text-left">{ &*props.description }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectCardsListProps {
    projects: Vec<ProjectCardProps>,
    #[prop_or("".into())]
    filter: AttrValue,
}

#[function_component(ProjectsList)]
fn project_cards_list(props: &ProjectCardsListProps) -> Html {
    let filtered_projects = props.projects.iter().filter(|project| {
        let filter_text = &props.filter.as_str();
        if filter_text.chars().any(|c| !c.is_whitespace()) {
            project.name.to_lowercase().contains(&filter_text.to_lowercase())
        } else {
            true
        }
    });

    html! {
        <div class="flex space-x-4">
            { filtered_projects.map(|project| html! {
                <ProjectCard name={project.name.clone()} description={project.description.clone()} />
            }).collect::<Html>() }
        </div>
    }
} 

#[derive(Properties, PartialEq)]
struct LineInputProps {
    oninput: Callback<String>,
}

#[function_component(LineInput)]
fn line_input(props: &LineInputProps) -> Html {
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

#[function_component(App)]
fn app() -> Html {
    let projects = vec![
        ProjectCardProps{ name: "Project".into(), description: "Lorem ipsum".into()},
        ProjectCardProps{ name: "Longer Project".into(), description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".into()},
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
