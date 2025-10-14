use bevy::prelude::*;
use crate::estructuras::plants::Plant;  
use crate::estructuras::goat::Goat;



pub fn plant_color_changes(
    goat_query: Query<&Transform, With<Goat>>,
    mut plant_query: Query<(&mut Sprite, &Transform), With<Plant>>,
) {
    let goat_transform = match goat_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return, 
    };

    for (mut sprite, plant_transform) in plant_query.iter_mut() {
        if is_colliding(goat_transform, plant_transform) {
            sprite.color = Color::rgb(1.0, 1.0, 0.0); 
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

