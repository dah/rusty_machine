use crate::machine::basket::Basket;
use crate::machine::tank::Tank;
use tabled::{Tabled};
use std::fmt;

pub mod basket;
pub mod recipe;
pub mod robot;
pub mod step;
pub mod tank;

pub struct Machine {
    pub tanks: Vec<Tank>,
}

impl Machine {
    pub fn new(tanks: Vec<Tank>) -> Self {
        Self { tanks }
    }

    pub fn tick(&mut self) {
        let mut tank_indices: Vec<usize> = Vec::new();
        let mut occupant_indices: Vec<usize> = Vec::new();
        for (i, tank) in self.tanks.iter().enumerate() {
            for (j, basket) in tank.occupants.iter().enumerate() {
                if basket.status == BasketStatus::Processing
                    || basket.status == BasketStatus::WaitingToMove
                {
                    tank_indices.push(i);
                    occupant_indices.push(j);
                }
            }
        }
        loop {
            if tank_indices.len() == 0 {
                break;
            }
            let b = &mut self.tanks[tank_indices.pop().unwrap()].occupants
                [occupant_indices.pop().unwrap()];
            if b.recipe.steps[b.current_step.unwrap()].tick() {
                b.status = BasketStatus::WaitingToMove;
            }
        }
    }

    pub fn is_tank_available(&self, tank_name: TankType) -> bool {
        let mut available = false;
        for tank in self.tanks.iter() {
            if tank_name == tank.tank_type {
                if tank.occupants.len() < tank.max_occupancy.into() {
                    available = true;
                }
            }
        }
        return available;
    }
    pub fn drop_off_basket(&mut self, mut basket: Basket) {
        //basket.current_step += 1;
        basket.status = BasketStatus::Processing;
        for tank in self.tanks.iter_mut() {
            if tank.tank_type == basket.recipe.steps[basket.current_step.unwrap()].tank_type {
                tank.occupants.push(basket);
                break;
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Tabled)]
pub enum TankType {
    Etch,
    EtchRinse,
    Clean,
    CleanRinse,
    DIRinse,
    Cool,
    Oven,
    Grey,
    Primer,
    Lacquer,
}

impl fmt::Display for TankType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TankType::Etch => "Etch",
            TankType::EtchRinse => "EtchRinse",
            TankType::Clean => "Clean",
            TankType::CleanRinse => "CleanRinse",
            TankType::DIRinse => "DIRinse",
            TankType::Cool => "Cool",
            TankType::Oven => "Oven",
            TankType::Grey => "Grey",
            TankType::Primer => "Primer",
            TankType::Lacquer => "Lacquer",
        };
        write!(f, "{}", printable)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BasketStatus {
    WaitingToLoad,
    Unloaded,
    Processing,
    WaitingToMove,
    Moving,
}

#[derive(PartialEq, Clone)]
pub enum RobotStatus {
    Idle,
    WillMove,
    Moving,
    DropOff,
}

impl Machine {
    pub fn umbra60_3t() -> Self {
        let tanks = vec![
            Tank {
                tank_type: TankType::Etch,
                occupants: Vec::new(),
                max_occupancy: 2,
            },
            Tank {
                tank_type: TankType::EtchRinse,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
            Tank {
                tank_type: TankType::Clean,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
            Tank {
                tank_type: TankType::CleanRinse,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
            Tank {
                tank_type: TankType::DIRinse,
                occupants: Vec::new(),
                max_occupancy: 2,
            },
            Tank {
                tank_type: TankType::Cool,
                occupants: Vec::new(),
                max_occupancy: 2,
            },
            Tank {
                tank_type: TankType::Oven,
                occupants: Vec::new(),
                max_occupancy: 5,
            },
            Tank {
                tank_type: TankType::Grey,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
            Tank {
                tank_type: TankType::Primer,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
            Tank {
                tank_type: TankType::Lacquer,
                occupants: Vec::new(),
                max_occupancy: 1,
            },
        ];
        Self::new(tanks)
    }
}