use actix_web::{App, HttpResponse, HttpServer, get};
use portfolio_common::Project;
use std::fs;

const PROJECTS_PATH: &'static str = "data/projects.json";

#[get("api/projects")]
async fn get_projects() -> HttpResponse {
    println!("projects");
    let contents = match fs::read_to_string(PROJECTS_PATH) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let projects: Vec<Project> = match serde_json::from_str(&contents) {
        Ok(projects) => projects,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(projects)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_projects))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
