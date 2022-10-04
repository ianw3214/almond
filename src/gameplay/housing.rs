use specs::prelude::*;

use crate::components::*;

use rand::prelude::*;

pub struct HousingSystem;

impl<'a> System<'a> for HousingSystem {
    type SystemData = (
        WriteStorage<'a, Tenant>,
        WriteStorage<'a, Housing>,
        WriteStorage<'a, Brain>,
        // List of all entities
        Entities<'a>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        for (tenant, brain) in (&mut data.0, &mut data.2).join() {
            // look for a housing if homeless
            if let None = tenant.house {
                for (house, entity) in (&mut data.1, &data.3).join() {
                    if house.num_tenants < house.capacity {
                        tenant.house = Some(entity);
                        house.num_tenants = house.num_tenants + 1;
                        break;
                    }
                }
            }
            // let the brain occasionally return home
            // TODO: This should probably be handled less randomly
            //  e.g. a return home at night system
            //  a time system will have to be implmented to make it work
            if let Some(house) = tenant.house {
                if let Task::IDLE = brain.task{
                    let mut rng = thread_rng();
                    let index : i32 = rng.gen_range(0..400);
                    if index == 0 {
                        brain.task = Task::HOME(house);
                    }
                }
            }
        }
    }
}