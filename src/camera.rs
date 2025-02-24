use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;

        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_space_pressed: bool,
    is_shift_pressed: bool,
    is_rotate_right_pressed: bool,
    is_rotate_left_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_space_pressed: false,
            is_shift_pressed: false,
            is_rotate_right_pressed: false,
            is_rotate_left_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    KeyCode::Space => {
                        self.is_space_pressed = is_pressed;
                        true
                    }
                    KeyCode::ShiftLeft => {
                        self.is_shift_pressed = is_pressed;
                        true
                    }
                    KeyCode::ArrowRight => {
                        self.is_rotate_right_pressed = is_pressed;
                        true
                    }
                    KeyCode::ArrowLeft => {
                        self.is_rotate_left_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, time:&crate::time::Time) {
        const LINEAR_TO_ANGULAR_FACTOR:f32 = 0.2;
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        if self.is_forward_pressed {
            camera.eye += forward_norm * self.speed * time.delta_time();
            camera.target += forward_norm * self.speed * time.delta_time();
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed * time.delta_time();
            camera.target -= forward_norm * self.speed * time.delta_time();
        }
        let right = forward_norm.cross(camera.up);

        if self.is_right_pressed {
            camera.eye += right * self.speed * time.delta_time();
            camera.target += right * self.speed * time.delta_time();
        }

        if self.is_left_pressed {
            camera.eye -= right * self.speed * time.delta_time();
            camera.target -= right * self.speed * time.delta_time();
        }

        if self.is_space_pressed {
            camera.eye += camera.up * self.speed * time.delta_time();
            camera.target += camera.up * self.speed * time.delta_time();
        }

        if self.is_shift_pressed {
            camera.eye -= camera.up * self.speed * time.delta_time();
            camera.target -= camera.up * self.speed * time.delta_time();
        }

        let angle = self.speed * LINEAR_TO_ANGULAR_FACTOR * time.delta_time();
        if self.is_rotate_right_pressed {      
            let rotated_norm = cgmath::Matrix4::from_angle_y(cgmath::Rad{ 0: -angle }) * forward_norm.extend(1.0);
            let target = rotated_norm.truncate().normalize() * 2.0;
            let (x, y, z) = (target.x + camera.eye.x, target.y + camera.eye.y, target.z + camera.eye.z);
            camera.target = cgmath::Point3::new(x, y, z);
        }
        if self.is_rotate_left_pressed {            
            let rotated_norm = cgmath::Matrix4::from_angle_y(cgmath::Rad{ 0: angle }) * forward_norm.extend(1.0);
            let target = rotated_norm.truncate().normalize() * 2.0;
            let (x, y, z) = (target.x + camera.eye.x, target.y + camera.eye.y, target.z + camera.eye.z);
            camera.target = cgmath::Point3::new(x, y, z);
        }

        // // Prevents glitching when the camera gets too close to the
        // // center of the scene.
        // if self.is_forward_pressed && forward_mag > self.speed {
        //     camera.eye += forward_norm * self.speed;
        // }
        // if self.is_backward_pressed {
        //     camera.eye -= forward_norm * self.speed;
        // }

        // let right = forward_norm.cross(camera.up);

        // // Redo radius calc in case the forward/backward is pressed.
        // let forward = camera.target - camera.eye;
        // let forward_mag = forward.magnitude();

        // if self.is_right_pressed {
        //     // Rescale the distance between the target and the eye so
        //     // that it doesn't change. The eye, therefore, still
        //     // lies on the circle made by the target and eye.
        //     camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        // }
        // if self.is_left_pressed {
        //     camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        // }

        // // Redo radius calc in case the left/right is pressed.
        // let forward = camera.target - camera.eye;
        // let forward_mag = forward.magnitude();

        // if self.is_space_pressed {
        //     camera.eye = camera.target - (forward - camera.up * self.speed).normalize() * forward_mag;
        // }
        // if self.is_shift_pressed {
        //     camera.eye = camera.target - (forward + camera.up * self.speed).normalize() * forward_mag;
        // }
    }
}
