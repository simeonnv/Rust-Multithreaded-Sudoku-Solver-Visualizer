use winit::window::Window;
use super::state::State;

impl State {
    pub fn get_window(&self) -> &Window {
        &self.window
    }
}