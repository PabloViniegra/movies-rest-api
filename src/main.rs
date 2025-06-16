mod db;
mod models;
mod route_handler;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db::establish_connection;
use route_handler::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        route_handler::list_movies_full,
        route_handler::list_movies,
        route_handler::add_movie,
        route_handler::list_directors,
        route_handler::add_director,
        route_handler::list_actors,
        route_handler::add_actor,
        route_handler::list_genres,
        route_handler::add_genre
    ),
    components(
        schemas(
            route_handler::CreateDirector,
            route_handler::MovieFull,
            route_handler::Meta,
            route_handler::MovieFullResponse,
            route_handler::CreateMovie,
            route_handler::CreateActor,
            route_handler::CreateGenre,
        )
    ),
    tags(
        (name = "movies", description = "Gestión de películas"),
        (name = "directors", description = "Gestión de directores"),
        (name = "actors", description = "Gestión de actores"),
        (name = "genres", description = "Gestión de géneros")
    )
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = establish_connection().await;
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(db.clone()))
            .route("/movies", web::get().to(list_movies))
            .route("/movies", web::post().to(add_movie))
            .route("/movies/full", web::get().to(list_movies_full))
            .route("/directors", web::get().to(list_directors))
            .route("/directors", web::post().to(add_director))
            .route("/actors", web::get().to(list_actors))
            .route("/actors", web::post().to(add_actor))
            .route("/genres", web::get().to(list_genres))
            .route("/genres", web::post().to(add_genre))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
