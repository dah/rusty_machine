use crate::machine::TankType;

#[derive(Clone, Debug)]
pub struct Step {
    pub tank_type: TankType,
    pub process_time: usize,
    pub actual_time: usize,
    pub remaining_time: usize,
}

impl Step {
    pub fn tick(&mut self) -> bool {
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
