use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use glam::{Vec2, Vec3};

pub struct Mouse{
    last_position: (f32,f32),
    device_state: DeviceState,
    was_pressed: bool,
}
impl Mouse{
    pub(crate) fn new() -> Mouse {
        Self{last_position: (0.0, 0.0), device_state: DeviceState::new(), was_pressed: false}
    }
    pub fn get_delta(&mut self) -> Vec2{
        if(!self.was_pressed){
            self.last_position=(0.0,0.0);
        }
        let mouse = self.device_state.get_mouse();
        if !mouse.button_pressed[1]{
            return Vec2::new(0.0,0.0);
            self.was_pressed=false
        }
        self.was_pressed=true;
        let current_pos = mouse.coords;
        let delta_x = current_pos.0 as f32 - self.last_position.0;
        let delta_y = current_pos.1 as f32 - self.last_position.1;
        self.last_position = (current_pos.0 as f32, current_pos.1 as f32);

        Vec2::new(delta_x, delta_y)
    }
}
