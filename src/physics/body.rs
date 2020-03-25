use super::Collider;
use crate::alg::{Bivec4, Rotor4, Vec4};
use cgmath::Vector4;

#[derive(Debug, Clone)]
pub struct Material {
    pub restitution: f32,
}

#[derive(Clone)]
pub struct Body {
    pub mass: f32,
    // for tesseracts it's sufficient to keep this as a scalar, but it really
    // should be a tensor of shape Bivec4 -> Bivec4
    pub moment_inertia_scalar: f32,
    pub material: Material,
    pub stationary: bool,

    pub pos: Vector4<f32>,
    pub vel: Vector4<f32>,
    pub rotation: Rotor4,
    pub angular_vel: Bivec4,

    pub collider: Collider,
}

impl Body {
    pub fn resolve_impulse(
        &mut self,
        impulse: Vector4<f32>,
        world_contact: Vector4<f32>,
    ) {
        if !self.stationary {
            let body_contact = self.world_pos_to_body(world_contact);
            let delta_angular_vel = self.inverse_moment_of_inertia(
                &Vec4::from(body_contact)
                    .wedge_v(&self.rotation.reverse().rotate(&impulse.into())),
            );

            self.vel += impulse / self.mass;
            self.angular_vel = self.angular_vel + delta_angular_vel;
        }
    }

    pub fn apply_projection(&mut self, projection: Vector4<f32>) {
        if !self.stationary {
            self.pos += projection;
        }
    }

    pub fn step(&mut self, dt: f32) {
        if !self.stationary {
            // apply gravity
            self.vel += Vector4::unit_y() * (-9.8 * dt);

            self.pos += self.vel * dt;
            self.rotation.update(&(dt * self.angular_vel));
        }
    }

    pub fn inverse_moment_of_inertia(&self, body_bivec: &Bivec4) -> Bivec4 {
        if self.moment_inertia_scalar <= 0.0 {
            return Bivec4::zero();
        }

        1.0 / self.moment_inertia_scalar * *body_bivec
    }

    pub fn body_vec_to_world(&self, v: Vector4<f32>) -> Vector4<f32> {
        self.rotation.rotate(&v.into()).into()
    }

    pub fn world_vec_to_body(&self, v: Vector4<f32>) -> Vector4<f32> {
        self.rotation.reverse().rotate(&v.into()).into()
    }

    pub fn body_pos_to_world(&self, v: Vector4<f32>) -> Vector4<f32> {
        let rotated: Vector4<f32> = self.rotation.rotate(&v.into()).into();
        rotated + self.pos
    }

    pub fn world_pos_to_body(&self, v: Vector4<f32>) -> Vector4<f32> {
        self.rotation
            .reverse()
            .rotate(&(v - self.pos).into())
            .into()
    }
}
