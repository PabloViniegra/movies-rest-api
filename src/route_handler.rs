use crate::models::{actor, director, genre, movie, movie_actor, movie_genre};
use actix_web::{HttpResponse, Responder, web};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//
// --- Director Endpoints ---
//

#[derive(Deserialize, ToSchema)]
pub struct CreateDirector {
    pub name: String,
}
#[utoipa::path(
    post,
    path = "/directors",
    request_body = CreateDirector,
    responses(
        (status = 201, description = "Director created", body = director::Model),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn add_director(
    db: web::Data<DatabaseConnection>,
    form: web::Json<CreateDirector>,
) -> impl Responder {
    let director = director::ActiveModel {
        name: Set(form.name.clone()),
        ..Default::default()
    };

    match director.insert(db.get_ref()).await {
        Ok(model) => HttpResponse::Created().json(model),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/directors",
    responses(
        (status = 200, description = "List of directors", body = Vec<director::Model>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn list_directors(db: web::Data<DatabaseConnection>) -> impl Responder {
    match director::Entity::find().all(db.get_ref()).await {
        Ok(directors) => HttpResponse::Ok().json(directors),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
//
// --- Actor endpoints ---
//

#[derive(Deserialize, ToSchema)]
pub struct CreateActor {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/actors",
    request_body = CreateActor,
    responses(
        (status = 201, description = "Actor created", body = actor::Model),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn add_actor(
    db: web::Data<DatabaseConnection>,
    form: web::Json<CreateActor>,
) -> impl Responder {
    let actor = actor::ActiveModel {
        name: Set(form.name.clone()),
        ..Default::default()
    };
    match actor.insert(db.get_ref()).await {
        Ok(actor) => HttpResponse::Created().json(actor),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/actors",
    responses(
        (status = 200, description = "List of actors", body = Vec<actor::Model>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn list_actors(db: web::Data<DatabaseConnection>) -> impl Responder {
    match actor::Entity::find().all(db.get_ref()).await {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

//
// --- Genre Endpoints ---
//

#[derive(Deserialize, ToSchema)]
pub struct CreateGenre {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/genres",
    request_body = CreateGenre,
    responses(
        (status = 201, description = "Genre created", body = genre::Model),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn add_genre(
    db: web::Data<DatabaseConnection>,
    form: web::Json<CreateGenre>,
) -> impl Responder {
    let genre = genre::ActiveModel {
        name: Set(form.name.clone()),
        ..Default::default()
    };

    match genre.insert(db.get_ref()).await {
        Ok(model) => HttpResponse::Created().json(model),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/genres",
    responses(
        (status = 200, description = "List of genres", body = Vec<genre::Model>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn list_genres(db: web::Data<DatabaseConnection>) -> impl Responder {
    match genre::Entity::find().all(db.get_ref()).await {
        Ok(genres) => HttpResponse::Ok().json(genres),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

//
// --- Movie Endpoints ---
//

#[derive(Deserialize, ToSchema)]
pub struct CreateMovie {
    pub title: String,
    pub director_id: i32,
    pub actor_ids: Vec<i32>,
    pub genre_ids: Vec<i32>,
}

// GET /movies
#[utoipa::path(
    get,
    path = "/movies",
    responses(
        (status = 200, description = "List of movies", body = Vec<movie::Model>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn list_movies(db: web::Data<DatabaseConnection>) -> impl Responder {
    match movie::Entity::find().all(db.get_ref()).await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// POST /movies (incluyendo relaciones)
#[utoipa::path(
    post,
    path = "/movies",
    request_body = CreateMovie,
    responses(
        (status = 201, description = "Movie created", body = movie::Model),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn add_movie(
    db: web::Data<DatabaseConnection>,
    form: web::Json<CreateMovie>,
) -> impl Responder {
    let director_exists = director::Entity::find_by_id(form.director_id)
        .one(db.get_ref())
        .await;

    match director_exists {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::BadRequest().body("Director does not exist"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }

    let movie = movie::ActiveModel {
        title: Set(form.title.clone()),
        director_id: Set(form.director_id),
        ..Default::default()
    };

    let created_movie = match movie.insert(db.get_ref()).await {
        Ok(model) => model,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };

    for &actor_id in &form.actor_ids {
        let actor_exists = actor::Entity::find_by_id(actor_id).one(db.get_ref()).await;
        match actor_exists {
            Ok(Some(_)) => {}
            Ok(None) => {
                return HttpResponse::BadRequest()
                    .body(format!("Actor id {} does not exist", actor_id));
            }
            Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
        }

        let movie_actor_rel = movie_actor::ActiveModel {
            movie_id: Set(created_movie.id),
            actor_id: Set(actor_id),
        };
        if let Err(e) = movie_actor_rel.insert(db.get_ref()).await {
            return HttpResponse::InternalServerError().body(format!("Error movie-actor: {}", e));
        }
    }

    for &genre_id in &form.genre_ids {
        let genre_exists = genre::Entity::find_by_id(genre_id).one(db.get_ref()).await;
        match genre_exists {
            Ok(Some(_)) => {}
            Ok(None) => {
                return HttpResponse::BadRequest()
                    .body(format!("Genre id {} does not exist", genre_id));
            }
            Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
        }

        let movie_genre_rel = movie_genre::ActiveModel {
            movie_id: Set(created_movie.id),
            genre_id: Set(genre_id),
        };
        if let Err(e) = movie_genre_rel.insert(db.get_ref()).await {
            return HttpResponse::InternalServerError().body(format!("Error movie-genre: {}", e));
        }
    }

    HttpResponse::Created().json(created_movie)
}

#[derive(Deserialize, ToSchema)]
pub struct MovieFullQuery {
    q: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct MovieFull {
    pub id: i32,
    pub title: String,
    pub director: Option<director::Model>,
    pub actors: Vec<actor::Model>,
    pub genres: Vec<genre::Model>,
}

#[derive(Serialize, ToSchema)]
pub struct Meta {
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub last_page: u32,
}

#[derive(Serialize, ToSchema)]
pub struct MovieFullResponse {
    pub meta: Meta,
    pub results: Vec<MovieFull>,
}

// GET /movies/full
#[utoipa::path(
    get,
    path = "/movies/full",
    responses(
        (status = 200, description = "Lista de películas full", body = MovieFullResponse)
    ),
    params(
        ("q" = Option<String>, Query, description = "Texto de búsqueda"),
        ("page" = Option<u32>, Query, description = "Página"),
        ("per_page" = Option<u32>, Query, description = "Resultados por página")
    )
)]
pub async fn list_movies_full(
    db: web::Data<DatabaseConnection>,
    query: web::Query<MovieFullQuery>,
) -> impl Responder {
    use std::collections::HashSet;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).max(1).min(100);
    let offset = (page - 1) * per_page;

    let q = query
        .q
        .as_ref()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty());

    let mut movie_ids = Vec::<i32>::new();

    if let Some(q) = q.clone() {
        let movies_by_title = movie::Entity::find()
            .filter(movie::Column::Title.contains(&q))
            .all(db.get_ref())
            .await
            .unwrap_or_default();
        movie_ids.extend(movies_by_title.iter().map(|m| m.id));

        let directors = director::Entity::find()
            .filter(director::Column::Name.contains(&q))
            .all(db.get_ref())
            .await
            .unwrap_or_default();
        let director_ids: Vec<i32> = directors.iter().map(|d| d.id).collect();
        if !director_ids.is_empty() {
            let movies_by_director = movie::Entity::find()
                .filter(movie::Column::DirectorId.is_in(director_ids.clone()))
                .all(db.get_ref())
                .await
                .unwrap_or_default();
            movie_ids.extend(movies_by_director.iter().map(|m| m.id));
        }

        let genres = genre::Entity::find()
            .filter(genre::Column::Name.contains(&q))
            .all(db.get_ref())
            .await
            .unwrap_or_default();
        let genre_ids: Vec<i32> = genres.iter().map(|g| g.id).collect();
        if !genre_ids.is_empty() {
            let movie_genres = movie_genre::Entity::find()
                .filter(movie_genre::Column::GenreId.is_in(genre_ids.clone()))
                .all(db.get_ref())
                .await
                .unwrap_or_default();
            movie_ids.extend(movie_genres.iter().map(|mg| mg.movie_id));
        }

        let actors = actor::Entity::find()
            .filter(actor::Column::Name.contains(&q))
            .all(db.get_ref())
            .await
            .unwrap_or_default();
        let actor_ids: Vec<i32> = actors.iter().map(|a| a.id).collect();
        if !actor_ids.is_empty() {
            let movie_actors = movie_actor::Entity::find()
                .filter(movie_actor::Column::ActorId.is_in(actor_ids.clone()))
                .all(db.get_ref())
                .await
                .unwrap_or_default();
            movie_ids.extend(movie_actors.iter().map(|ma| ma.movie_id));
        }
    }

    let movie_ids: Vec<i32> = {
        let set: HashSet<_> = movie_ids.into_iter().collect();
        set.into_iter().collect()
    };

    let mut movies_query = movie::Entity::find();
    if query.q.is_some() {
        if movie_ids.is_empty() {
            let meta = Meta {
                total: 0,
                page,
                per_page,
                last_page: 1,
            };
            let response = MovieFullResponse {
                meta,
                results: Vec::new(),
            };
            return HttpResponse::Ok().json(response);
        }
        movies_query = movies_query.filter(movie::Column::Id.is_in(movie_ids));
    }

    let total = match movies_query.clone().count(db.get_ref()).await {
        Ok(c) => c as usize,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };

    let last_page = ((total as f32) / (per_page as f32)).ceil().max(1.0) as u32;

    let movies = match movies_query
        .order_by_asc(movie::Column::Id)
        .offset(Some(offset as u64))
        .limit(Some(per_page as u64))
        .all(db.get_ref())
        .await
    {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    };

    let mut result = Vec::new();

    for mov in movies {
        let director = director::Entity::find_by_id(mov.director_id)
            .one(db.get_ref())
            .await
            .ok()
            .flatten();

        let actor_ids: Vec<i32> = movie_actor::Entity::find()
            .filter(movie_actor::Column::MovieId.eq(mov.id))
            .all(db.get_ref())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|ma| ma.actor_id)
            .collect();

        let actors = if !actor_ids.is_empty() {
            actor::Entity::find()
                .filter(actor::Column::Id.is_in(actor_ids))
                .all(db.get_ref())
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let genre_ids: Vec<i32> = movie_genre::Entity::find()
            .filter(movie_genre::Column::MovieId.eq(mov.id))
            .all(db.get_ref())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|mg| mg.genre_id)
            .collect();

        let genres = if !genre_ids.is_empty() {
            genre::Entity::find()
                .filter(genre::Column::Id.is_in(genre_ids))
                .all(db.get_ref())
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        result.push(MovieFull {
            id: mov.id,
            title: mov.title,
            director,
            actors,
            genres,
        });
    }

    let meta = Meta {
        total,
        page,
        per_page,
        last_page,
    };

    let response = MovieFullResponse {
        meta,
        results: result,
    };

    HttpResponse::Ok().json(response)
}
