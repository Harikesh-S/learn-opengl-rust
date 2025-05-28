// Object outlining using stencil buffer

use std::ffi::CStr;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 4 : Part 2 : Object outlining using stencil";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Stencil Testing";

pub fn main_4_2() {

    println!("{}\n{}", WINDOW_TITLE, MESSAGE);

    // --Initialize GLFW, Create window and load OpenGL functions------------------------------------------------------------------ //

    // Initialize GLFW
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    // Set hints for open gl version
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create window
    let (mut window, events) = glfw
        .create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window!");

    // Set current context , enable polling
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);

    // Load open gl functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // --Creating OpenGL Objects--------------------------------------------------------------------------------------------------- //

    // Shader Program
    let default_shader = Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_3_model_loading/shaders/1_default.frag");
    let outline_shader = Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/2_outline_shader.frag");
    
    // Loading models
    let mut model_ferris = Model::new();
    model_ferris.load_model("./resources/models/ferris3d_v1.0.obj");

    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Camera
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(0.,0.5,4.), 0., 0., -90., WINDOW_WIDTH, WINDOW_HEIGHT);

    // Viewport
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as GLint, WINDOW_HEIGHT as GLint);
    }

    // Wireframe mode - optional
    // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

    // Enable depth testing to put display top most primitives
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Enable stencil testing
    unsafe {
        gl::Enable(gl::STENCIL_TEST);
    }

    // Time
    let mut prev_time = glfw.get_time();

    // Model matrices and transformations
    let identity = glm::Mat4::identity();
    let model_matrix_f = identity;

    // --Render loop--------------------------------------------------------------------------------------------------------------- //

    while !window.should_close() {

        // Time
        let curr_time = glfw.get_time();
        let time_delta = curr_time - prev_time;

        // Update -- restricting to 60 ups
        if time_delta >= 1./60. { 
            process_input(&mut window);
            camera.update(&mut window, time_delta);
            prev_time = curr_time;
        }

        // Fps is not restricted, but it could be with the same time_delta

        // Handle window events
        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            // passing all events to the camera - maybe not required? but will have to move the match block outside and create separate handlers
            camera.handle_window_event(&event, &time_delta);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                }
                _ => {}
            }
        }

        // Rendering
        unsafe {
            // Clearing the screen
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            // Drawing the object
            // default_shader.use_program(); // done by camera.set_cam_matrix, not sure if this will be repeated by the driver
            
            // Set transformation matrices
            camera.force_set_cam_matrix(&default_shader); // force set since we are switching programs
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            // Set light uniforms - directional light
            default_shader.set_vec3_values(c_str!("dirLight.ambient"),  0.5, 0.5, 0.5);
            default_shader.set_vec3_values(c_str!("dirLight.diffuse"),  1.0, 1.0, 1.0);
            default_shader.set_vec3_values(c_str!("dirLight.specular"),  1.0,1.0,1.0);
            // rotating the directional light
            let light_dir = glm::rotate_vec3(&glm::vec3(0.,1.,0.),glfw.get_time() as f32, &glm::Vec3::z_axis());
            default_shader.set_vec3(c_str!("dirLight.direction"), light_dir);

            // Stencil - store 1's wherever we draw anything
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE); // replace values in stencil buffer if both checks pass
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF); // always replace with 1
            gl::StencilMask(0xFF); // Enable writing to the buffer

            // Drawing ferris at the center
            default_shader.set_mat4(c_str!("model"), model_matrix_f);
            model_ferris.draw(&default_shader);

            // Draw outline using stencil (by not drawing where stencil buffer = 1)
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00); // disable writing to buffer
            gl::Disable(gl::DEPTH_TEST); // disable depth buffer since we want to draw the outline regardless of depth

            camera.force_set_cam_matrix(&outline_shader); // force set since we are switching programs
            outline_shader.set_mat4(c_str!("model"), glm::scale(&model_matrix_f, &glm::vec3(1.1,1.1,1.1)));
            model_ferris.draw(&outline_shader);

            gl::StencilMask(0xFF);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::Enable(gl::DEPTH_TEST);
            
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for window events
        glfw.poll_events();
    }
    
    // --Terminate----------------------------------------------------------------------------------------------------------------- //

    //unsafe {
        // gl::DeleteVertexArrays(1, &vao);
        // gl::DeleteBuffers(1, &vbo);
        // gl::DeleteVertexArrays(1, &light_vao);
        // gl::DeleteBuffers(1, &light_vbo);
    //}
}

/// Function to process input
fn process_input(_window: &mut glfw::PWindow) {
    // Inputs can be processed by going through events instead
    // using for (_, event) in glfw::flush_messages(&events) { match event ... }
    // if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
    //     window.set_should_close(true);
    // }

    // Not used, but can process inputs like camera.update()
}
