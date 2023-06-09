use crate::Vec3 as Vector;

pub struct Camera {
    pub position: Vector,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f64,
    pub fov: f64,
    pub sensitivity: f64,
}

fn pitch_yaw_to_direction(pitch: f32, yaw: f32) -> Vector {
    let xz_unit_len = pitch.cos();
    let x = -xz_unit_len * yaw.cos();
    let y = -xz_unit_len * (yaw).sin();
    let z = pitch.sin();
    Vector::new(x, y, z)
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector::new(0., 0., 0.),
            pitch: 0.0,
            yaw: std::f32::consts::PI * 0.5,
            speed: 1.,
            fov: 80.0,
            sensitivity: 4.,
        }
    }

    fn direction(&self) -> Vector {
        pitch_yaw_to_direction(self.pitch, self.yaw)
    }

    fn look_to_rh(&self, direction: Vector, up: Vector) -> glam::Mat4 {
        let front = direction.normalize();
        let side = front.cross(up).normalize();
        let local_up = side.cross(front);
        glam::Mat4::from_cols(
            glam::vec4(side.x, local_up.x, -front.x, 0.0),
            glam::vec4(side.y, local_up.y, -front.y, 0.0),
            glam::vec4(side.z, local_up.z, -front.z, 0.0),
            glam::vec4(
                -self.position.dot(side),
                -self.position.dot(local_up),
                self.position.dot(front),
                1.0,
            ),
        )
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position += self.direction() * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        let yaw = self.yaw - std::f32::consts::PI * 0.5;
        let direction = pitch_yaw_to_direction(0., yaw);

        self.position += direction * distance;
    }

    pub fn move_up(&mut self, distance: f32) {
        self.position += Vector::Z * distance;
    }

    pub fn calc_view_matrix(&self) -> glam::Mat4 {
        self.look_to_rh(pitch_yaw_to_direction(self.pitch, self.yaw), Vector::Z)
    }
}
