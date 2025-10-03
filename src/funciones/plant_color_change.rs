use bevy::prelude::*;
use crate::estructuras::plants::Plant;  

pub fn plant_color_change(mut query: Query<&mut Sprite, With<Plant>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(
            255.0, 
            255.0, 
            0.0
        );
    }
    
}