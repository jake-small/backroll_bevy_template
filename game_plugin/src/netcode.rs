use crate::actions::{set_movement_actions, Actions};
use crate::player::{player_movement, PlayerState};
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use std::net::SocketAddr;
use std::env;

const MATCH_UPDATE_LABEL: &str = "MATCH_UPDATE";
const DELTA_TIME: f32 = 1.0 / 60.0; // in ms
const LOCAL_PLAYER_NUMBER: usize = 0;

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
            .insert_resource(get_network_config());
    }
}

#[derive(Debug)]
pub struct StartupNetworkConfig {
    pub local_player_number: usize,
    pub local_ip: SocketAddr,
    pub remote_ip: SocketAddr,
}

fn get_network_config() -> StartupNetworkConfig {
    let player_number = get_player_number();
    return StartupNetworkConfig {
        local_player_number: player_number,
        local_ip: local_ip(player_number).parse().unwrap(),
        remote_ip: remote_ip(player_number).parse().unwrap(),
    }
}

fn get_player_number() -> usize {
    if let Some(player_number) = env::args().nth(1) {
        println!("Local Player Number: {}", &player_number);
        return player_number.parse().unwrap();
    }
    panic!("Missing arg: Must specify local player number in cli command");
}

fn local_ip(player_number: usize) -> String {
    if player_number == 0 {
        return format!("127.0.0.1:{}", 59480);
    } else if player_number == 1 {
        return format!("127.0.0.1:{}", 59481);
    } else {
        panic!("Error: This is only setup to work with 2 players");
    }
}

fn remote_ip(player_number: usize) -> String {
    if player_number == 0 {
        return format!("127.0.0.1:{}", 59481);
    } else if player_number == 1 {
        return format!("127.0.0.1:{}", 59480);
    } else {
        panic!("Error: This is only setup to work with 2 players");
    }
}

fn save_world(player_query: Query<&PlayerState>) -> PlayState {
    println!("Save State");
    let mut player_states = Vec::new();
    for player_state in player_query.iter() {
        println!("Save State - query player");
        player_states.push(player_state.clone());
    }
    return PlayState {
        players: player_states,
    };
}

fn load_world(state: In<PlayState>, mut player_query: Query<&mut PlayerState>) {
    println!("Load State");
    let incoming_player_states = state.0.players;
    for mut player in player_query.iter_mut() {
        println!("Load State - query player");
        let player_pos = incoming_player_states
            .iter()
            .position(|s| s.id == player.id)
            .unwrap();
        let incoming_player = incoming_player_states.get(player_pos).unwrap();
        player.clone_from(&incoming_player);
    }
}