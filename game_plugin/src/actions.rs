use bevy::prelude::*;
use bevy_backroll::backroll::PlayerHandle;
use bytemuck::{Pod, Zeroable};

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>();
    }
}

#[derive(Default, Clone, Copy, PartialEq, Pod, Zeroable, Debug)]
#[repr(C)]
pub struct Actions {
    // pub player_movement: Option<Vec2>, // TODO: use Vec2 instead of two floats...
    pub player_movement_x: f32,
    pub player_movement_y: f32,
}

impl Actions {
    pub fn is_some(&self) -> bool {
        if self.player_movement_x == 0.0 && self.player_movement_y == 0.0 {
            return false;
        }
        return true;
    }

    pub fn is_none(&self) -> bool {
        return !self.is_some();
    }
}

pub fn set_movement_actions(
    handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> Actions {
    // println!("set_movement_actions");

    dbg!(handle.0);

    let mut actions = Actions::default();
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            // player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
            player_movement.y = actions.player_movement_y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            // player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
            player_movement.x = actions.player_movement_x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            // actions.player_movement = Some(player_movement);
            actions.player_movement_x = player_movement.x;
            actions.player_movement_y = player_movement.y;
        }
    } else {
        // actions.player_movement = None;
        actions.player_movement_x = 0.0;
        actions.player_movement_y = 0.0;
    }

    return actions;
}

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
