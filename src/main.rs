use actix_files::NamedFile;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

mod server;
use self::server::WWWebsocket;

async fn ww_server(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WWWebsocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting the WS server on ws://localhost:3001");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(ww_server)))
            .wrap(middleware::Logger::default())
    })
    .workers(4)
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
