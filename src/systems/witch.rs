use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::space::Witch;

#[derive(SystemDesc)]
pub struct MoveWitchSystem;

impl<'s> System<'s> for MoveWitchSystem {
    type SystemData = (
        ReadStorage<'s, Witch>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (witches, mut locals, time): Self::SystemData) {
        for (witch, local) in (&witches, &mut locals).join() {
            local.prepend_translation_x(time.delta_seconds().cos() * 32.0);
            local.prepend_translation_y(time.delta_seconds().sin() * 32.0);
        }
    }
}
