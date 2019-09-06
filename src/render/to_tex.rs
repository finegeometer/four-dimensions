use super::program::Program;
use crate::utils::as_f32_array;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

type GL = web_sys::WebGl2RenderingContext;

const VERTEX_SHADER: &str = r#"#version 300 es

in vec4 pos;
in vec3 texcoord;
in float sign;

out vec3 vtexcoord;
out float vsign;

uniform mat4 proj_3d_screen;

void main() {
    vsign = sign;
    vtexcoord = texcoord;
    gl_Position = proj_3d_screen * pos;
}

"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec3 vtexcoord;
in float vsign;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = vec4((1.0 - texture(tex, vtexcoord.xy / vtexcoord.z).rgb) * 0.5 * vsign, 1.0);
}

"#;

fn iter_triangles(triangles: &[render_4d::Triangle]) -> impl Iterator<Item = f64> + '_ {
    triangles
        .iter()
        .flat_map(|render_4d::Triangle { vertices, negated }| {
            let sign = if *negated { &-1.0 } else { &1.0 };
            vertices
                .iter()
                .flat_map(move |render_4d::Vertex { position, texcoord }| {
                    position.iter().chain(texcoord).chain(std::iter::once(sign))
                })
        })
        .copied()
}

pub fn make_fn(
    gl: Rc<GL>,
    render_texture: &web_sys::WebGlTexture,
) -> Result<impl 'static + Fn(&[render_4d::Triangle], Mat4Wrapper) -> Result<(), JsValue>, JsValue>
{
    let program = Program::new(Rc::clone(&gl), VERTEX_SHADER, FRAGMENT_SHADER)?;

    let pos_loc = program.attribute("pos")?;
    let texcoord_loc = program.attribute("texcoord")?;
    let sign_loc = program.attribute("sign")?;
    let tex_loc = program.uniform("tex")?;
    let proj_3d_screen_loc = program.uniform("proj_3d_screen")?;

    let vao = gl
        .create_vertex_array()
        .ok_or("create_vertex_array failed")?;
    gl.bind_vertex_array(Some(&vao));

    let vertex_buffer = gl.create_buffer().ok_or("create_buffer failed")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.enable_vertex_attrib_array(pos_loc);
    gl.vertex_attrib_pointer_with_i32(pos_loc, 4, GL::FLOAT, false, 8 * 4, 0);
    gl.enable_vertex_attrib_array(texcoord_loc);
    gl.vertex_attrib_pointer_with_i32(texcoord_loc, 3, GL::FLOAT, false, 8 * 4, 4 * 4);
    gl.enable_vertex_attrib_array(sign_loc);
    gl.vertex_attrib_pointer_with_i32(sign_loc, 1, GL::FLOAT, false, 8 * 4, 7 * 4);

    let framebuffer = gl.create_framebuffer().ok_or("create_framebuffer failed")?;
    gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
    gl.framebuffer_texture_2d(
        GL::FRAMEBUFFER,
        GL::COLOR_ATTACHMENT0,
        GL::TEXTURE_2D,
        Some(render_texture),
        0,
    );

    let texture = gl.create_texture().ok_or("create_texture failed")?;
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,                 // level
        GL::RGBA as i32,   // internal_format
        256,               // width
        256,               // height
        0,                 // border
        GL::RGBA,          // format
        GL::UNSIGNED_BYTE, // type
        Some(include_bytes!("../../resources/tex")),
    )?;
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);

    Ok(move |data: &[render_4d::Triangle], mat: Mat4Wrapper| {
        let data: Vec<f32> = iter_triangles(data).map(|x| x as f32).collect();

        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
        gl.bind_vertex_array(Some(&vao));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &as_f32_array(&data)?.into(),
            GL::DYNAMIC_DRAW,
        );

        gl.viewport(0, 0, 800, 800);
        gl.clear_color(0., 0., 0., 1.);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&program));
        gl.bind_vertex_array(Some(&vao));

        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        gl.uniform1i(Some(&tex_loc), 0);

        let mat: nalgebra::Matrix4<f64> = mat.0;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&proj_3d_screen_loc),
            false,
            &mat.into_iter().map(|&x| x as f32).collect::<Vec<_>>(),
        );

        gl.draw_arrays(GL::TRIANGLES, 0, (data.len() / 8) as i32);

        Ok(())
    })
}

// For some reason, it won't compile without the wrapper.
pub struct Mat4Wrapper(pub nalgebra::Matrix4<f64>);
impl From<nalgebra::Matrix4<f64>> for Mat4Wrapper {
    fn from(m: nalgebra::Matrix4<f64>) -> Self {
        Self(m)
    }
}
