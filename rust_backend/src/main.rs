#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

use std::fs::read_to_string;
use std::path::PathBuf;

lazy_static! {
    static ref BASE_DIR: PathBuf = PathBuf::from("/home/cam/Programming/camwilliams_ca");
    static ref STATIC_FILES_DIR: PathBuf =
        PathBuf::from("/home/cam/Programming/camwilliams_ca/frontend/dist/static/");
    static ref TEMPLATE_DIR: PathBuf =
        PathBuf::from("/home/cam/Programming/camwilliams_ca/frontend/dist/");
}

/// Helper function to return a template from given &str/String
fn render_template<'a, N: Into<&'a str>>(name: N) -> impl Responder {
    // Try and open template from static files, return 404 if template doesn't exist?
    let mut p = TEMPLATE_DIR.clone();
    p.push(name.into());
    if !p.exists() {
        return HttpResponse::NotFound().body("");
    }
    let contents: String = read_to_string(p).unwrap().parse().unwrap();
    HttpResponse::Ok().body(contents)
}

/// Main index route.
#[get("/")]
async fn index() -> impl Responder {
    render_template("index.html")
}

/// Route for static files serving.
#[get("/static/{filename:.*}")]
async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let mut p = STATIC_FILES_DIR.clone();
    let path: PathBuf = req.match_info().query("filename").parse()?;
    p.push(&path);
    info!("{:?}", p);
    Ok(NamedFile::open(p)?)
}


#[get("/portfolio")]
async fn portfolio() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Hello world.");

    HttpServer::new(|| App::new().service(index).service(static_files))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
