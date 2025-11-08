use crate::gameboy::{GameBoy, InputState};

impl GameBoy {
    pub(crate) fn update_input(&mut self, state: &InputState) {
        let mut new_state: u8 = 0;
        if !state.down {
            new_state |= 1 << 7
        }
        if !state.up {
            new_state |= 1 << 6
        }
        if !state.left {
            new_state |= 1 << 5
        }
        if !state.right {
            new_state |= 1 << 4
        }
        if !state.start {
            new_state |= 1 << 3
        }
        if !state.select {
            new_state |= 1 << 2
        }
        if !state.b {
            new_state |= 1 << 1
        }
        if !state.a {
            new_state |= 1 << 0
        }
        self.joypad_state = new_state;
    }
}
