use std::collections::HashMap;



use oort_api::prelude::Vec2;
use oort_api::prelude::Vec2Extras;
use oort_api::prelude::current_tick;
use oort_api::prelude::maths_rs::prelude::Cast;
use oort_api::prelude::oorandom;

use super::target_information::{self, Target};
pub struct TickInformation {
    pub my_position: Vec2,
    pub my_velocity: Vec2,
    pub my_acceleration: Vec2,
}

pub struct InformationBank {
    pub tick_storage: Vec<TickInformation>,
    pub targets: HashMap<u32, target_information::Target>,
    target_making_history: Vec<u32>
}



impl InformationBank {
    pub fn new() -> InformationBank {
        InformationBank {
            tick_storage: vec![],
            targets: HashMap::new(),
            target_making_history: vec![],

        }
    }

    pub fn make_target(&mut self,target_position:Vec2) -> &mut Target {
        let target_object = Target::new();

        let mut rng = oorandom::Rand32::new(current_tick().as_u64()*100*target_position.length().as_u64());
        let id = rng.rand_u32();

        self.targets.insert(id, target_object);

        self.target_making_history.push(id);

        self.targets.get_mut(&id).unwrap()
    }

    pub fn get_last_target_making(&mut self) -> &mut Target {
        let id = self.target_making_history.last().unwrap();

        self.targets.get_mut(id).unwrap()

    }

    pub fn get_mut_target_with_id(&mut self,id:&u32) ->  Option<&mut Target> {
        self.targets.get_mut(id)
    }

    pub fn get_id_last_target_making(&self) -> &u32 {
        self.target_making_history.last().unwrap()
    }
}

/*





last_target_velocities: Vec<Vec2>,
    predicted_target_positions: Vec<Vec2>,
    predicted_target_positions_for_drawing: Vec<Vec2>,
    target_positions: Vec<Vec2>,
    information_storage: Vec<tick_information>,

*/