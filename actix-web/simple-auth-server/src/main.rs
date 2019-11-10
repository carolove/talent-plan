extern crate actix_web;
use actix_web::{middleware, web, App, HttpRequest, HttpServer};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.htlm").to(|| "hello world!"))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}
