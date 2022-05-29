use crate::machine::recipe::Recipe;
use crate::machine::BasketStatus;

#[derive(Debug, Clone)]
pub struct Basket {
    pub recipe: Recipe,
    pub job_id: u64,
    pub status: BasketStatus,
    pub current_step: Option<usize>,
    pub next_step: Option<usize>,
}

impl Basket {
    pub fn new(recipe: Recipe, job_id: u64) -> Self {
        let current_step = None;
        let next_step = Some(0);
        let status = BasketStatus::WaitingToLoad;
        Self {
            recipe,
            job_id,
            status,
            current_step,
            next_step,
        }
    }

    pub fn is_final_step(&self) -> bool {
        //self.current_step + 1 == self.recipe.steps.len().try_into().unwrap()
        self.next_step == None
    }

    pub fn set_next_step(&mut self) {
        let at_last_step: bool =
            self.current_step.unwrap() + 1 == self.recipe.steps.len().try_into().unwrap();
        if at_last_step {
            self.next_step = None;
        } else {
            self.next_step = Some(self.next_step.unwrap() + 1);
        }
    }
    pub fn print_waiting_steps(&self) {
        let mut did_print_something: bool = false;
        for step in self.recipe.steps.iter() {
            if step.actual_time != step.process_time {
                println!(
                    "      Stuck at {:?} for {} extra seconds.",
                    step.tank_type,
                    step.actual_time - step.process_time
                );
                did_print_something = true;
            }
        }
        if !did_print_something {
            println!("      Basket Processed Perfectly")
        }
    }
}
