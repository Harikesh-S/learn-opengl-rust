// Loading sponza model, with depth buffer visualization
// Press N to change shader

use std::ffi::CStr;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 4 : Part 1 : Sponza model, N - change shader (Normal, Visualize Depth Buffer Linear, Fog)\nPlease download the model (link in README) and update the path in this fileFil";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Depth Buffer";

pub fn main_4_1_1() {

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
    let shaders = [
        Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/1_default.frag"),
        Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/1_depth_buffer_linear.frag"),
        Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/1_fog.frag"),
    ];
    let shader_names = [
        "Default","Visualize Depth Buffer - Linear","Fog"
    ];
    let mut current_shader = 0;

    println!("Current shader : {}", shader_names[current_shader]);

    // Loading models
    let mut model_sponza = Model::new();
    model_sponza.load_model("C:/Users/harik/Downloads/Sponza-master/Sponza-master/sponza.obj");

    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Camera
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(-1220.,100.,-45.), 0., 0., 0., WINDOW_WIDTH, WINDOW_HEIGHT);
    camera.near_plane = 1.; // to reduce z fighting
    camera.far_plane = 3000.; // needed since the model is very large without scaling down
    camera.speed = 100.; // needed since the model is very large without scaling down
    camera.update_cam_matrix(false); // recalculate matrices

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
    let mut frame_counter = 0;
    let mut fps;

    // Model matrices and transformations

    let identity = glm::Mat4::identity();
    let model_matrix_f = identity;

    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    unsafe {
    gl::Enable(gl::CULL_FACE);
    gl::CullFace(gl::FRONT);
    gl::FrontFace(gl::CW);
}

    // --Render loop--------------------------------------------------------------------------------------------------------------- //

    while !window.should_close() {

        // Time
        let curr_time = glfw.get_time();
        let time_delta = curr_time - prev_time;

        frame_counter += 1;

        // Update -- restricting to 60 ups
        if time_delta >= 1./60. { 
            process_input(&mut window);
            camera.update(&mut window, time_delta);
            fps = format!("FPS : {} / MS : {}", (1./time_delta as f32)*(frame_counter as f32) , (time_delta as f32/(frame_counter as f32)*1000.));
            window.set_title(format!("{} {}",WINDOW_TITLE,fps).as_str());
            prev_time = curr_time;
            frame_counter = 0;
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
                glfw::WindowEvent::Key(glfw::Key::N, _, glfw::Action::Press, _) => {
                    current_shader = (current_shader + 1) % shaders.len();
                    camera.force_set_cam_matrix(&shaders[current_shader]);
                    println!("Current shader : {}", shader_names[current_shader]);
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

            let default_shader = &shaders[current_shader];

            // Drawing the object
            default_shader.use_program();
            
            // Set transformation matrices
            camera.set_cam_matrix(&default_shader);
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            // Set light uniforms - directional light
            default_shader.set_vec3_values(c_str!("dirLight.ambient"),  0.2, 0.2, 0.2);
            default_shader.set_vec3_values(c_str!("dirLight.diffuse"),  1.0, 1.0, 1.0);
            default_shader.set_vec3_values(c_str!("dirLight.specular"),  1.0,1.0,1.0);
            // rotating the directional light
            //let light_dir = glm::rotate_vec3(&glm::vec3(0.,1.,0.),glfw.get_time() as f32, &glm::Vec3::z_axis());
            let light_dir = glm::vec3(0.,1.,1.);
            default_shader.set_vec3(c_str!("dirLight.direction"), light_dir);

            // For depth buffer - linear / fog
            default_shader.set_float(c_str!("far"), camera.far_plane);
            default_shader.set_float(c_str!("near"), camera.near_plane);

            // Drawing ferris at the center
            default_shader.set_mat4(c_str!("model"), model_matrix_f);
            model_sponza.draw(&default_shader);
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
