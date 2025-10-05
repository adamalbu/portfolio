use portfolio_common::Project;
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
        <div class="rounded-md w-55 p-4 bg-bg-secondary hover:z-10 transition hover:scale-105">
            <h1 class="text-center text-text-dark font-medium text-lg">{ &*props.name }</h1>
            <hr />
            <p class="text-left text-text-dark">{ &*props.description }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectCardsListProps {
    pub projects: Vec<Project>,
    #[prop_or("".into())]
    pub filter: AttrValue,
    pub show_create_new: bool,
}

#[function_component(ProjectsList)]
pub fn project_cards_list(props: &ProjectCardsListProps) -> Html {
    let filtered_projects = props.projects.iter().filter(|project| {
        let filter_text = &props.filter.as_str();
        if filter_text.chars().any(|c| !c.is_whitespace()) {
            project
                .name
                .to_lowercase()
                .contains(&filter_text.to_lowercase())
        } else {
            true
        }
    });

    html! {
        <div class="flex space-x-4 justify-center">
            { filtered_projects.map(|project| html! {
                <ProjectCard name={project.name.clone()} description={project.description.clone()} />
            }).collect::<Html>() }
            if props.show_create_new {
                <div class="rounded-md w-55 p-4 bg-accent-secondary hover:z-10 transition hover:scale-110 flex flex-col items-center justify-center">
                    <h1 class="text-center text-text-dark font-medium text-lg font-bold">{ "Create" }</h1>
                    <div class="w-full h-full flex items-center justify-center">
                        <div class="inset-shadow-sm/50 w-30 h-30 rounded-full">
                            <svg class="w-full h-full p-2 text-text-dark" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                                <line x1="12" y1="5" x2="12" y2="19"></line>
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                            </svg>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
