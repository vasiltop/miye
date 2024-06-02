use winit::event::ElementState;
use winit::keyboard::KeyCode;
use winit::{event::KeyEvent, keyboard::PhysicalKey};

use glam::Vec3;

pub fn handle_keyboard_event(event: KeyEvent, app_state: &mut crate::state::State) {
    if let KeyEvent {
        physical_key: PhysicalKey::Code(code),
        state,
        ..
    } = event
    {
        move_camera(code, app_state, state)
    };
}

fn move_camera(code: KeyCode, app_state: &mut crate::state::State, state: ElementState) {
    if state != ElementState::Pressed {
        return;
    };

    const SPEED: f32 = 0.2;
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    let camera = &app_state.camera;
    let forward = -camera.position;

    let forward_norm = forward.normalize();

    let right = forward_norm.cross(glam::Vec3::new(0.0, 1.0, 0.0));
    match code {
        KeyCode::KeyW => movement += forward_norm * SPEED,
        KeyCode::KeyA => movement -= right.normalize() * SPEED,
        KeyCode::KeyS => movement -= forward_norm * SPEED,
        KeyCode::KeyD => movement += right.normalize() * SPEED,
        KeyCode::KeyE => movement.y += SPEED,
        KeyCode::KeyQ => movement.y -= SPEED,

        _ => {}
    };

    app_state.camera.apply_movement(movement);
}
