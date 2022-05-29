use crate::machine::basket::Basket;
use crate::machine::RobotStatus;

#[derive(Clone)]
pub struct Robot {
    pub status: RobotStatus,
    pub time_remaining: u8,
    pub basket: Option<Basket>,
}

impl Robot {
    pub fn new() -> Self {
        let status = RobotStatus::Idle;
        let time_remaining = 0;
        let basket = None;
        Self {
            status,
            time_remaining,
            basket,
        }
    }

    pub fn tick(&mut self) {
        if self.time_remaining > 0 {
            self.time_remaining -= 1;
            if self.time_remaining < 1 {
                self.status = RobotStatus::DropOff;
                //println!("robot dropoff");
            }
        }
    }

    pub fn is_idle(&self) -> bool {
        self.status == RobotStatus::Idle
    }

    pub fn dropoff_basket(&mut self) -> Basket {
        if self.basket.is_none() {
            panic!("Tried to drop off nonexistent basket");
        } else {
            let b = self.basket.as_ref().unwrap().clone();
            //self.basket = None;
            return b;
        }
    }
}
