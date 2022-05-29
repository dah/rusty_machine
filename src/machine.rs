use crate::machine::basket::Basket;
use crate::machine::tank::Tank;
use crate::Robot;
use tabled::Table;

use std::fmt;

pub mod basket;
pub mod recipe;
pub mod robot;
pub mod step;
pub mod tank;

pub struct Machine {
    pub tanks: Vec<Tank>,
    pub robot: Robot,
    pub baskets: Vec<Basket>,
    pub unloaded_baskets: Vec<Basket>,
    pub elapsed_seconds: u64,
    pub tank_pickup: Option<usize>,
    pub position_pickup: Option<usize>,
}

impl Machine {
    fn new(tanks: Vec<Tank>) -> Self {
        let robot = Robot::new();
        let baskets = Vec::new();
        let unloaded_baskets = Vec::new();
        let elapsed_seconds = 0;
        let tank_pickup = None;
        let position_pickup = None;
        Self {
            tanks,
            robot,
            baskets,
            unloaded_baskets,
            elapsed_seconds,
            tank_pickup,
            position_pickup,
        }
    }

    pub fn load(&mut self, baskets: Vec<Basket>) {
        self.baskets = baskets;
    }

    pub fn run(&mut self) {
        loop {
            // Check if run is complete and break out from loop
            if self.baskets.is_empty() && self.is_empty() {
                println!("\nFinished Unloading All Baskets\n");
                for basket in self.unloaded_baskets.iter() {
                    println!("***** Basket {}", basket.job_id);
                    //basket.print_waiting_steps();
                    let steps = &basket.recipe.steps;
                    let table = Table::new(steps).to_string();
                    println!("{}", table);
                    println!("\n");
                }

                break;
            }

            // Check if machine is blocked
            if self.is_stuck() {
                println!("Robot Stuck: Could not complete after in 1 simulated day.");
                println!(
                    "Unloaded {} out of {} baskets.",
                    self.unloaded_baskets.len(),
                    self.unloaded_baskets.len() + self.baskets.len() + self.baskets_in_machine()
                );
                break;
            }

            // If the robot isn't do anything scan for available jobs
            if self.robot.is_idle() {
                //Scan for a basket that is ready to move in the machine
                for (i, tank) in self.tanks.iter().enumerate() {
                    for (j, basket) in tank.occupants.iter().enumerate() {
                        if let BasketStatus::WaitingToMove = basket.status {
                            // first check if that basket needs to unload
                            if basket.is_final_step() {
                                println!("Robot will unload basket {}", basket.job_id);
                                self.robot.status = RobotStatus::WillMove;
                                self.tank_pickup = Some(i);
                                self.position_pickup = Some(j);
                            } else {
                                let available: Option<bool>;
                                // if the next step is not unload, then check if the next tank is available to move to
                                match basket.recipe.steps[basket.next_step.unwrap()].tank_type {
                                    TankType::Etch => {
                                        available = Some(self.is_tank_available(TankType::Etch))
                                    }
                                    TankType::EtchRinse => {
                                        available =
                                            Some(self.is_tank_available(TankType::EtchRinse))
                                    }
                                    TankType::Clean => {
                                        available = Some(self.is_tank_available(TankType::Clean))
                                    }
                                    TankType::CleanRinse => {
                                        available =
                                            Some(self.is_tank_available(TankType::CleanRinse))
                                    }
                                    TankType::DIRinse => {
                                        available = Some(self.is_tank_available(TankType::DIRinse))
                                    }
                                    TankType::Cool => {
                                        available = Some(self.is_tank_available(TankType::Cool))
                                    }
                                    TankType::Oven => {
                                        available = Some(self.is_tank_available(TankType::Oven))
                                    }
                                    TankType::Grey => {
                                        available = Some(self.is_tank_available(TankType::Grey))
                                    }
                                    TankType::Primer => {
                                        available = Some(self.is_tank_available(TankType::Primer))
                                    }
                                    TankType::Lacquer => {
                                        available = Some(self.is_tank_available(TankType::Lacquer))
                                    }
                                }
                                if available.unwrap() {
                                    println!(
                                        "Robot will move basket:{} from step:{}",
                                        basket.job_id,
                                        basket.current_step.unwrap()
                                    );
                                    self.robot.status = RobotStatus::WillMove;
                                    self.tank_pickup = Some(i);
                                    self.position_pickup = Some(j);
                                }
                            }
                        }
                        if self.robot.status != RobotStatus::Idle {
                            break;
                        }
                    }
                    if self.robot.status != RobotStatus::Idle {
                        break;
                    }
                }
            }

            // If robot is idel, try loading a new job
            if self.robot.is_idle() {
                if !self.baskets.is_empty() {
                    let b = self.baskets.clone().pop().unwrap(); // Using clone here because we don't want to pop off the original stack
                    let available: Option<bool>;
                    // if the next step is no unload, then check if the next tank is available to move to
                    match b.recipe.steps[b.next_step.unwrap()].tank_type {
                        TankType::Etch => available = Some(self.is_tank_available(TankType::Etch)),
                        TankType::EtchRinse => {
                            available = Some(self.is_tank_available(TankType::EtchRinse))
                        }
                        TankType::Clean => {
                            available = Some(self.is_tank_available(TankType::Clean))
                        }
                        TankType::CleanRinse => {
                            available = Some(self.is_tank_available(TankType::CleanRinse))
                        }
                        TankType::DIRinse => {
                            available = Some(self.is_tank_available(TankType::DIRinse))
                        }
                        TankType::Cool => available = Some(self.is_tank_available(TankType::Cool)),
                        TankType::Oven => available = Some(self.is_tank_available(TankType::Oven)),
                        TankType::Grey => available = Some(self.is_tank_available(TankType::Grey)),
                        TankType::Primer => {
                            available = Some(self.is_tank_available(TankType::Primer))
                        }
                        TankType::Lacquer => {
                            available = Some(self.is_tank_available(TankType::Lacquer))
                        }
                    }
                    if available.unwrap() {
                        let mut b = self.baskets.pop().unwrap();
                        b.status = BasketStatus::Moving;
                        println!("Basket {} begins loading", b.job_id);
                        self.robot.basket = Some(b);
                        self.robot.status = RobotStatus::Moving;
                        self.robot.time_remaining = 20;
                    } else {
                        //println!("Basket {} cannot begin loading", b.job_id);
                    }
                }
            }

            if self.robot.status == RobotStatus::WillMove {
                let mut b = self.tanks[self.tank_pickup.unwrap()]
                    .occupants
                    .remove(self.position_pickup.unwrap());
                b.status = BasketStatus::Moving;
                self.robot.basket = Some(b);
                self.robot.status = RobotStatus::Moving;
                self.robot.time_remaining = 20;
            }

            if self.robot.status == RobotStatus::DropOff {
                let mut b = self.robot.dropoff_basket();
                if b.is_final_step() {
                    b.current_step = None;
                    self.robot.basket = None;
                    b.status = BasketStatus::Unloaded;
                    self.robot.status = RobotStatus::Idle;
                    println!("Job Number {} unloaded", b.job_id);
                    self.unloaded_baskets.push(b);
                } else {
                    b.current_step = b.next_step;
                    b.set_next_step();
                    self.robot.basket = None;
                    self.robot.status = RobotStatus::Idle;
                    self.drop_off_basket(b);
                }
            }
            self.tick();
        }
    }

    fn is_empty(&self) -> bool {
        if self.baskets_in_machine() > 0 {
            false
        } else {
            true
        }
    }

    fn baskets_in_machine(&self) -> usize {
        let mut basket_count = 0;
        for tank in self.tanks.iter() {
            for _ in tank.occupants.iter() {
                basket_count += 1;
            }
        }
        if !self.robot.is_idle() {
            basket_count += 1;
        }
        basket_count
    }

    fn is_stuck(&self) -> bool {
        self.elapsed_seconds > 60 * 60 * 24
    }

    fn tick(&mut self) {
        self.robot.tick();
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
        self.elapsed_seconds += 1;
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

#[derive(PartialEq, Debug, Clone)]
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
