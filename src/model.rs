use crate::{fps, render, world};
use core::f64::consts::*;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use nalgebra as na;

/// All of the information stored by the program
pub struct Model {
    keys: HashSet<String>,
    fps: Option<fps::FrameCounter>,
    //
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,
    pub info_box: web_sys::HtmlParagraphElement,

    #[allow(clippy::type_complexity)]
    render: Box<dyn Fn(&[render::Vertex], render::Mat4Wrapper) -> Result<(), JsValue>>,
    occluded_mesh: Option<Vec<render::Vertex>>,

    world: world::World,
    //
    screen_theta: f64,
    screen_phi: f64,
    //
    position: na::Vector4<f64>,
    horizontal_orientation: na::UnitQuaternion<f64>,
    vertical_angle: f64,
}

impl Model {
    pub fn init() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global `window` exists")?;
        let document = window
            .document()
            .ok_or("should have a document on window")?;
        let body = document.body().ok_or("document should have a body")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_attribute("width", "800")?;
        canvas.set_attribute("height", "800")?;
        body.append_child(&canvas)?;

        let info_box = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        body.append_child(&info_box)?;

        let render = Box::new(render::make_fn(&canvas)?);

        let world = world::World::new();

        Ok(Model {
            keys: HashSet::new(),
            fps: None,
            //
            window,
            document,
            canvas,
            info_box,
            render,
            occluded_mesh: None,
            world,
            //
            screen_theta: 0.3,
            screen_phi: -0.2,
            //
            position: na::Vector4::new(1.5, 1.5, 1.5, 1.5),
            horizontal_orientation: na::UnitQuaternion::new(na::Vector3::new(0., 0., 0.)),
            vertical_angle: 0.,
        })
    }

    pub fn view(&mut self) -> Result<(), JsValue> {
        // web_sys::console::time_with_label("view");

        let occluded_mesh: &[render::Vertex];

        if let Some(x) = &self.occluded_mesh {
            occluded_mesh = x;
        } else {
            self.occluded_mesh = Some(
                self.world
                    .mesh()
                    .project(self.projection_matrix())
                    .flat_map(|render_4d::Triangle { negated, vertices }| {
                        let sign = if negated { -1. } else { 1. };
                        let [a, b, c] = vertices;
                        vec![
                            crate::render::Vertex {
                                pos: a.position.into(),
                                texcoord: a.texcoord.into(),
                                sign,
                            },
                            crate::render::Vertex {
                                pos: b.position.into(),
                                texcoord: b.texcoord.into(),
                                sign,
                            },
                            crate::render::Vertex {
                                pos: c.position.into(),
                                texcoord: c.texcoord.into(),
                                sign,
                            },
                        ]
                    })
                    .collect(),
            );
            occluded_mesh = &self.occluded_mesh.as_ref().unwrap_throw(); // Is there a better way to do this?
        }

        (self.render)(occluded_mesh, self.screen_matrix().into())?;

        // web_sys::console::time_end_with_label("view");

        Ok(())
    }

    pub fn needs_rerender(&mut self) {
        self.occluded_mesh = None;
    }

    pub fn update(&mut self, msg: Msg) -> Result<(), JsValue> {
        match msg {
            Msg::Click => {
                if !self.pointer_lock() {
                    self.canvas.request_pointer_lock();
                }
            }
            Msg::KeyDown(k) => {
                self.keys.insert(k.to_lowercase());
            }
            Msg::MouseMove([x, y]) => {
                if self.pointer_lock() {
                    self.horizontal_orientation *=
                        na::UnitQuaternion::new(na::Vector3::new(0., f64::from(x) * 3e-3, 0.));
                    self.vertical_angle -= f64::from(y) * 3e-3;
                    self.vertical_angle = self.vertical_angle.min(FRAC_PI_2);
                    self.vertical_angle = self.vertical_angle.max(-FRAC_PI_2);
                    self.needs_rerender();
                }
            }
            Msg::MouseWheel(z) => {
                if self.pointer_lock() {
                    self.horizontal_orientation *=
                        na::UnitQuaternion::new(na::Vector3::new(z * 1e-2, 0., 0.));
                }
                self.needs_rerender();
            }
            Msg::KeyUp(k) => {
                self.keys.remove(&k.to_lowercase());
            }
            Msg::Frame(time) => {
                let dt: f64;
                if let Some(fps) = &mut self.fps {
                    dt = fps.frame(time);

                    self.info_box.set_inner_text(&format!("{}", fps));

                    self.rotate_screen(dt);
                    self.move_player(dt);

                    self.eat_block();

                    self.view()?;
                } else {
                    self.fps = Some(<fps::FrameCounter>::new(time));
                }
            }
        }
        Ok(())
    }

    fn pointer_lock(&self) -> bool {
        self.document.pointer_lock_element().is_some()
    }

    fn rotate_screen(&mut self, dt: f64) {
        if self.keys.contains("arrowleft") {
            self.screen_theta -= dt;
        }
        if self.keys.contains("arrowright") {
            self.screen_theta += dt;
        }
        if self.keys.contains("arrowup") {
            self.screen_phi += dt;
            self.screen_phi = self.screen_phi.min(FRAC_PI_2)
        }
        if self.keys.contains("arrowdown") {
            self.screen_phi -= dt;
            self.screen_phi = self.screen_phi.max(-FRAC_PI_2)
        }
    }

    fn move_player(&mut self, dt: f64) {
        let m = self.horizontal_rotation().matrix() * dt;
        if self.keys.contains("w") {
            self.position += m * na::Vector4::w();
            self.needs_rerender();
        }
        if self.keys.contains("s") {
            self.position -= m * na::Vector4::w();
            self.needs_rerender();
        }
        if self.keys.contains("d") {
            self.position += m * na::Vector4::x();
            self.needs_rerender();
        }
        if self.keys.contains("a") {
            self.position -= m * na::Vector4::x();
            self.needs_rerender();
        }
        if self.keys.contains(" ") {
            self.position += m * na::Vector4::y();
            self.needs_rerender();
        }
        if self.keys.contains("shift") {
            self.position -= m * na::Vector4::y();
            self.needs_rerender();
        }
        if self.keys.contains("q") {
            self.position += m * na::Vector4::z();
            self.needs_rerender();
        }
        if self.keys.contains("e") {
            self.position -= m * na::Vector4::z();
            self.needs_rerender();
        }
    }

    fn screen_matrix(&self) -> na::Matrix4<f64> {
        na::Matrix4::new(
            1.,
            0.,
            0.,
            0.,
            0.,
            self.screen_phi.cos(),
            -self.screen_phi.sin(),
            0.,
            0.,
            self.screen_phi.sin(),
            self.screen_phi.cos(),
            0.,
            0.,
            0.,
            0.,
            1.,
        ) * na::Matrix4::new(
            self.screen_theta.cos(),
            0.,
            -self.screen_theta.sin(),
            0.,
            0.,
            1.,
            0.,
            0.,
            self.screen_theta.sin(),
            0.,
            self.screen_theta.cos(),
            0.,
            0.,
            0.,
            0.,
            1.,
        )
    }

    fn horizontal_rotation(&self) -> na::Rotation<f64, na::U4> {
        let mut out = self
            .horizontal_orientation
            .to_rotation_matrix()
            .matrix()
            .insert_row(1, 0.)
            .insert_column(1, 0.);
        out[(1, 1)] = 1.;
        na::Rotation::from_matrix_unchecked(out)
    }

    fn vertical_rotation(&self) -> na::Rotation<f64, na::U4> {
        let mut out = na::Matrix4::identity();
        out[(1, 1)] = self.vertical_angle.cos();
        out[(1, 3)] = self.vertical_angle.sin();
        out[(3, 1)] = -self.vertical_angle.sin();
        out[(3, 3)] = self.vertical_angle.cos();
        na::Rotation::from_matrix_unchecked(out)
    }

    fn projection_matrix(&self) -> na::Matrix5<f64> {
        let cotangent_half_fov = 0.5;
        let projection: na::Matrix5<f64> = na::Matrix5::new(
            cotangent_half_fov,
            0.,
            0.,
            0.,
            0.,
            0.,
            cotangent_half_fov,
            0.,
            0.,
            0.,
            0.,
            0.,
            cotangent_half_fov,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            -1.,
            0.,
            0.,
            0.,
            1.,
            0.,
        );

        let isometry = na::Isometry::from_parts(
            na::Translation {
                vector: self.position,
            },
            self.horizontal_rotation() * self.vertical_rotation(),
        )
        .inverse();
        projection * isometry.to_homogeneous()
    }

    fn eat_block(&mut self) {
        let [x, y, z, w]: [f64; 4] = self.position.into();
        if let Some(block) = self
            .world
            .block_mut([x as isize, y as isize, z as isize, w as isize])
        {
            *block = world::Block::Air;
        }
    }
}

pub enum Msg {
    Click,
    Frame(f64), // time in milliseconds, counted from the start of the program.
    MouseMove([i32; 2]),
    MouseWheel(f64),
    KeyDown(String),
    KeyUp(String),
}
