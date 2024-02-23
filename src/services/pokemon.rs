use std::sync::Mutex;
use lazy_static::lazy_static;
use ntex::web::types::Json;
use ntex::web;
use ntex::web::{HttpResponse};



use crate::models::pokemon::Pokemon;

lazy_static! {
    static ref POKEMON_VEC: Mutex<Vec<Pokemon>> = Mutex::new(Vec::new());
}


/// List All Pokemons
#[utoipa::path(
    get,
    path = "/pokemon",
    responses(
    (status = 200, description = "List of Pokemon")
    ),
)]
#[web::get("/pokemon")]
pub async fn get_pokemons() -> web::HttpResponse {
    let data = POKEMON_VEC.lock().unwrap(); // Get the data from POKEMON_VEC with lock

    let json_data = serde_json::to_string(&*data).unwrap(); // Transform data into Json
    web::HttpResponse::Ok().json(&json_data) // Return JSON response

}


/// Create a Pokemon
#[utoipa::path(
    post,
    path = "/pokemon",
    request_body = Pokemon,
    responses(
    (status = 201, description = "Pokemon created", body = Pokemon),
    ),
)]
#[web::post("/pokemon")]
pub async fn create_pokemon(pokemon: Json<Pokemon>) -> HttpResponse {
    let mut data = POKEMON_VEC.lock().unwrap();
    data.push(pokemon.clone());
    HttpResponse::Created().body(format!("Created pokemon {}", pokemon.id))
}



/// Find Pokemon by ID
#[utoipa::path(
    get,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon found", body = Pokemon),
    (status = 404, description = "Pokemon not found", body = HttpError),
    ),
)]
#[web::get("/pokemon/{id}")]
pub async fn get_pokemon(id: web::types::Path<u64>) -> web::HttpResponse {
    // Take ID of request
    let id_pokemon = id.into_inner();

    // Unlock the vec
    let data = POKEMON_VEC.
        lock()
        .unwrap();

    // Get Pokemon with the ID
    let found_pokemon = data
        .iter()
        .find(|pokemon| pokemon.id == id_pokemon);

    // Return the Pokemon
    web::HttpResponse::Ok().json(&found_pokemon)
}
/// Update a Pokemon by ID
#[utoipa::path(
    put,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon updated", body = Pokemon),
    (status = 404, description = "Pokemon not updated", body = HttpError),
    ),
)]
#[web::put("/pokemon/{id}")]
pub async fn update_pokemon(pokemon: Json<Pokemon>, id: web::types::Path<u64>) -> web::HttpResponse {
    let id_pokemon = id.into_inner(); // Get ID of Pokemon

    let mut data = POKEMON_VEC.lock().unwrap(); // Lock Pokemon_VEC to get the data

    // If the Pokemon exist
    if let Some(index) = data.iter().position(|p| p.id == id_pokemon) {
        // Update the Pokemon at the found index
        let old_id = data[index].id; // To get ID before update for the body reponse
        data[index] = pokemon.into_inner();

        web::HttpResponse::Ok().body(format!("Pokemon {} was updated", old_id))
    }
    else { // Pokemon not found
        web::HttpResponse::NotFound().body("Pokemon not found")
    }
}

/// Delete a Pokemon by ID
#[utoipa::path(
delete,
    path = "/pokemon/{id}",
    responses(
    (status = 200, description = "Pokemon deleted"),
    (status = 404, description = "Pokemon not found", body = HttpError),
    ),
)]
#[web::delete("/pokemon/{id}")]
pub async fn delete_pokemon(id: web::types::Path<u64>) -> web::HttpResponse {
    let id_pokemon = id.into_inner();

    let mut data = POKEMON_VEC.lock().unwrap();

    // Find the index of the Pokemon with the specified ID
    if let Some(index) = data.iter().position(|pokemon| pokemon.id == id_pokemon) {

        // Remove the Pokemon at the found index
        data.remove(index);
        web::HttpResponse::Ok().body(format!("Pokemon with ID {} deleted", id_pokemon))
    } else {
        // If the Pokemon with the specified ID is not found
        web::HttpResponse::NotFound().body("Pokemon not found")
    }
}


pub fn ntex_config(cfg: &mut web::ServiceConfig) { // List of all functions for ntex config
  cfg.service(get_pokemons);
  cfg.service(create_pokemon);
  cfg.service(get_pokemon);
  cfg.service(update_pokemon);
  cfg.service(delete_pokemon);
}
