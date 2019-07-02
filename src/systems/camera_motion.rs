use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Marine, SubjectTag},
    resources::Context,
};

pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
        ReadStorage<'s, Marine>,
        ReadStorage<'s, SubjectTag>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (marines, subject_tags, mut transforms, context): Self::SystemData) {
        let mut marine_x = 0.;
        let map_width = context.map_width;
        let background_width = context.background_width;

        for (_marine, transform) in (&marines, &transforms).join() {
            marine_x = transform.translation().x.as_f32();
        }

        for (_subject_tag, transform) in (&subject_tags, &mut transforms).join() {
            if marine_x >= background_width && marine_x <= map_width - background_width {
                transform.set_translation_x(marine_x);            
            }
        }
    }
}
