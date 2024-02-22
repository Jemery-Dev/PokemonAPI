use ntex::web;

mod error;
mod services;
mod models;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Register swagger endpoints
      .configure(services::openapi::ntex_config)
      // Register pokemon endpoints
      .configure(services::pokemon::ntex_config)
      // Default endpoint for unregistered endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())

}
