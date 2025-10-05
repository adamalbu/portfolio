use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    cookie::{
        Cookie,
        time::{Duration, UtcDateTime},
    },
    get, post,
    web::Json,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::Salt};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode,
    jws::{Jws, encode},
};
use portfolio_common::{Claims, LoginRequest, Project};
use std::{error::Error, fs, sync::LazyLock};

const PROJECTS_PATH: &'static str = "data/projects.json";
static HASHED_PASSWORD: LazyLock<PasswordHash<'_>> = LazyLock::new(|| {
    PasswordHash::new(
        "$argon2id$v=19$m=19456,t=2,p=1$23pZSQaNArI4$gm/0NnLyT1GOOxQyvmonH/Z665JnUsAiYavK3bi39do",
    )
    .unwrap()
});
static SALT: LazyLock<Salt<'_>> =
    LazyLock::new(|| Salt::from_b64("23pZSQaNArI4").expect("Failed to create salt"));

static JWT_SECRET: LazyLock<Vec<u8>> = LazyLock::new(|| {
    dotenvy::dotenv().ok();
    std::env::var("JWT_SECRET").unwrap().into_bytes()
});

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

    if req.username != "admin" {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    match ctx.verify_password(req.password.as_bytes(), &*HASHED_PASSWORD) {
        Err(_) => return HttpResponse::Unauthorized().body("Invalid username or password"),
        Ok(()) => {}
    };

    let expiration = UtcDateTime::now()
        .checked_add(Duration::days(7))
        .unwrap()
        .unix_timestamp() as usize;

    let claims = Claims { exp: expiration };

    let jws: Jws<Claims> = encode(
        &Header::default(),
        Some(&claims),
        &EncodingKey::from_secret(&*JWT_SECRET),
    )
    .unwrap();

    let token = format!("{}.{}.{}", jws.protected, jws.payload, jws.signature);

    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Lax)
        .secure(cfg!(not(debug_assertions))) // Secure in prod, insecure in debug
        .max_age(Duration::days(7))
        .finish();

    println!("cookie: {:#?}", cookie);
    HttpResponse::Ok().cookie(cookie).body("Login successful")
}

#[post("/api/logout")]
async fn logout() -> HttpResponse {
    let cookie = Cookie::build("auth_token", "")
        .path("/")
        .max_age(Duration::seconds(0))
        .finish();

    HttpResponse::Ok().cookie(cookie).finish()
}

fn validate(req: &HttpRequest) -> Result<(), actix_web::Error> {
    let jwt_cookie = req
        .cookie("auth_token")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("No auth token"))?;

    decode::<Claims>(
        jwt_cookie.value(),
        &DecodingKey::from_secret(&*JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(())
}

#[get("api/auth-status")]
async fn auth_status(req: HttpRequest) -> HttpResponse {
    match validate(&req) {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::Unauthorized().body(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_projects)
            .service(login)
            .service(logout)
            .service(auth_status)
    })
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
