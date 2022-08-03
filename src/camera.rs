use atomecs::bevy_bridge::Scale;
use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct DemoCamera {
    pub orbit: f32,
    pub delta: f32,
    pub radius: f32,
    pub target: Vec3,
}
impl DemoCamera {
    pub fn get_quaternion(&self) -> Quat {
        Quat::from_rotation_y(self.orbit) * Quat::from_rotation_x(self.delta)
    }
    pub fn get_transform(&self, scale: f32) -> Transform {
        let x = self.radius * self.orbit.cos() * self.delta.cos() + self.target.x * scale;
        let y = self.radius * self.delta.sin() + self.target.y * scale;
        let z = self.radius * self.orbit.sin() * self.delta.cos() + self.target.z * scale;
        Transform::from_xyz(x,y,z).looking_at(self.target * scale, Vec3::Y)
    }

    pub fn new(radius: f32, target: Vec3) -> Self {
        Self { radius, target, ..default() }
    }
}
impl Default for DemoCamera {
    fn default() -> Self {
        Self { orbit: 0.6, delta: 0.6, radius: 5.0, target: Vec3::ZERO }
    }
}

pub fn control_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    scale: Res<Scale>,
    mut query: Query<(&mut DemoCamera, &mut Transform)>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta * 1e-2;
    }
    for (mut demo_camera, mut transform) in query.iter_mut() {
        if mouse_button_input.pressed(MouseButton::Right) {
            demo_camera.orbit = demo_camera.orbit + delta.x;
            demo_camera.delta = demo_camera.delta + delta.y;
            demo_camera.delta = demo_camera.delta.min(1.4);
            demo_camera.delta = demo_camera.delta.max(-1.4);
            let t = demo_camera.get_transform(scale.0 as f32);
            transform.translation = t.translation;
            transform.rotation = t.rotation;
        }
    }
}