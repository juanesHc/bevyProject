use bevy::prelude::*;
use crate::estructuras::plants::Plant;  
use crate::Collition;

pub fn plant_color_changes(
    mut collision_events: EventReader<Collition>,
    mut plant_query: Query<(&mut Sprite, &Transform), With<Plant>>,
) {
    for event in collision_events.iter() {
        for (mut sprite, plant_transform) in plant_query.iter_mut() {
            if plant_transform.translation.distance(event.plant_position) < 1.0 {
                sprite.color = Color::rgb(1.0, 1.0, 0.0);
            }
        }
    }
}
