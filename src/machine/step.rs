use crate::machine::TankType;
use std::fmt;
use tabled::Tabled;

#[derive(Clone, Debug, Tabled)]
pub struct Step {
    pub tank_type: TankType,
    pub process_time: usize,
    pub actual_time: usize,
    pub remaining_time: usize,
    pub extra_time: usize,
}

impl Step {
    pub fn tick(&mut self) -> bool {
        self.actual_time += 1;
        if self.remaining_time > 0 {
            self.remaining_time -= 1;
            if self.remaining_time == 0 {
                return true;
            }
        } else {
            self.extra_time += 1;
        }
        return false;
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.tank_type,
            self.process_time,
            self.actual_time,
            self.remaining_time,
            self.extra_time
        )
    }
}
