pub mod pokemon;
pub mod openapi;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
