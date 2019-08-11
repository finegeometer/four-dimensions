use crate::{mesh, render};
use core::f64::consts::*;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use nalgebra as na;

/// All of the information stored by the program
pub struct Model {
    keys: HashSet<String>,
    time: Option<f64>,
    //
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,

    #[allow(clippy::type_complexity)]
    render: Box<dyn Fn(&[render::Vertex], render::Mat4Wrapper) -> Result<(), JsValue>>,
    world: Box<[[[[bool; 16]; 16]; 16]; 16]>,
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

        let render = Box::new(render::make_fn(&canvas)?);

        let mut world = Box::new([[[[false; 16]; 16]; 16]; 16]);

        world[8][8][8][8] = true;
        world[8][8][8][9] = true;
        world[9][8][8][9] = true;
        world[9][8][8][10] = true;
        world[8][7][8][9] = true;

        Ok(Model {
            keys: HashSet::new(),
            time: None,
            //
            window,
            document,
            canvas,
            render,
            world,
            //
            screen_theta: 0.0,
            screen_phi: 0.0,
            //
            position: na::Vector4::new(
                8.508_503_748_531_79,
                8.505_781_659_781_6,
                8.504_372_659_81,
                8.507_813_668_1,
            ),
            horizontal_orientation: na::UnitQuaternion::new(na::Vector3::new(
                0.004_278_651_483_965_198,
                0.004_368_756_483_652_789,
                0.003_428_975_823_465_897,
            )),
            vertical_angle: 0.007_513_658_36,
        })
    }

    pub fn view(&self) -> Result<(), JsValue> {
        web_sys::console::time_with_label("view");

        (self.render)(
            &mesh::Mesh::new(&self.world).project(self.projection_matrix()),
            self.screen_matrix().into(),
        )?;

        web_sys::console::time_end_with_label("view");

        Ok(())
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
                }
            }
            Msg::MouseWheel(z) => {
                if self.pointer_lock() {
                    self.horizontal_orientation *=
                        na::UnitQuaternion::new(na::Vector3::new(z * 1e-2, 0., 0.));
                }
            }
            Msg::KeyUp(k) => {
                self.keys.remove(&k.to_lowercase());
            }
            Msg::Frame(time) => {
                let dt: f64;
                if let Some(old_time) = self.time {
                    dt = (time - old_time) * 1e-3;
                } else {
                    dt = 0.;
                }
                self.time = Some(time);

                self.rotate_screen(dt);
                self.move_player(dt);

                self.view()?;
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
        }
        if self.keys.contains("s") {
            self.position -= m * na::Vector4::w();
        }
        if self.keys.contains("d") {
            self.position += m * na::Vector4::x();
        }
        if self.keys.contains("a") {
            self.position -= m * na::Vector4::x();
        }
        if self.keys.contains(" ") {
            self.position += m * na::Vector4::y();
        }
        if self.keys.contains("shift") {
            self.position -= m * na::Vector4::y();
        }
        if self.keys.contains("q") {
            self.position += m * na::Vector4::z();
        }
        if self.keys.contains("e") {
            self.position -= m * na::Vector4::z();
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
}

pub enum Msg {
    Click,
    Frame(f64), // time in milliseconds, counted from the start of the program.
    MouseMove([i32; 2]),
    MouseWheel(f64),
    KeyDown(String),
    KeyUp(String),
}
