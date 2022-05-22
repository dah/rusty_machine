#[derive(PartialEq, Debug)]
#[derive(Clone)]
enum TankType { 
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


#[derive(PartialEq, Debug)]
#[derive(Clone)]
enum BasketStatus {
    WaitingToLoad,
    Unloaded,
    Processing,
    WaitingToMove,
    Moving,
}

#[derive(PartialEq)]
#[derive(Clone)]
enum RobotStatus {
    Idle,
    WillMove,
    Moving,
    DropOff,
}

#[derive(Clone, Debug)]
struct Step {
    tank_type: TankType,
    process_time: usize,
    actual_time: usize,
    remaining_time: usize,
}

impl Step {
    fn tick(&mut self) -> bool {
        self.actual_time += 1;
        if self.remaining_time > 0 {
            self.remaining_time -= 1;
            if self.remaining_time == 0 {
                return true;
            }
        } 
        return false;
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct Recipe {
    steps: Vec<Step>,
}

impl Recipe {
    fn new(steps: Vec<Step>) -> Self { Self { steps } }

    fn clean() -> Self {
        let steps = vec![
            Step {
                tank_type: TankType::Etch,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
            },
            Step {
                tank_type: TankType::EtchRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
            },
            Step {
                tank_type: TankType::Clean,
                process_time: 240,
                actual_time: 0,
                remaining_time: 240,
            },
            Step {
                tank_type: TankType::CleanRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
            },
            Step {
                tank_type: TankType::DIRinse,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
            },
        ];
        Self::new(steps)
    }
        fn grey() -> Self {
        let steps = vec![
            Step {
                tank_type: TankType::Etch,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
            },
            Step {
                tank_type: TankType::EtchRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
            },
            Step {
                tank_type: TankType::Clean,
                process_time: 240,
                actual_time: 0,
                remaining_time: 240,
            },
            Step {
                tank_type: TankType::CleanRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
            },
            Step {
                tank_type: TankType::DIRinse,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Grey,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 1200,
                actual_time: 0,
                remaining_time: 1200,
            },
            Step {
                tank_type: TankType::Primer,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Cool,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Lacquer,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
            },
        ];
        Self::new(steps)
    }
}

struct Tank { 
    tank_type: TankType, 
    occupants: Vec<Basket>,
    max_occupancy: u8,
} 

struct Machine { 
    tanks: Vec<Tank>,
}

impl Machine {
    fn new(tanks: Vec<Tank>) -> Self { Self { tanks } }

    fn tick(&mut self) {
        let mut tank_indices: Vec<usize> = Vec::new();
        let mut occupant_indices: Vec<usize> = Vec::new();
        for (i, tank) in self.tanks.iter().enumerate() {
            for (j, basket) in tank.occupants.iter().enumerate() {
                if basket.status == BasketStatus::Processing || basket.status == BasketStatus::WaitingToMove {
                    tank_indices.push(i);
                    occupant_indices.push(j);
                }
            }
        }
        loop {
            if tank_indices.len() == 0 {
                break;
            }
            let b = &mut self.tanks[tank_indices.pop().unwrap()].occupants[occupant_indices.pop().unwrap()];
            if b.recipe.steps[b.current_step.clone().unwrap()].tick() {
                b.status = BasketStatus::WaitingToMove;
            }

        }
    }

    fn umbra60_3t() -> Self {
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
            } 
        ];
        Self::new(tanks)
    }
    fn is_tank_available(&self, tank_name: TankType) -> bool {
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
    fn drop_off_basket(&mut self, mut basket: Basket) {
        //basket.current_step += 1;
        basket.status = BasketStatus::Processing;
        for tank in self.tanks.iter_mut() {
            if tank.tank_type == basket.recipe.steps[basket.current_step.clone().unwrap()].tank_type {
                tank.occupants.push(basket);
                break;
            }
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct Basket {
    recipe: Recipe,
    job_id: u64,
    status: BasketStatus,
    current_step: Option<usize>,
    next_step: Option<usize>,
}
#[derive(Clone)]
struct Robot {
    status: RobotStatus,
    time_remaining: u8,
    basket: Option<Basket>,
}

impl Robot {
    fn new() -> Self { 
        let status = RobotStatus::Idle;
        let time_remaining = 0;
        let basket = None;
        Self { status, time_remaining, basket } 
    }

    fn tick(&mut self) {
        if self.time_remaining > 0 {
            self.time_remaining -= 1;
            if self.time_remaining < 1 {
                self.status = RobotStatus::DropOff;
                println!("robot dropoff");
            }
        }
    }
}

impl Basket {
    fn new(recipe: Recipe, job_id: u64, status: BasketStatus) -> Self {
        let current_step = None;
        let next_step = Some(0); 
        Self { recipe, job_id, status, current_step, next_step } 
    }

    fn is_final_step(&self) -> bool {
        //self.current_step + 1 == self.recipe.steps.len().try_into().unwrap()
        self.next_step == None
    }

    fn set_next_step(&mut self) {
        let at_last_step: bool = self.current_step.clone().unwrap() + 1 == self.recipe.steps.len().try_into().unwrap();
        if at_last_step {
            self.next_step = None;
        } else {
            self.next_step = Some(self.next_step.clone().unwrap() + 1);
        }
    }
    fn print_waiting_steps(&self) {
        let mut did_print_something: bool = false;
        for step in self.recipe.steps.iter() {
            if step.actual_time != step.process_time {
                println!("      Stuck at {:?} for {} extra seconds.", step.tank_type, step.actual_time - step.process_time);
                did_print_something = true;
            }
        }
        if !did_print_something {
            println!("      Basket Processed Perfectly")
        }
    }
}

fn main() { 
    println!("Hello, world!"); 
    let mut tank_pickup: Option<usize> = None;
    let mut position_pickup: Option<usize> = None;

    let mut robot = Robot::new();
    let mut umbra60_3t: Machine = Machine::umbra60_3t();
    let mut baskets: Vec<Basket> = Vec::new();
    let mut unloaded_baskets: Vec<Basket> = Vec::new();
    let number_of_jobs = 7;
    let mut job_id: u64 = 0;
    let mut elapsed_seconds: u64 = 0;
    loop {
        if job_id == number_of_jobs {
            break;
        }

        //baskets.insert(0, Basket::new(Recipe::clean(), job_id, BasketStatus::WaitingToLoad));
        baskets.insert(0, Basket::new(Recipe::grey(), job_id, BasketStatus::WaitingToLoad));
        job_id += 1;
    }

    loop {
        if unloaded_baskets.len() == number_of_jobs.try_into().unwrap() {
            println!("\nFinished Unloading All Baskets\n");
            for basket in unloaded_baskets.iter() {
                println!("***** Basket {}", basket.job_id);
                basket.print_waiting_steps();
                println!("\n");
            }
            
            break;
        }
        // if >0 jobs in machine && all jobs @ 0 time && robot not moving then break would be better
        if elapsed_seconds > 60 * 60 * 24 {
            println!("Robot Stuck: Could not complete after in 1 simulated day.");
            println!("Unloaded {} out of {} baskets.", unloaded_baskets.len(), number_of_jobs);
            break;
        }
        if robot.status == RobotStatus::Idle {

            //Scan for a basket that is ready to move in the machine
            for (i, tank) in umbra60_3t.tanks.iter().enumerate() {
                for (j, basket) in tank.occupants.iter().enumerate() {
                    if let BasketStatus::WaitingToMove = basket.status {

                        // first check if that basket needs to unload
                        if basket.is_final_step() {
                            println!("Robot will unload a basket");
                            robot.status = RobotStatus::WillMove;
                            tank_pickup = Some(i);
                            position_pickup = Some(j);
                        } else {
                            let available: Option<bool>;
                            // if the next step is no unload, then check if the next tank is available to move to
                            match basket.recipe.steps[basket.next_step.unwrap()].tank_type {
                                TankType::Etch => available = Some(umbra60_3t.is_tank_available(TankType::Etch)),
                                TankType::EtchRinse => available = Some(umbra60_3t.is_tank_available(TankType::EtchRinse)),
                                TankType::Clean => available = Some(umbra60_3t.is_tank_available(TankType::Clean)),
                                TankType::CleanRinse => available = Some(umbra60_3t.is_tank_available(TankType::CleanRinse)),
                                TankType::DIRinse => available = Some(umbra60_3t.is_tank_available(TankType::DIRinse)),
                                TankType::Cool => available = Some(umbra60_3t.is_tank_available(TankType::Cool)),
                                TankType::Oven => available = Some(umbra60_3t.is_tank_available(TankType::Oven)),
                                TankType::Grey => available = Some(umbra60_3t.is_tank_available(TankType::Grey)),
                                TankType::Primer => available = Some(umbra60_3t.is_tank_available(TankType::Primer)),
                                TankType::Lacquer => available = Some(umbra60_3t.is_tank_available(TankType::Lacquer)),
                            }
                            if available.unwrap() {
                                println!("robot will move a basket");
                                robot.status = RobotStatus::WillMove;
                                tank_pickup = Some(i);
                                position_pickup = Some(j);
                            }
                        }
                    }
                    if robot.status != RobotStatus::Idle { break; }
                }
                if robot.status != RobotStatus::Idle { break; }
            }

            //After scanning unsuccessfully, try to load a new job
            if robot.status == RobotStatus::Idle {
                if !baskets.is_empty() {
                    let b = baskets.clone().pop().unwrap();
                    let available: Option<bool>;
                    // if the next step is no unload, then check if the next tank is available to move to
                    match b.recipe.steps[b.next_step.unwrap()].tank_type {
                        TankType::Etch => available = Some(umbra60_3t.is_tank_available(TankType::Etch)),
                        TankType::EtchRinse => available = Some(umbra60_3t.is_tank_available(TankType::EtchRinse)),
                        TankType::Clean => available = Some(umbra60_3t.is_tank_available(TankType::Clean)),
                        TankType::CleanRinse => available = Some(umbra60_3t.is_tank_available(TankType::CleanRinse)),
                        TankType::DIRinse => available = Some(umbra60_3t.is_tank_available(TankType::DIRinse)),
                        TankType::Cool => available = Some(umbra60_3t.is_tank_available(TankType::Cool)),
                        TankType::Oven => available = Some(umbra60_3t.is_tank_available(TankType::Oven)),
                        TankType::Grey => available = Some(umbra60_3t.is_tank_available(TankType::Grey)),
                        TankType::Primer => available = Some(umbra60_3t.is_tank_available(TankType::Primer)),
                        TankType::Lacquer => available = Some(umbra60_3t.is_tank_available(TankType::Lacquer)),
                    }
                    if available.unwrap() {
                        let mut b = baskets.pop().unwrap();
                        b.status = BasketStatus::Moving;
                        println!("Basket {} begins loading", b.job_id);
                        robot.basket = Some(b);
                        robot.status = RobotStatus::Moving;
                        robot.time_remaining = 20;
                    } else {
                        println!("Basket {} cannot begin loading", b.job_id);
                    }
                }
            }
        }

        if robot.status == RobotStatus::WillMove {
            let mut b = umbra60_3t.tanks[tank_pickup.unwrap()].occupants.remove(position_pickup.unwrap());
            b.status = BasketStatus::Moving;
            robot.basket = Some(b);
            robot.status = RobotStatus::Moving;
            robot.time_remaining = 20;
        }

        if robot.status == RobotStatus::DropOff {
            let mut b = robot.basket.clone().unwrap();
            if b.is_final_step() {
                b.current_step = None;
                robot.basket = None;
                b.status = BasketStatus::Unloaded;
                robot.status = RobotStatus::Idle;
                println!("Job Number {} unloaded", b.job_id);
                unloaded_baskets.push(b);

            } else {
                b.current_step = b.next_step.clone();
                b.set_next_step();
                robot.basket = None;
                robot.status = RobotStatus::Idle;
                umbra60_3t.drop_off_basket(b);
            }
        }

        robot.tick();
        umbra60_3t.tick();
        elapsed_seconds += 1;
    }

}



