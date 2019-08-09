use wasm_bindgen::prelude::*;

use super::program::Program;
use crate::utils::as_f32_array;
use std::rc::Rc;

type GL = web_sys::WebGl2RenderingContext;

const VERTEX_SHADER: &str = r#"#version 300 es

in vec2 coord;
out vec2 vcoord;

void main() {
	vcoord = coord;
	gl_Position = vec4(coord * 2.0 - 1.0, 0.0, 1.0);
}

"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec2 vcoord;
out vec4 color;
uniform sampler2D tex;

void main() {
	color = exp(-texture(tex, vcoord));
}

"#;

pub fn make_fn(gl: Rc<GL>) -> Result<impl 'static + Fn(&web_sys::WebGlTexture), JsValue> {
    let program = Program::new(Rc::clone(&gl), VERTEX_SHADER, FRAGMENT_SHADER)?;

    let coord_loc = program.attribute("coord")?;
    let tex_loc = program.uniform("tex")?;

    let vao = gl
        .create_vertex_array()
        .ok_or("create_vertex_array failed")?;
    gl.bind_vertex_array(Some(&vao));

    let vertex_buffer = gl.create_buffer().ok_or("create_buffer failed")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.enable_vertex_attrib_array(coord_loc);
    gl.vertex_attrib_pointer_with_i32(coord_loc, 2, GL::FLOAT, false, 0, 0);
    gl.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &as_f32_array(&[0., 0., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0.])?.into(),
        GL::STATIC_DRAW,
    );

    Ok(move |tex: &web_sys::WebGlTexture| {
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.bind_vertex_array(Some(&vao));

        gl.viewport(0, 0, 800, 800);
        gl.clear_color(0., 0., 0., 1.);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&program));
        gl.bind_vertex_array(Some(&vao));

        gl.bind_texture(GL::TEXTURE_2D, Some(tex));
        gl.uniform1i(Some(&tex_loc), 0);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    })
}
