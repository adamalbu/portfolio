use actix_web::body::MessageBody;
use actix_web::{App, HttpResponse, HttpServer, get, post, web::Json};
use argon2::password_hash::{PasswordHasher, Salt, SaltString};
use argon2::{Argon2, Params, PasswordHash, PasswordVerifier};
use portfolio_common::{LoginRequest, Project};
use std::fs;
use std::sync::LazyLock;

const PROJECTS_PATH: &'static str = "data/projects.json";
static HASHED_PASSWORD: LazyLock<PasswordHash<'_>> = LazyLock::new(|| {
    PasswordHash::new(
        "$argon2id$v=19$m=19456,t=2,p=1$23pZSQaNArI4$gm/0NnLyT1GOOxQyvmonH/Z665JnUsAiYavK3bi39do",
    )
    .unwrap()
});
static SALT: LazyLock<Salt<'_>> =
    LazyLock::new(|| Salt::from_b64("23pZSQaNArI4").expect("Failed to create salt"));

#[get("api/projects")]
async fn get_projects() -> HttpResponse {
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

#[post("api/login")]
async fn login(req: Json<LoginRequest>) -> HttpResponse {
    println!("Login request from {}", req.username);
    let ctx = Argon2::default();

    match ctx.verify_password(req.password.as_bytes(), &*HASHED_PASSWORD) {
        Ok(()) => HttpResponse::Ok().body("TODO"),
        Err(_) => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_projects))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

// fn main() {
//     let ctx = Argon2::default();
//     match ctx.verify_password("asdf".as_bytes(), &*HASHED_PASSWORD) {
//         Ok(()) => println!("Ok"),
//         Err(e) => eprintln!("Err: {}", e),
//     };
// }
