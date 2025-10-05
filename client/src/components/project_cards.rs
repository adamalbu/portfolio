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
        <div class="flex space-x-4">
            { filtered_projects.map(|project| html! {
                <ProjectCard name={project.name.clone()} description={project.description.clone()} />
            }).collect::<Html>() }
        </div>
    }
}
