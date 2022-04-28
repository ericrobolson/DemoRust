use ext_glam::{Vec2, Vec3};

use crate::{image::Color, world::World};

pub struct Renderer {
    max_marching_steps: u32,
    min_dist: f32,
    max_dist: f32,
    epsilon: f32,
    eye: Vec3,
}
impl Renderer {
    pub fn new() -> Self {
        let eye: Vec3 = (0.0, 0.0, 5.0).into();

        Self {
            max_marching_steps: 255,
            min_dist: 0.0,
            max_dist: 100.0,
            epsilon: 0.0001,
            eye,
        }
    }

    /// Returns the direction of a ray
    pub fn ray_direction(field_of_view: f32, size: Vec2, frag_coord: Vec2) -> Vec3 {
        let xy = frag_coord - size / 2.0;
        let z = size.y / (field_of_view.to_radians() / 2.0).tan();

        Vec3::new(xy.x, xy.y, -z).normalize()
    }

    /// Estimates a normal
    pub fn estimate_normal(&self, point: Vec3, world: &World) -> Vec3 {
        let x = world.scene_sdf((point.x + self.epsilon, point.y, point.z).into())
            - world.scene_sdf((point.x - self.epsilon, point.y, point.z).into());
        let y = world.scene_sdf((point.x, point.y + self.epsilon, point.z).into())
            - world.scene_sdf((point.x, point.y - self.epsilon, point.z).into());
        let z = world.scene_sdf((point.x, point.y, point.z + self.epsilon).into())
            - world.scene_sdf((point.x, point.y, point.z - self.epsilon).into());

        Vec3::new(x, y, z).normalize()
    }

    /// Returns the shortest distance to a surface
    pub fn shortest_dist_to_surface(
        &self,
        world: &World,
        eye: Vec3,
        marching_direction: Vec3,
        start: f32,
        end: f32,
    ) -> f32 {
        let mut depth = start;
        for _i in 0..self.max_marching_steps {
            let dist = world.scene_sdf(eye + depth * marching_direction);
            if dist < self.epsilon {
                return depth;
            }

            depth += dist;
            if depth >= end {
                return end;
            }
        }

        end
    }

    pub fn phong_contrib_for_light(
        &self,
        world: &World,
        k_d: Vec3,
        k_s: Vec3,
        alpha: f32,
        p: Vec3,
        eye: Vec3,
        light_pos: Vec3,
        light_intensity: Vec3,
    ) -> Vec3 {
        /**
         * Lighting contribution of a single point light source via Phong illumination.
         *
         * The vec3 returned is the RGB color of the light's contribution.
         *
         * k_a: Ambient color
         * k_d: Diffuse color
         * k_s: Specular color
         * alpha: Shininess coefficient
         * p: position of point being lit
         * eye: the position of the camera
         * lightPos: the position of the light
         * lightIntensity: color/intensity of the light
         *
         * See https://en.wikipedia.org/wiki/Phong_reflection_model#Description
         */
        let n = self.estimate_normal(p, world);
        let l = (light_pos - p).normalize();
        let v = (eye - p).normalize();
        let r = (reflect(-l, n)).normalize();

        let dot_ln = l.dot(n);
        let dot_rv = r.dot(v);

        if (dot_ln < 0.0) {
            return light_intensity * (k_d * dot_ln);
        }

        return light_intensity * (k_d * dot_ln + k_s * f32::powf(dot_rv, alpha));
    }

    /// Returns the illumination for the given point
    pub fn phong_illumination(
        &self,
        world: &World,
        k_a: Vec3,
        k_d: Vec3,
        k_s: Vec3,
        alpha: f32,
        p: Vec3,
        eye: Vec3,
    ) -> Vec3 {
        /**
         * Lighting via Phong illumination.
         *
         * The vec3 returned is the RGB color of that point after lighting is applied.
         * k_a: Ambient color
         * k_d: Diffuse color
         * k_s: Specular color
         * alpha: Shininess coefficient
         * p: position of point being lit
         * eye: the position of the camera
         *
         * See https://en.wikipedia.org/wiki/Phong_reflection_model#Description
         */
        let ambient_light = 0.5 * Vec3::new(1., 1., 1.);
        let mut color = ambient_light * k_a;

        let light1_pos: Vec3 = (4.0, 2.0, 4.0).into();
        let light1_intensity: Vec3 = (0.4, 0.4, 0.4).into();
        color += self.phong_contrib_for_light(
            world,
            k_d,
            k_s,
            alpha,
            p,
            eye,
            light1_pos,
            light1_intensity,
        );

        let light2_pos: Vec3 = (2.0, 2.0, 2.0).into();
        let light2_intensity: Vec3 = (0.4, 0.4, 0.4).into();
        color += self.phong_contrib_for_light(
            world,
            k_d,
            k_s,
            alpha,
            p,
            eye,
            light2_pos,
            light2_intensity,
        );

        color
    }

    /// Returns the color for the given point
    pub fn color(&self, world: &World, size: Vec2, frag_coord: Vec2) -> Color {
        let marching_direction = Self::ray_direction(45.0, size, frag_coord);

        let dist = self.shortest_dist_to_surface(
            world,
            self.eye,
            marching_direction,
            self.min_dist,
            self.max_dist,
        );

        if dist > (self.max_dist - self.epsilon) {
            return (0, 0, 0, 0).into();
        }

        let p = self.eye + dist * marching_direction;

        let k_a: Vec3 = (0.2, 0.2, 0.2).into();
        let k_d: Vec3 = (0.7, 0.2, 0.2).into();
        let k_s: Vec3 = (1., 1., 1.).into();
        let shininess = 10.0;

        let color = self.phong_illumination(world, k_a, k_d, k_s, shininess, p, self.eye);

        (color.x, color.y, color.z, 1.0).into()
    }
}

fn reflect(d: Vec3, normal: Vec3) -> Vec3 {
    let n = normal.normalize();

    2.0 * (d.dot(n)) * n - n
}
