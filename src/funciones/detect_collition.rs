use bevy::prelude::*;

use crate::estructuras::goat::Goat;
use crate::estructuras::plants::Plant;
use crate::Collition; 


pub fn check_collisions(
    goat_query: Query<&Transform, With<Goat>>,
    plant_query: Query<&Transform, With<Plant>>,
    mut collision_events: EventWriter<Collition>,
) {
    let goat_transform = match goat_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for plant_transform in plant_query.iter() {
        if is_colliding(goat_transform, plant_transform) {
            collision_events.send(Collition {
                plant_position: plant_transform.translation,
            });
        }
    }
}

fn is_colliding(goat_transform: &Transform, plant_transform: &Transform) -> bool {
    let goat_size = Vec2::new(100.0, 100.0);
    let plant_size = Vec2::new(55.0, 55.0);
    
    let goat_pos = goat_transform.translation;
    let plant_pos = plant_transform.translation;
    
    (goat_pos.x - goat_size.x / 2.0) < (plant_pos.x + plant_size.x / 2.0) 
        && (goat_pos.x + goat_size.x / 2.0) > (plant_pos.x - plant_size.x / 2.0)
        && (goat_pos.y - goat_size.y / 2.0) < (plant_pos.y + plant_size.y / 2.0)
        && (goat_pos.y + goat_size.y / 2.0) > (plant_pos.y - plant_size.y / 2.0)
}

