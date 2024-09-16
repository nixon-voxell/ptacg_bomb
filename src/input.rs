use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Jump,
}

// Input mappings for PlayerAction
impl PlayerAction {
    pub fn create_input_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::new(Self::default_bindings());
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map
    }

    // Default key bindings for actions
    fn default_bindings() -> [(PlayerAction, UserInput); 2] {
        use KeyCode::*;
        [
            (Self::Jump, UserInput::Single(InputKind::PhysicalKey(Space))),
            (Self::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
        ]
    }
}

// Handle player input actions
pub fn handle_player_input(action_state: Res<ActionState<PlayerAction>>) {
    if action_state.pressed(&PlayerAction::Move) {
        process_movement(&action_state); // Process movement input
    }

    if action_state.pressed(&PlayerAction::Jump) {
        execute_jump(); // Execute jump action
    }
}

// Process movement input
fn process_movement(action_state: &Res<ActionState<PlayerAction>>) {
    if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
        println!("Move: ({}, {})", axis_pair.x(), axis_pair.y());
    }
}

// Execute jump action
fn execute_jump() {
    println!("Jumping!");
}
