use crate::actions::{Actions, GameControl, set_movement_actions};
use crate::player::{PlayerState, player_movement};
use crate::loading::TextureAssets;
use crate::GameState;
use backroll_transport_udp::*;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
// use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
// use ordered_float::OrderedFloat;
use std::net::SocketAddr;
use std::ops::Deref;
use uuid::Uuid;


const MATCH_UPDATE_LABEL: &str = "MATCH_UPDATE";
const DELTA_TIME: f32 = 1.0 / 60.0; // in ms
const LOCAL_IP: &str = "127.0.0.1:59486";
const REMOTE_IP: &str = "127.0.0.1:59487";
const PLAYER_NUMBER: usize = 0;

pub struct BackrollConfig;
impl Config for BackrollConfig {
    type Input = Actions;
    type State = PlayState;
}
#[derive(Clone, Hash)]
pub struct PlayState {
    pub players: Vec<PlayerState>,
}

pub struct BevyBackrollPlugin;

impl Plugin for BevyBackrollPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_plugin(BackrollPlugin::<BackrollConfig>::default())
            .with_rollback_run_criteria::<BackrollConfig, _>(
                FixedTimestep::step(DELTA_TIME.into()).with_label(MATCH_UPDATE_LABEL),
            )
            .with_input_sampler_system::<BackrollConfig, _>(set_movement_actions.system())
            .with_world_save_system::<BackrollConfig, _>(save_world.system())
            .with_world_load_system::<BackrollConfig, _>(load_world.system())
            .with_rollback_system::<BackrollConfig, _>(player_movement.system())
            .insert_resource(StartupNetworkConfig {
                player_number: PLAYER_NUMBER,
                bind: LOCAL_IP.parse().unwrap(),
                remote: REMOTE_IP.parse().unwrap(),
                local_ip: LOCAL_IP.to_string(),
                remote_ip: REMOTE_IP.to_string(),
            });
    }
}

// TODO: simplify this
#[derive(Debug)]
pub struct StartupNetworkConfig {
    pub player_number: usize,
    pub bind: SocketAddr,
    pub remote: SocketAddr,
    pub local_ip: String,
    pub remote_ip: String,
}

fn save_world(query: Query<&PlayerState>) -> PlayState {
    // println!("Save State");
    let mut player_states = Vec::new();
    for player_state in query.iter() {
        println!("Save State - query player");
        player_states.push(player_state.clone());
    }
    return PlayState {
        players: player_states,
    };
}

fn load_world(state: In<PlayState>, mut query: Query<&mut PlayerState>) {
    println!("Load State");
    let incoming_player_states = state.0.players;
    for mut player in query.iter_mut() {
        println!("Load State - query player");
        let player_pos = incoming_player_states
            .iter()
            .position(|s| s.id == player.id)
            .unwrap();
        let incoming_player = incoming_player_states.get(player_pos).unwrap();
        *player = incoming_player.clone();
    }
}