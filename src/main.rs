pub mod machine;
use crate::machine::basket::Basket;
use crate::machine::recipe::Recipe;

use crate::machine::Machine;

fn main() {
    let mut machine: Machine = Machine::umbra60_3t();
    let mut baskets: Vec<Basket> = Vec::new();
    let number_of_jobs = 5;
    let mut job_id: u64 = 0;

    loop {
        if job_id == number_of_jobs {
            break;
        }

        baskets.insert(0, Basket::new(Recipe::grey(), job_id));
        job_id += 1;
    }

    machine.load(baskets);
    machine.run();
}
