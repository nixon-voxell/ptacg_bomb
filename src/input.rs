use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Jump,
}

// Exhaustively match `PlayerAction` and define the default binding to the input
impl PlayerAction {
    pub fn mkb_input_map() -> InputMap<PlayerAction> {
        use KeyCode::*;
        let mut input_map = InputMap::new([
            (Self::Jump, UserInput::Single(InputKind::PhysicalKey(Space))),
            (Self::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
        ]);
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map
    }
}

pub fn move_player(
    // action_state is stored as a resource
    action_state: Res<ActionState<PlayerAction>>,
) {
    if action_state.pressed(&PlayerAction::Move) {
        // We're working with gamepads, so we want to defensively ensure that we're using the clamped values
        let axis_pair = action_state.clamped_axis_pair(&PlayerAction::Move).unwrap();
        println!("Move: ({}, {})", axis_pair.x(), axis_pair.y());
    }

    if action_state.pressed(&PlayerAction::Jump) {
        println!("Jumping!");
    }
}
