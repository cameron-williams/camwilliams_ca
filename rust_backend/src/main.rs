#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

use std::fs::read_to_string;
use std::path::PathBuf;

lazy_static! {
    // Base directory. Allowed to be set with $CW_DIR but defaults to current dir.
    static ref BASE_DIR: PathBuf = match std::env::var("CW_DIR") {
            Ok(p) => PathBuf::from(p),
            Err(_) => {
                std::env::current_dir().map(|mut p| {p.pop(); p}).unwrap()
            }
        };
}

/// Helper function to return a template from given &str/String
fn render_template<'a, N: Into<&'a str>>(name: N) -> impl Responder {
    // Try and open template from static files, return 404 if template doesn't exist?
    let mut p = BASE_DIR.clone();
    p.push(format!("frontend/dist/{}", name.into()));
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
    let mut p = BASE_DIR.clone();
    let path: PathBuf = req.match_info().query("filename").parse()?;
    p.push(format!("frontend/dist/static/{}", &path.to_str().unwrap()));
    Ok(NamedFile::open(p)?)
}

/// Route specific for resume.
#[get("/resume")]
async fn resume() -> Result<NamedFile> {
    let mut p = BASE_DIR.clone();
    p.push(format!("frontend/dist/static/resume.pdf"));
    Ok(NamedFile::open(p)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Listening on 127.0.0.1:3000");
    HttpServer::new(|| App::new().service(index).service(static_files).service(resume))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
