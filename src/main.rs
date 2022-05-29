pub mod machine;
use crate::machine::basket::Basket;
use crate::machine::recipe::Recipe;
use crate::machine::robot::Robot;
use crate::machine::BasketStatus;
use crate::machine::Machine;
use crate::machine::RobotStatus;
use crate::machine::TankType;
use tabled::Table;

fn main() {
    println!("Hello, world!");
    let mut tank_pickup: Option<usize> = None;
    let mut position_pickup: Option<usize> = None;

    let mut robot = Robot::new();
    let mut umbra60_3t: Machine = Machine::umbra60_3t();
    let mut baskets: Vec<Basket> = Vec::new();
    let mut unloaded_baskets: Vec<Basket> = Vec::new();
    let number_of_jobs = 3;
    let mut job_id: u64 = 0;
    let mut elapsed_seconds: u64 = 0;
    loop {
        if job_id == number_of_jobs {
            break;
        }

        //baskets.insert(0, Basket::new(Recipe::clean(), job_id, BasketStatus::WaitingToLoad));
        baskets.insert(
            0,
            Basket::new(Recipe::grey(), job_id),
        );
        job_id += 1;
    }

    loop {
        if unloaded_baskets.len() == number_of_jobs.try_into().unwrap() {
            println!("\nFinished Unloading All Baskets\n");
            for basket in unloaded_baskets.iter() {
                println!("***** Basket {}", basket.job_id);
                //basket.print_waiting_steps();
                let steps = &basket.recipe.steps;
                let table = Table::new(steps).to_string();
                println!("{}", table);
                println!("\n");
            }

            break;
        }
        // if >0 jobs in machine && all jobs @ 0 time && robot not moving then break would be better
        if elapsed_seconds > 60 * 60 * 24 {
            println!("Robot Stuck: Could not complete after in 1 simulated day.");
            println!(
                "Unloaded {} out of {} baskets.",
                unloaded_baskets.len(),
                number_of_jobs
            );
            break;
        }
        if robot.status == RobotStatus::Idle {
            //Scan for a basket that is ready to move in the machine
            for (i, tank) in umbra60_3t.tanks.iter().enumerate() {
                for (j, basket) in tank.occupants.iter().enumerate() {
                    if let BasketStatus::WaitingToMove = basket.status {
                        // first check if that basket needs to unload
                        if basket.is_final_step() {
                            println!("Robot will unload basket {}", basket.job_id);
                            robot.status = RobotStatus::WillMove;
                            tank_pickup = Some(i);
                            position_pickup = Some(j);
                        } else {
                            let available: Option<bool>;
                            // if the next step is not unload, then check if the next tank is available to move to
                            match basket.recipe.steps[basket.next_step.unwrap()].tank_type {
                                TankType::Etch => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Etch))
                                }
                                TankType::EtchRinse => {
                                    available =
                                        Some(umbra60_3t.is_tank_available(TankType::EtchRinse))
                                }
                                TankType::Clean => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Clean))
                                }
                                TankType::CleanRinse => {
                                    available =
                                        Some(umbra60_3t.is_tank_available(TankType::CleanRinse))
                                }
                                TankType::DIRinse => {
                                    available =
                                        Some(umbra60_3t.is_tank_available(TankType::DIRinse))
                                }
                                TankType::Cool => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Cool))
                                }
                                TankType::Oven => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Oven))
                                }
                                TankType::Grey => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Grey))
                                }
                                TankType::Primer => {
                                    available = Some(umbra60_3t.is_tank_available(TankType::Primer))
                                }
                                TankType::Lacquer => {
                                    available =
                                        Some(umbra60_3t.is_tank_available(TankType::Lacquer))
                                }
                            }
                            if available.unwrap() {
                                println!(
                                    "Robot will move basket:{} from step:{}",
                                    basket.job_id,
                                    basket.current_step.unwrap()
                                );
                                robot.status = RobotStatus::WillMove;
                                tank_pickup = Some(i);
                                position_pickup = Some(j);
                            }
                        }
                    }
                    if robot.status != RobotStatus::Idle {
                        break;
                    }
                }
                if robot.status != RobotStatus::Idle {
                    break;
                }
            }

            //After scanning unsuccessfully, try to load a new job
            if robot.status == RobotStatus::Idle {
                if !baskets.is_empty() {
                    let b = baskets.clone().pop().unwrap(); // Using clone here because we don't want to pop off the original stack
                    let available: Option<bool>;
                    // if the next step is no unload, then check if the next tank is available to move to
                    match b.recipe.steps[b.next_step.unwrap()].tank_type {
                        TankType::Etch => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Etch))
                        }
                        TankType::EtchRinse => {
                            available = Some(umbra60_3t.is_tank_available(TankType::EtchRinse))
                        }
                        TankType::Clean => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Clean))
                        }
                        TankType::CleanRinse => {
                            available = Some(umbra60_3t.is_tank_available(TankType::CleanRinse))
                        }
                        TankType::DIRinse => {
                            available = Some(umbra60_3t.is_tank_available(TankType::DIRinse))
                        }
                        TankType::Cool => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Cool))
                        }
                        TankType::Oven => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Oven))
                        }
                        TankType::Grey => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Grey))
                        }
                        TankType::Primer => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Primer))
                        }
                        TankType::Lacquer => {
                            available = Some(umbra60_3t.is_tank_available(TankType::Lacquer))
                        }
                    }
                    if available.unwrap() {
                        let mut b = baskets.pop().unwrap();
                        b.status = BasketStatus::Moving;
                        println!("Basket {} begins loading", b.job_id);
                        robot.basket = Some(b);
                        robot.status = RobotStatus::Moving;
                        robot.time_remaining = 20;
                    } else {
                        //println!("Basket {} cannot begin loading", b.job_id);
                    }
                }
            }
        }

        if robot.status == RobotStatus::WillMove {
            let mut b = umbra60_3t.tanks[tank_pickup.unwrap()]
                .occupants
                .remove(position_pickup.unwrap());
            b.status = BasketStatus::Moving;
            robot.basket = Some(b);
            robot.status = RobotStatus::Moving;
            robot.time_remaining = 20;
        }

        if robot.status == RobotStatus::DropOff {
            let mut b = robot.basket.unwrap();
            if b.is_final_step() {
                b.current_step = None;
                robot.basket = None;
                b.status = BasketStatus::Unloaded;
                robot.status = RobotStatus::Idle;
                println!("Job Number {} unloaded", b.job_id);
                unloaded_baskets.push(b);
            } else {
                b.current_step = b.next_step;
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
