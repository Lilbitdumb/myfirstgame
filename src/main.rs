use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::render::camera::ScalingMode;
use pig::PigPlugin;

mod pig;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 256.0, min_height: 144.0 };    
    
    commands.spawn(camera);  

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {     
            texture,
            ..default()
        },
        Player {speed: 100.0},
    ));
}

fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform,player) in &mut characters{
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW){
            transform.translation.y += movement_amount;            
        }
        if input.pressed(KeyCode::KeyS){
            transform.translation.y -= movement_amount;            
        }
        if input.pressed(KeyCode::KeyD){
            transform.translation.x += movement_amount;            
        }
        if input.pressed(KeyCode::KeyA){
            transform.translation.x -= movement_amount;            
        }
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32
}

#[derive(Resource)]
pub struct Money(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest()) 
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: "My first Game".into(),
                    resolution:(640.0,480.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
          .build(),
        )   
        .insert_resource(Money(100.0))
        .add_plugins(PigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update,player_movement)
        .run();
}

