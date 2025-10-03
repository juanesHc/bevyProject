use bevy::prelude::*;
use crate::estructuras::goat::Goat;

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