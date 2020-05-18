use std::f32;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;
extern crate genmesh;

mod app_state;
mod common_funcs;
mod constants;
mod gl_setup;
mod programs;
mod shaders;
pub mod debug;

// Tidy variable management
#[wasm_bindgen]
pub struct DougsClient {
    gl: WebGlRenderingContext,
    program_mesh: programs::Mesh,
}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            program_mesh: programs::Mesh::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        // Generate some test data
        let grid = common_funcs::get_position_grid_n_by_n(constants::GRID_SIZE);
        // Clone, iter skip, step over, collect
        let mut x_vals : Vec<f32> = grid.0.clone().into_iter().step_by(3).collect();
        // Clone, iter skip, step over, collect
        let mut z_vals : Vec<f32> = grid.0.clone().into_iter().skip(2).step_by(3).collect();
        // Clone, iter skip, step over, collect
        let mut y_vals : Vec<f32> = x_vals.iter().zip(z_vals.iter()).map(|xz| (xz.0*xz.0 + xz.1*xz.1).sqrt() - 1.).collect();
        // Make a mesh
        let mut vertices: Vec<f32> = vec![0.; grid.0.len()];
        for i in 0..vertices.len() {
            vertices[i] = match i%3 {
                0 => x_vals[i / 3],
                1 => y_vals[i / 3],
                _ => z_vals[i / 3],
            };
        }
        // Indices
        let indices = grid.1;

        // Normals
        let normals = common_funcs::get_grid_normals(constants::GRID_SIZE, &y_vals);

        // Colors
        let colors = vertices.clone().into_iter().map(|v| v.max(0.).min(1.)).collect();

        self.program_mesh.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            curr_state.scale,
            &vertices,
            &normals,
            &indices,
            &colors,
        );
    }
}
