use winit::keyboard::KeyCode;
use winit::{event::KeyEvent, keyboard::PhysicalKey};

use glam::Vec3;

pub fn handle_keyboard_event(event: KeyEvent, state: &mut crate::state::State) {
    match event {
        KeyEvent {
            physical_key: PhysicalKey::Code(code),
            ..
        } => move_camera(code, state),
        _ => {}
    };
}

fn move_camera(code: KeyCode, state: &mut crate::state::State) {
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    match code {
        KeyCode::KeyW => movement.z += 0.1,
        KeyCode::KeyA => movement.x -= 0.1,
        KeyCode::KeyS => movement.z -= 0.1,
        KeyCode::KeyD => movement.x += 0.1,
        KeyCode::KeyE => movement.y += 0.1,
        KeyCode::KeyQ => movement.y -= 0.1,

        _ => {}
    };

    state.camera.apply_movement(movement);
}
