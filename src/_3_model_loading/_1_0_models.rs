// Model Loading using tobj

use std::ffi::CString;
use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::path::Path;
use std::os::raw::c_void;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;
use tobj;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 3 : Part 1 : Model Loading using tobj";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Model Loading";

pub fn main_3_1() {

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
    let default_shader = Shader::new("./src/_3_model_loading/shaders/1_t_default.vert","./src/_3_model_loading/shaders/1_t_default.frag");
    //let default_shader = Shader::new("./src/_3_model_loading/shaders/1_t_default.vert","./src/_3_model_loading/shaders/1_t_default_color.frag");
    //let light_shader = Shader::new("./src/_2_lighting/shaders/1_0_default.vert","./src/_2_lighting/shaders/3_1_light.frag");
    
    let mut object = Model::new();
    // object.load_model("./resources/models/vehicle-vintage-racer.obj");
    object.load_model("./resources/models/Sting-Sword.obj");


    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Camera
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(-2.,0.,2.), 0., 0., -40., WINDOW_WIDTH, WINDOW_HEIGHT);

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

    // Time
    let mut prev_time = glfw.get_time();

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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Drawing the object
            default_shader.use_program();
            
            // Set transformation matrices
            camera.set_cam_matrix(&default_shader);

             default_shader.set_mat4(c_str!("model"), glm::Mat4::identity());
            
            // Set material
            // default_shader.set_int(c_str!("material.diffuse"), 0);  // Using texture unit 0 for diffuse map
            // default_shader.set_int(c_str!("material.specular"), 1);  // Using texture unit 1 for specular map
            default_shader.set_float(c_str!("material.shininess"),  32.0); 
            // View position for specular highlights based on viewer
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            // Used by directional light
            default_shader.set_vec3_values(c_str!("dirLight.ambient"),  0.2, 0.2, 0.2);      // low because we dont want amient color to be too dominant
            default_shader.set_vec3_values(c_str!("dirLight.diffuse"),  0.5, 0.5, 0.5);      // exact color that we want
            default_shader.set_vec3_values(c_str!("dirLight.specular"),  0.5,0.5,0.5);         // high because we want the light's color in the highlight
            default_shader.set_vec3_values(c_str!("dirLight.direction"), -0.2, -1.0, -0.3); // direction that the light is pointing to

            object.draw(&default_shader);

            // // Set the maps (textures)
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            // gl::ActiveTexture(gl::TEXTURE1);
            // gl::BindTexture(gl::TEXTURE_2D, specular_map);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for window events
        glfw.poll_events();
    }
    
    // --Terminate----------------------------------------------------------------------------------------------------------------- //

    unsafe {
        // gl::DeleteVertexArrays(1, &vao);
        // gl::DeleteBuffers(1, &vbo);
        // gl::DeleteVertexArrays(1, &light_vao);
        // gl::DeleteBuffers(1, &light_vbo);
    }
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
