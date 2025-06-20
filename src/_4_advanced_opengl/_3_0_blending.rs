// Blending and Discarding fragments

use std::ffi::CStr;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 4 : Part 3 : Blending and Discarding fragments";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Blending";


const FERRIS_POSITION: glm::Vec3 = glm::Vec3::new(0.,0.,-5.);
const GRASS_POSITIONS: [glm::Vec3;4] = [
    glm::Vec3::new(0.,0.5,-3.),
    glm::Vec3::new(0.5,0.5,-5.),
    glm::Vec3::new(-2.,0.5,-2.),
    glm::Vec3::new(1.,0.5,-2.),
];
const GLASS_WINDOW_POSITIONS: [glm::Vec3;4] = [
    glm::Vec3::new(0.,0.5,-1.),
    glm::Vec3::new(0.5,0.5,3.),
    glm::Vec3::new(-2.,0.5,0.),
    glm::Vec3::new(1.,0.5,-1.5),
];

pub fn main_4_3() {

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
    let default_shader = Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/3_default.frag");
    
    // Loading models
    let mut model_ground = Model::new();
    model_ground.load_plane("./resources/textures/ground.jpg",10.0,10.0);
    let mut model_grass = Model::new();
    model_grass.load_plane("./resources/textures/grass.png",1.,1.);
    let mut model_window = Model::new();
    model_window.load_plane("./resources/textures/window.png",0.5,1.);
    let mut model_ferris = Model::new();
    model_ferris.load_model("./resources/models/ferris3d_v1.0.obj");

    // Set texture unit 0 as a blank texture
    // Required since shader is expecting a emission texture but none are provided
    // Not required since we have use_texture_* set in the shader, but this is another option for the same effect
    // let blank_texture : GLuint;
    // unsafe { 
    //     blank_texture = Model::load_texture("./resources/textures/blank.png","");
    //     gl::ActiveTexture(gl::TEXTURE0);
    //     gl::BindTexture(gl::TEXTURE_2D, blank_texture);
    // }


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

    // Blending - for semi transparent objects
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);  
    }

    // Time
    let mut prev_time = glfw.get_time();

    // Get all model matrices
    let identity = glm::Mat4::identity();
    let model_matrix_f = glm::translate(&identity, &FERRIS_POSITION);
    let model_matrix_ground = glm::rotate(&identity, f32::to_radians(-90.),&glm::Vec3::x());

    let mut model_matrices_transparent: Vec<(glm::Mat4, &Model, &glm::Vec3, f32)> = Vec::with_capacity(GRASS_POSITIONS.len()+GLASS_WINDOW_POSITIONS.len());
    for grass_position in &GRASS_POSITIONS {
        model_matrices_transparent.push((glm::translate(&identity, &grass_position),&model_grass, grass_position, 0.));
    }
    for window_position in &GLASS_WINDOW_POSITIONS {
        model_matrices_transparent.push((glm::translate(&identity, &window_position),&model_window, window_position, 0.));
    }
    

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
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            // Set light uniforms - directional light
            default_shader.set_vec3_values(c_str!("dirLight.ambient"),  0.5, 0.5, 0.5);
            default_shader.set_vec3_values(c_str!("dirLight.diffuse"),  1.0, 1.0, 1.0);
            default_shader.set_vec3_values(c_str!("dirLight.specular"),  1.0,1.0,1.0);
            // rotating the directional light
            //let light_dir = glm::rotate_vec3(&glm::vec3(0.,1.,0.),glfw.get_time() as f32, &glm::Vec3::z_axis());
            let light_dir = glm::vec3(-1.,-1.,0.);
            default_shader.set_vec3(c_str!("dirLight.direction"), light_dir);

            // Draw
            default_shader.set_mat4(c_str!("model"), model_matrix_f);
            model_ferris.draw(&default_shader);

            default_shader.set_mat4(c_str!("model"), model_matrix_ground);
            model_ground.draw(&default_shader);

            // Sorting all transparent objects - for proper blending
            // only if camera matrix is updated
            if camera.is_matrix_updated  {
                for model_transparent in &mut model_matrices_transparent {
                    model_transparent.3 = glm::length2(&(camera.position - *model_transparent.2)); // calculate distance from camera, not using sqrt since we are only comparing
                }

                // sort in reverse order
                model_matrices_transparent.sort_by(|a,b| b.3.total_cmp(&a.3));
            }

            for (matrix, model, _, _) in &model_matrices_transparent {
                default_shader.set_mat4(c_str!("model"), *matrix);
                model.draw(&default_shader);
            }
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for window events
        glfw.poll_events();
    }
    
    // --Terminate----------------------------------------------------------------------------------------------------------------- //

    // unsafe {
    //     // gl::DeleteVertexArrays(1, &vao);
    //     // gl::DeleteBuffers(1, &vbo);
    //     // gl::DeleteVertexArrays(1, &light_vao);
    //     // gl::DeleteBuffers(1, &light_vbo);
    //     gl::DeleteTextures(1, &blank_texture);
    // }
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
