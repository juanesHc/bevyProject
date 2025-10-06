use bevy::prelude::*;

use crate::estructuras::goat::Goat;
use crate::estructuras::plants::Plant;
use crate::Collition; 

pub fn collision_detection_system(
    goat_query: Query<&Transform, With<Goat>>,
    plant_query: Query<(&Transform, &Plant)>,
    mut collision_events: EventWriter<Collition>,
) {

    let goat_transform = match goat_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return, 
    };


    for (plant_transform, plant) in plant_query.iter() {
        if is_colliding(goat_transform, plant_transform) {
            collision_events.send(Collition {
                plant_id: plant.id,
            });
        }
    }
}

fn is_colliding(goat_transform: &Transform, plant_transform: &Transform) -> bool {

    let goat_size = Vec2::new(100.0, 100.0);
    
    let plant_size = Vec2::new(55.0, 55.0); 
    
    let goat_pos = goat_transform.translation;
    let plant_pos = plant_transform.translation;

    let collision_x = (goat_pos.x - goat_size.x / 2.0) < (plant_pos.x + plant_size.x / 2.0) 
        && (goat_pos.x + goat_size.x / 2.0) > (plant_pos.x - plant_size.x / 2.0);
    
    let collision_y = (goat_pos.y - goat_size.y / 2.0) < (plant_pos.y + plant_size.y / 2.0) 
        && (goat_pos.y + goat_size.y / 2.0) > (plant_pos.y - plant_size.y / 2.0);
    
    collision_x && collision_y
}

pub fn teleport_to_nearest_plant(
    input: Res<Input<KeyCode>>,
    mut goat_query: Query<&mut Transform, With<Goat>>,
    plant_query: Query<&Transform, (With<Plant>, Without<Goat>)>,
) {
    if input.just_pressed(KeyCode::E) {
        let mut goat_transform = match goat_query.get_single_mut() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let goat_position = goat_transform.translation;

        let nearest_plant = plant_query.iter()
            .min_by_key(|plant_transform| {
                let distance = (plant_transform.translation - goat_position).length_squared();
                (distance * 1000.0) as i32 
            });

        if let Some(nearest_plant) = nearest_plant {
            goat_transform.translation = nearest_plant.translation;
        }
    }
}