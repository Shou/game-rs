use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::space::{Boi, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct BoiSystem;

impl<'s> System<'s> for BoiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Boi>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, _boi, input): Self::SystemData) {
        let mut i = 0;
        for transform in (&mut transforms).join() {
            if i == 0 {
                let xy = || {
                    let x = input.axis_value("x")?;
                    let y = input.axis_value("y")?;
                    Some((x, y))
                };
                if let Some((x, y)) = xy() {
                    let old_y = transform.translation().y;
                    let old_x = transform.translation().x;
                    transform.set_translation_x(old_x + x * 4.0);
                    transform.set_translation_y(old_y + y * 4.0);
                    println!("Moving player {} x {}", old_x, old_y)
                }
            }
            i += 1;
        }
    }
}
