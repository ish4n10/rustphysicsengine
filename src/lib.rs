use bevy::prelude::*;

mod entity;
mod components;

pub use entity::*;
pub use components::*;


pub const DELTA_TIME: f32 = 1. / 60.;


fn stimulate(mut query: Query<(&mut Pos, &mut PrevPos, &Mass)>, gravity: Res<Gravity>) {
    for (mut pos, mut prev_pos, mass) in query.iter_mut() {
        let gravity_force = mass.0 * gravity.0;
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME + DELTA_TIME * gravity_force;
        prev_pos.0 = pos.0;
        pos.0 = pos.0 + velocity * DELTA_TIME;

    }
}

fn sync_transformer(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}

#[derive(Debug, Default)]
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
        .add_systems(Update, stimulate)
            .add_systems(Update, sync_transformer);
        }
}