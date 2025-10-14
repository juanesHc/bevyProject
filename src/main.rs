use bevy::prelude::*;
use rand::Rng;

mod estructuras;
mod funciones;

use estructuras::goat::Goat;
use estructuras::plants::Plant;
use estructuras::collition::Collition;
use funciones::goat_movement::movement;
use funciones::plant_color_change::plant_color_changes;
use funciones::detect_collition::check_collisions;


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<Collition>()
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(plant_color_changes)
        .add_system(check_collisions)
        .run();
}

fn setup(mut commands: Commands) {


    commands.spawn(Camera2dBundle::default());


    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.2),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Goat);


        let mut rng = rand::thread_rng();

        for  _ in 1..19 {
        
        let pos_x: f32 = rng.gen_range(-400.0..=400.0); 
        let pos_y: f32 = rng.gen_range(-300.0..=300.0); 

        let size_x: f32 = rng.gen_range(30.0..=80.0); 
        let size_y: f32 = rng.gen_range(30.0..=80.0); 

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 105.0, 0.0),
                custom_size: Some(Vec2::new(size_y, size_x)),
                ..Default::default()
            },
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            ..Default::default()
        }).insert(Plant);
    }
}





