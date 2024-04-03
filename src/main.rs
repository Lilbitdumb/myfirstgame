use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::render::camera::{self, ScalingMode};


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

fn spwan_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,    
){
    if !input.just_pressed(KeyCode::Space){        
        return;    
    }

    let player_transform = player.single();

    if money.0 >= 10.0{
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}",money.0)
    }

    let texture = asset_server.load("pig.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()            
        },
        Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
){
    for (pig_entity, mut pig) in &mut pigs{
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished(){
            money.0 += 15.0;

            commands.entity(pig_entity).despawn();

            info!("Pig solf for $15! Current Money: {:?}", money.0);
        }
    }
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

#[derive(Component)]
pub struct Pig{
    pub lifetime: Timer,
}

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
        .add_systems(Startup, setup)
        .add_systems(Update,(player_movement,spwan_pig,pig_lifetime))
        .run();
}

