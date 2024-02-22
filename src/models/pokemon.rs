use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// Pokemon Model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Pokemon {
  pub id: u64,
  pub name: String,
  pub types: PokemonType,
  /// u16 -> 0 : 65535
  pub attack: u16,
  pub defense: u16,
  pub health: u16,
  pub birthday: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum PokemonType {
  Normal,
  Fire,
  Water,
  Electric,
  Grass,
  Ice,
  Fighting,
  Poison,
  Ground,
  Flying,
  Psychic,
  Bug,
  Rock,
  Ghost,
  Dragon,
}
