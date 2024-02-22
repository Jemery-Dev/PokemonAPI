use ntex::web;

use crate::models::pokemon::{Pokemon, PokemonType};

// List All Pokemons
#[utoipa::path( // Make a path for each function with status, description, body...
    get,
    path = "/pokemon",
    responses(
    (status = 200, description = "List of Pokemon", body = [Pokemon]),
    ),
)]
#[web::get("/pokemon")]
pub async fn get_pokemons() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}


//Create a Pokemon
#[utoipa::path(
    post,
    path = "/pokemon",
    request_body = Pokemon,
    responses(
    (status = 201, description = "Pokemon created", body = Pokemon),
    ),
)]
#[web::post("/pokemon")]
pub async fn create_pokemon(
  _pokemon: web::types::Json<Pokemon>,
) -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}


/// Two status if the pokemon is or not found
#[utoipa::path(
    get,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon found", body = Pokemon),
    (status = 404, description = "Pokemon not found", body = HttpError),
    ),
)]
#[web::get("/pokemon/{id}")]
pub async fn get_pokemon(_path: web::types::Path<u64>) -> web::HttpResponse {
    // Make instance of a Pokemon
    let first_zissure = Pokemon {
        id: 1,
        name: String::from("Pikachu"),
        types: (PokemonType::Electric, None),
        attack: 100,
        defense: 80,
        health: 120,
        birthday: String::from("2012-12-12"),
    };

    // Show JSON
    web::HttpResponse::Ok().json(&first_zissure)
}

#[utoipa::path(
    put,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon updated", body = Pokemon),
    (status = 404, description = "Pokemon not updated", body = HttpError),
    ),
)]
#[web::put("/pokemon/{id}")]
pub async fn update_pokemon() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}


#[utoipa::path(
get,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon deleted", body = Pokemon),
    (status = 404, description = "Pokemon not found", body = HttpError),
    ),
)]
#[web::delete("/pokemon/{id}")]
pub async fn delete_pokemon() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) { // List of all functions for ntex config
  cfg.service(get_pokemons);
  cfg.service(create_pokemon);
  cfg.service(get_pokemon);
  cfg.service(update_pokemon);
  cfg.service(delete_pokemon);
}
