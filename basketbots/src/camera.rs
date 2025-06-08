use raylib::prelude::{CameraMode, KeyboardKey, RaylibHandle, camera::Camera3D, math::Vector3};

pub fn init() -> Camera3D {
    Camera3D::perspective(
        Vector3::new(20.0, 8.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 2.5, 0.0),
        60.0,
    )
}

pub fn update(rlib: &RaylibHandle, camera: &mut Camera3D) {
    if rlib.is_key_down(KeyboardKey::KEY_SPACE) && camera.position.y < 20.0 {
        camera.position.y += 0.2;
        camera.target.y += 0.2;
    }

    if rlib.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) && camera.position.y > 0.8 {
        camera.position.y -= 0.2;
        camera.target.y -= 0.2;
    }

    rlib.update_camera(camera, CameraMode::CAMERA_FIRST_PERSON);
}
