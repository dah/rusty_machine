use crate::machine::basket::Basket;
use crate::machine::TankType;

pub struct Tank {
    pub tank_type: TankType,
    pub occupants: Vec<Basket>,
    pub max_occupancy: u8,
}
