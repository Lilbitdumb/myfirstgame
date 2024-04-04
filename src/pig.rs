use bevy::{ecs::reflect, prelude::*};
use crate::{Money, Player};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig{
    pub lifetime: Timer,
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

pub struct PigPlugin;
impl Plugin for PigPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update,(spwan_pig, pig_lifetime))
        .register_type::<Pig>();
    }
}