use bevy::prelude::*;

use crate::estructuras::{goat::Goat, plants::Plant};

pub fn movement(
    input:Res<Input<KeyCode>>, 
    mut query: Query<&mut Transform, 
    With<Goat>>) {
        
    if input.pressed(KeyCode::A){
            for mut transform in query.iter_mut() {
        transform.translation.x -= 5.0; 
    }
    }

    if input.pressed(KeyCode::D){
    for mut transform in query.iter_mut() {
        transform.translation.x += 5.0;
    }
    }
        if input.pressed(KeyCode::S){
            for mut transform in query.iter_mut() {
        transform.translation.y -= 5.0; 
    }
    }

    if input.pressed(KeyCode::W){
    for mut transform in query.iter_mut() {
        transform.translation.y += 5.0;
    }
    }

}

pub fn teleport_to_nearest_green_plant(
    input: Res<Input<KeyCode>>,
    mut goat_query: Query<&mut Transform, With<Goat>>,
    plant_query: Query<(&Transform, &Sprite), (With<Plant>, Without<Goat>)>
) {
    if input.just_pressed(KeyCode::E) {
        let mut goat_transform = match goat_query.get_single_mut() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let goat_position = goat_transform.translation;

        if let Some((nearest_transform, _)) = plant_query
            .iter()
            .filter(|(_, sprite)| {
                sprite.color.r() < 0.1 &&  
                sprite.color.g() > 0.4 &&  
                sprite.color.b() < 0.1     
            })
            .min_by(|(a, _), (b, _)| {
                let da = (a.translation - goat_position).length_squared();
                let db = (b.translation - goat_position).length_squared();
                da.partial_cmp(&db).unwrap()
            })
        {

            goat_transform.translation = nearest_transform.translation;
        }
    }
}