#![deny(unsafe_code)]
#![allow(dead_code)]

mod mesh;
mod model;
mod render;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use model::{Model, Msg};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let state: Rc<RefCell<Model>> = Rc::new(RefCell::new(Model::init()?));
    let model = state.borrow_mut();

    // Handle clicks
    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
            Closure::wrap(Box::new(move |_evt| {
                state
                    .borrow_mut()
                    .update(Msg::Click)
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Handle mouse movements
    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
            Closure::wrap(Box::new(move |evt| {
                state
                    .borrow_mut()
                    .update(Msg::MouseMove([evt.movement_x(), evt.movement_y()]))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Handle scroll wheel
    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::WheelEvent)> =
            Closure::wrap(Box::new(move |evt| {
                state
                    .borrow_mut()
                    .update(Msg::MouseWheel(evt.delta_y()))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .canvas
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Handle keys
    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::KeyboardEvent)> =
            Closure::wrap(Box::new(move |evt| {
                state
                    .borrow_mut()
                    .update(Msg::KeyDown(evt.key()))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Handle keys
    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::KeyboardEvent)> =
            Closure::wrap(Box::new(move |evt| {
                state
                    .borrow_mut()
                    .update(Msg::KeyUp(evt.key()))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .document
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Handle frames (frames in the sense of FPS)
    {
        let state = state.clone();

        #[allow(clippy::type_complexity)]
        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        let closure = move |time: f64| {
            let mut model = state.borrow_mut();
            model.update(Msg::Frame(time))?;
            model.window.request_animation_frame(
                f.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref(),
            )?;
            Ok::<(), JsValue>(())
        };

        let closure = move |time: f64| {
            closure(time).unwrap_or_else(|err| wasm_bindgen::throw_val(err));
        };

        *g.borrow_mut() = Some(Closure::wrap(Box::new(closure)));
        model
            .window
            .request_animation_frame(g.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref())?;
    }

    model.view()?;

    Ok(())
}
