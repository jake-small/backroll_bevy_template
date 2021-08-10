use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::backroll::{BevyBackrollPlugin, StartupNetworkConfig, BackrollConfig};
use crate::GameState;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use backroll_transport_udp::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::ops::Deref;
use uuid::Uuid;

pub struct PlayerPlugin;

pub struct Player;

#[derive(Clone, Hash, Debug)]
pub struct PlayerState {
    pub id: Uuid,
    pub info: Vec<u8>, // PlayerInfo serialized
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PlayerHandle")]
struct PlayerHandleDef(pub usize);

#[derive(Serialize, Deserialize, Debug)]
struct PlayerInfo {
    #[serde(with = "PlayerHandleDef")]
    handle: PlayerHandle,
    position: Vec3,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_players.system())
                .with_system(spawn_camera.system()),
        )
        .add_plugin(BevyBackrollPlugin);
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_players(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<StartupNetworkConfig>,
    pool: Res<IoTaskPool>,
) {
    let socket = UdpManager::bind(pool.deref().deref().clone(), config.bind).unwrap();
    let peer = socket.connect(UdpConnectionConfig::unbounded(config.remote));

    commands.insert_resource(socket);

    let mut builder = backroll::P2PSession::<BackrollConfig>::build();

    let spawn1 = if config.player_number == 0 {
        Vec3::new(-200., 0., 1.)
    } else {
        Vec3::new(200., 0., 1.)
    };
    let spawn2 = if config.player_number == 1 {
        Vec3::new(-200., 0., 1.)
    } else {
        Vec3::new(200., 0., 1.)
    };

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.texture_bevy.clone().into()),
            transform: Transform::from_translation(spawn1),
            ..Default::default()
        })
        // make sure to clone the player handles for reference stuff
        .insert(if config.player_number == 0 {
            // set up local player
            let player_info = PlayerInfo {
                handle: builder.add_player(backroll::Player::Local),
                position: spawn1,
            };
            PlayerState {
                id: Uuid::new_v4(),
                info: bincode::serialize::<PlayerInfo>(&player_info).unwrap(),
            }
        } else {
            // set up remote player
            let player_info = PlayerInfo {
                // make sure to clone the remote peer for reference stuff
                handle: builder.add_player(backroll::Player::Remote(peer.clone())),
                position: spawn2,
            };
            PlayerState {
                id: Uuid::new_v4(),
                info: bincode::serialize::<PlayerInfo>(&player_info).unwrap(),
            }
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.texture_bevy.clone().into()),
            transform: Transform::from_translation(spawn2),
            ..Default::default()
        })
        .insert({
            // set up remote player
            let player_info = PlayerInfo {
                // make sure to clone the remote peer for reference stuff
                handle: builder.add_player(backroll::Player::Remote(peer.clone())),
                position: spawn2,
            };
            PlayerState {
                id: Uuid::new_v4(),
                info: bincode::serialize::<PlayerInfo>(&player_info).unwrap(),
            }
        });

    commands.start_backroll_session(builder.start(pool.deref().deref().clone()).unwrap());
}

pub fn player_movement(
    action_res: Res<GameInput<Actions>>,
    mut player_query: Query<(&mut Transform, &PlayerState)>,
) {
    // println!("Player Movement");
    let speed = 10.;
    for (mut player_transform, player) in player_query.iter_mut() {
        let player_id = player.id;

        let player_info = bincode::deserialize::<PlayerInfo>(&player.info).unwrap();

        let action = action_res.get(player_info.handle).unwrap();
        if action.is_none() {
            return;
        }
        println!("ACTION!");
        let movement = Vec3::new(
            action.player_movement_x * speed,
            action.player_movement_y * speed,
            0.,
        );
        println!("movement: {}", movement);
        player_transform.translation += movement;
    }
}
