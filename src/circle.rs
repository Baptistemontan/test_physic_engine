use std::{f64::consts::TAU, mem, time::Duration};

use physic_engine::{
    matrix::static_vector::StaticColumnVector,
    soft_body::SphereObject,
    verlet_object::{VerletObject, VerletObjectBase},
};
use web_sys::CanvasRenderingContext2d;
// use gloo::console::log;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Circle {
    radius: f64,
    verlet_base: VerletObjectBase<2>,
}

impl VerletObject<2> for Circle {
    fn get_verlet_infos_mut(&mut self) -> &mut VerletObjectBase<2> {
        &mut self.verlet_base
    }

    fn get_verlet_infos(&self) -> &VerletObjectBase<2> {
        &self.verlet_base
    }
}

impl SphereObject<2> for Circle {
    fn radius(&self) -> f64 {
        self.radius
    }
}

impl Circle {
    pub fn new(
        radius: f64,
        position: StaticColumnVector<2>,
        gravity: StaticColumnVector<2>,
    ) -> Self {
        Self {
            radius,
            verlet_base: VerletObjectBase::new_accelerated(position, gravity),
        }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let [x, y] = self.position().map(|x| x);
        ctx.move_to(x + self.radius, y);
        ctx.arc(x, y, self.radius, 0.0, TAU).unwrap();
    }

    fn update_positions(circles: &mut Vec<Self>, dt: Duration) {
        circles.iter_mut().for_each(|circle| {
            circle.update(dt);
        })
    }

    fn collisions(circles: &mut Vec<Self>) {
        for i in 0..circles.len() {
            let mut circle = mem::take(&mut circles[i]);
            for other in circles[i + 1..].iter_mut() {
                if let Some(mut delta) = circle.collide_sphere(other) {
                    delta /= 2.0;

                    *circle.position_mut() += &delta;
                    *other.position_mut() -= delta;
                }
            }
            circles[i] = circle;
        }
    }

    fn apply_contraints(circles: &mut Vec<Self>, middle: &StaticColumnVector<2>, radius: f64) {
        circles.iter_mut().for_each(|circle| {
            let vec = circle.position() - middle;
            let dist = vec.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            if dist > radius - circle.radius {
                let normalized = vec / dist;
                // let s = format!("normalized: {:?}", normalized);
                // log!(s);
                *circle.position_mut() = normalized * (radius - circle.radius) + middle;
            }
        })
    }

    pub fn update_all(
        circles: &mut Vec<Self>,
        middle: &StaticColumnVector<2>,
        radius: f64,
        mut dt: Duration,
    ) {
        let sub_steps = 8;
        dt /= sub_steps;
        for _ in 0..sub_steps {
            Self::update_positions(circles, dt);
            Self::apply_contraints(circles, middle, radius);
            Self::collisions(circles);
        }
    }
}
