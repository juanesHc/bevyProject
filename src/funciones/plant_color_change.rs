use bevy::prelude::*;
use crate::estructuras::plants::Plant;  
use crate::estructuras::collition::Collition;  

pub fn plant_color_changes(
    mut collision_events: EventReader<Collition>,
    mut plant_query: Query<(&mut Sprite, &Plant)>,
) {
    for event in collision_events.iter() {
        for (mut sprite, plant) in plant_query.iter_mut() {
            if plant.id == event.plant_id {
 
                sprite.color = Color::rgb(1.0, 1.0, 0.0);
            }
        }
    }
}