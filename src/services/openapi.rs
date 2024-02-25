use std::sync::Arc;

use ntex::web;
use ntex::http;
use ntex::util::Bytes;
use utoipa::OpenApi;

use crate::error::HttpError;
use crate::models::pokemon::{Pokemon, PokemonType};
use crate::services::pokemon::{create_pokemon, delete_pokemon, get_pokemon, get_pokemons, get_stats, update_pokemon};

use super::pokemon;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
  paths(
    pokemon::get_pokemons,
    pokemon::create_pokemon,
    pokemon::get_pokemon,
    pokemon::update_pokemon,
    pokemon::delete_pokemon,
  ),
  components(schemas(Pokemon, PokemonType, HttpError))
)]

pub(crate) struct ApiDoc;

// OpenpApi Code

#[web::get("/{tail}*")]
async fn get_swagger(
  tail: web::types::Path<String>,
  openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> Result<web::HttpResponse, HttpError> {
  if tail.as_ref() == "swagger.json" {
    let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
      msg: format!("Error generating OpenAPI spec: {}", err),
    })?;
    return Ok(
      web::HttpResponse::Ok()
        .content_type("application/json")
        .body(spec),
    );
  }
  let conf = openapi_conf.as_ref().clone();
  match utoipa_swagger_ui::serve(&tail, conf.into()).map_err(|err| {
    HttpError {
      msg: format!("Error serving Swagger UI: {}", err),
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  })? {
    None => Err(HttpError {
      status: http::StatusCode::NOT_FOUND,
      msg: format!("path not found: {}", tail),
    }),
    Some(file) => Ok({
      let bytes = Bytes::from(file.bytes.to_vec());
      web::HttpResponse::Ok()
        .content_type(file.content_type)
        .body(bytes)
    }),
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  let swagger_config = Arc::new(
    utoipa_swagger_ui::Config::new(["/api/swagger.json"])
      .use_base_layout(),
  );
  config.service(
    web::scope("api") // For all paths with "localhost:8080/api/..."
        .service(get_stats)
      .service(get_pokemons)
      .service(create_pokemon)
      .service(get_pokemon)
      .service(update_pokemon)
      .service(delete_pokemon)
      .state(swagger_config)
      .service(get_swagger),
  );
}
