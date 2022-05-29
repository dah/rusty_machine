use crate::machine::step::Step;
use crate::machine::TankType;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub steps: Vec<Step>,
}

impl Recipe {
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }
}

impl Recipe {
    pub fn clean() -> Self {
        let steps = vec![
            Step {
                tank_type: TankType::Etch,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::EtchRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Clean,
                process_time: 240,
                actual_time: 0,
                remaining_time: 240,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::CleanRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::DIRinse,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
                extra_time: 0,
            },
        ];
        Self::new(steps)
    }
    pub fn grey() -> Self {
        let steps = vec![
            Step {
                tank_type: TankType::Etch,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::EtchRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Clean,
                process_time: 240,
                actual_time: 0,
                remaining_time: 240,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::CleanRinse,
                process_time: 180,
                actual_time: 0,
                remaining_time: 180,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::DIRinse,
                process_time: 500,
                actual_time: 0,
                remaining_time: 500,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Grey,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 1200,
                actual_time: 0,
                remaining_time: 1200,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Primer,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Cool,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Lacquer,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
            Step {
                tank_type: TankType::Oven,
                process_time: 300,
                actual_time: 0,
                remaining_time: 300,
                extra_time: 0,
            },
        ];
        Self::new(steps)
    }
}
