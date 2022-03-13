use bevy::prelude::*;
use bevy_aseprite::{aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin};

aseprite!(pub AsepritePlayer, "assets/sprites/Sprite-0002.aseprite");

#[derive(Component)]
pub struct Player;

pub enum Direction {
    North,
    South,
    East,
    West,
}
pub enum InputAction {
    Move(Direction),
}

pub fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: AsepritePlayer::sprite(),
            animation: AsepritePlayer::tags::SOUTH_WALK.into(),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

pub fn input(mut input_actions: EventWriter<InputAction>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::W) {
        input_actions.send(InputAction::Move(Direction::North));
    }
    if keys.just_pressed(KeyCode::S) {
        input_actions.send(InputAction::Move(Direction::South));
    }
    if keys.just_pressed(KeyCode::A) {
        input_actions.send(InputAction::Move(Direction::West));
    }
    if keys.just_pressed(KeyCode::D) {
        input_actions.send(InputAction::Move(Direction::East));
    }
}

pub fn player_walk(
    mut query: Query<&mut AsepriteAnimation, With<Player>>,
    mut input_actions: EventReader<InputAction>,
) {
    if let Ok(mut player_animation) = query.get_single_mut() {
        for event in input_actions.iter() {
            match *event {
                InputAction::Move(Direction::North) => {
                    *player_animation = AsepritePlayer::tags::NORTH_WALK.into()
                }
                InputAction::Move(Direction::South) => {
                    *player_animation = AsepritePlayer::tags::SOUTH_WALK.into()
                }
                InputAction::Move(Direction::West) => {
                    *player_animation = AsepritePlayer::tags::WEST_WALK.into()
                }
                InputAction::Move(Direction::East) => {
                    *player_animation = AsepritePlayer::tags::EAST_WALK.into()
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsepritePlugin)
        .add_event::<InputAction>()
        .add_startup_system(startup)
        .add_system(input)
        .add_system(player_walk)
        .run();
}
