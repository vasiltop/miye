use glam::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraUniform {
    pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
    ]);

    pub fn new() -> Self {
        CameraUniform {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj =
            (camera.get_projection_matrix() * camera.get_view_matrix()).to_cols_array_2d();
    }
}

impl Camera {
    pub fn apply_movement(&mut self, movement: Vec3) {
        self.position += movement;
    }

    pub fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        Camera {
            position: Vec3::new(0.0, 0.0, -1.0),
            yaw: 0.0,
            pitch: 0.0,
            fov: 45.5,
            aspect_ratio: size.width as f32 / size.height as f32,
        }
    }

    pub fn reconfigure_aspect_ratio(&mut self, window: &winit::window::Window) {
        let size = window.inner_size();
        self.aspect_ratio = size.width as f32 / size.height as f32;
    }

    pub fn get_view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(
            self.position,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    }

    pub fn get_projection_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(self.fov.to_radians(), self.aspect_ratio, 0.1, 1000.0)
    }
}
