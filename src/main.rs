use bevy::prelude::*;
use bevy_aseprite::{aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin};

mod random;
use random::Random;

aseprite!(pub AsepritePlayer, "assets/sprites/Sprite-0002.aseprite");
aseprite!(pub AsepriteTerrain, "assets/sprites/terrain.aseprite");

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
pub enum InputAction {
    Move(Direction),
    StopMovement,
}

pub fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);

    let mut random = Random::new(0);

    commands.spawn_batch((-20..20).flat_map(|y| (-20..20).map(move |x| (x, y))).map(
        move |(x, y)| AsepriteBundle {
            aseprite: AsepriteTerrain::sprite(),
            animation: if random.chance(0.05) {
                AsepriteTerrain::tags::GRASS2.into()
            } else {
                AsepriteTerrain::tags::GRASS.into()
            },
            transform: Transform::from_xyz((x as f32) * 16., (y as f32) * 16., 0.),
            ..Default::default()
        },
    ));

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: AsepritePlayer::sprite(),
            animation: AsepritePlayer::tags::SOUTH_IDLE.into(),
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Direction::South);
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
    if !keys.pressed(KeyCode::W)
        && !keys.pressed(KeyCode::S)
        && !keys.pressed(KeyCode::A)
        && !keys.pressed(KeyCode::D)
    {
        input_actions.send(InputAction::StopMovement);
    }
}

pub fn player_walk(
    mut query: Query<(&mut AsepriteAnimation, &mut Direction), With<Player>>,
    mut input_actions: EventReader<InputAction>,
) {
    if let Ok((mut player_animation, mut direction)) = query.get_single_mut() {
        for event in input_actions.iter() {
            match *event {
                InputAction::Move(Direction::North) => {
                    *player_animation = AsepritePlayer::tags::NORTH_WALK.into();
                    *direction = Direction::North;
                }
                InputAction::Move(Direction::South) => {
                    *player_animation = AsepritePlayer::tags::SOUTH_WALK.into();
                    *direction = Direction::South;
                }
                InputAction::Move(Direction::West) => {
                    *player_animation = AsepritePlayer::tags::WEST_WALK.into();
                    *direction = Direction::West;
                }
                InputAction::Move(Direction::East) => {
                    *player_animation = AsepritePlayer::tags::EAST_WALK.into();
                    *direction = Direction::East;
                }
                InputAction::StopMovement => {
                    *player_animation = match *direction {
                        Direction::North => AsepritePlayer::tags::NORTH_IDLE.into(),
                        Direction::South => AsepritePlayer::tags::SOUTH_IDLE.into(),
                        Direction::West => AsepritePlayer::tags::WEST_IDLE.into(),
                        Direction::East => AsepritePlayer::tags::EAST_IDLE.into(),
                    }
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
