mod program;
mod shader;

mod from_tex;
mod to_tex;

use std::rc::Rc;
pub use to_tex::Mat4Wrapper;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type GL = web_sys::WebGl2RenderingContext;

pub fn make_fn(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<
    impl 'static + Fn(&[render_4d::Triangle], to_tex::Mat4Wrapper) -> Result<(), JsValue>,
    JsValue,
> {
    let gl = canvas
        .get_context("webgl2")?
        .ok_or("\"webgl2\" context identifier not supported.")?
        .dyn_into::<GL>()?;

    gl.enable(GL::BLEND);
    gl.blend_func(GL::ONE, GL::ONE);

    gl.get_extension("EXT_color_buffer_float")?
        .ok_or("OpenGL extension \"EXT_color_buffer_float\" not found.")?;

    let tex = gl.create_texture().ok_or("create_texture failed.")?;
    gl.bind_texture(GL::TEXTURE_2D, Some(&tex));
    gl.pixel_storei(GL::UNPACK_ALIGNMENT, 1);

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,                  // level
        GL::RGBA32F as i32, // internal_format
        800,                // width
        800,                // height
        0,                  // border
        GL::RGBA,           // format
        GL::FLOAT,          // type
        None,
    )?;

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

    let gl = Rc::new(gl);
    let to_tex = to_tex::make_fn(Rc::clone(&gl), &tex)?;
    let from_tex = from_tex::make_fn(gl)?;

    Ok(move |data: &[render_4d::Triangle], mat| {
        to_tex(data, mat)?;
        from_tex(&tex);
        Ok(())
    })
}
