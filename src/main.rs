use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Result};
use enigo::{Enigo, KeyboardControllable, Key};

fn index_html() -> std::io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/")]
async fn hello() -> Result<NamedFile> {
    Ok(index_html()?)
}

#[get("/play")]
async fn echo() -> Result<NamedFile> {
    Enigo::new().key_click(Key::CapsLock);

    Ok(index_html()?)
}

const PORT: u16 = 8081;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server: http://localhost:{PORT}");

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
