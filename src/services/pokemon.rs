use std::fs::OpenOptions;
use std::io::{Write};
use std::sync::Mutex;
use lazy_static::lazy_static;
use ntex::web::types::Json;
use ntex::web;
use ntex::web::{HttpResponse};


use crate::models::pokemon::Pokemon;

lazy_static! {
    static ref POKEMON_VEC: Mutex<Vec<Pokemon>> = Mutex::new(Vec::new());
}

macro_rules! save_to_json_file {
    () => {
        let mut file = OpenOptions::new() // Make instance OpenOptions
            .create(true) // If the file doesn't exist
            .write(true) // Write acces to let the file be created
            .truncate(true)
            .open("pokemon.json") // File name
            .unwrap(); // To get file

        let json_data = serde_json::to_string_pretty(&*POKEMON_VEC);

        file.write_all(json_data.unwrap().as_bytes())// Write all data of json_data in file
            .expect("Failed to write to file");
        };
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
    save_to_json_file!();
    web::HttpResponse::Ok().json(&*POKEMON_VEC) // Send back the data of the Vec
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

    //Get data of Vec
    let mut data = POKEMON_VEC.lock().unwrap();

    //Check if pokemon of {{id}} already exists
    if(data.iter().find(|pokemondata| pokemondata.id == pokemon.id)).is_some(){
        //Return error
        HttpResponse::BadRequest().body(format!("Pokemon of ID {} already exists", pokemon.id))
    } else {
        //Else push into vec
        data.push(pokemon.clone());

        drop(data); // To drap clone before using file

        save_to_json_file!(); // Save in file
        HttpResponse::Created().body(format!("Created pokemon {}", pokemon.id))
    }
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

    // Check if pokemon exist
    if data.iter()
        .find(|pokemon| pokemon.id == id_pokemon).is_some(){

        // Return the Pokemon
        web::HttpResponse::Ok().json(&data
            .iter()
            .find(|pokemon| pokemon.id == id_pokemon))
    }
    // Return 404 error
    else{
        web::HttpResponse::BadRequest().body(format!("Pokemon of ID {} not found", id_pokemon))
    }


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

        //Check if some Pokemon of {{id}} already exist
        if(data.iter().find(|pokemondata| pokemondata.id == pokemon.id)).is_some() {
            HttpResponse::BadRequest().body(format!("Pokemon of ID {} already exists", pokemon.id))
        }
        else {
            // Update the Pokemon at the found index
            let old_id = data[index].id; // To get ID before update for the body reponse
            data[index] = pokemon.into_inner();

            drop(data);
            // To drap clone before using file
            save_to_json_file!(); // Save in file
            web::HttpResponse::Ok().body(format!("Pokemon {} was updated", old_id))
        }
    }
    else { // Pokemon not found
        drop(data); // To drap clone before using file
        save_to_json_file!(); // Save in file
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

        drop(data); // To drap clone before using file
        save_to_json_file!(); // Save in file
        web::HttpResponse::Ok().body(format!("Pokemon with ID {} deleted", id_pokemon))
    } else {

        drop(data); // To drap clone before using file
        save_to_json_file!(); // Save in file
        // If the Pokemon with the specified ID is not found
        web::HttpResponse::NotFound().body("Pokemon doesn't not exist")
    }
}

///Get statistics from data
#[utoipa::path(
get,
path = "/pokemon/stat",
responses(
(status = 200, description = "List of Pokemon")
),
)]
#[web::get("/pokemon/stat")]
pub async fn get_stats() -> web::HttpResponse {
    let data = POKEMON_VEC.lock().unwrap(); // Get the data from POKEMON_VEC with lock
    let json_data = format!("There are {} Pokemons currently \n", data.len());

    web::HttpResponse::Ok().json(&json_data) // Return JSON response

}

pub fn ntex_config(cfg: &mut web::ServiceConfig) { // List of all functions for ntex config
  cfg.service(get_stats);
  cfg.service(get_pokemons);
  cfg.service(create_pokemon);
  cfg.service(get_pokemon);
  cfg.service(update_pokemon);
  cfg.service(delete_pokemon);
}
