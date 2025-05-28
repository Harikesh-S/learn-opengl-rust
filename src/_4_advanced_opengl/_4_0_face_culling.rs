// Face Culling - using subdivion from _4_e_subdivision

const MODEL_PATH : &str = "./resources/models/ferris3d_v1.0.obj";
// using existing model to not add large files to the repo, may help people with poor internet (me in the past) plus is a good excuse to implement mesh.subdivide()
// feel free to replace the model, larger models give more noticable results 
// you may have to change the FrontFace back to CCW if you are using a different model
const SUBDIVIDE_MODEL : u32 = 3; // Make sure to reduce subdivide_model to 0 if you are using a larger model
// feel free to increase this count if your computer can handle it (wireframe mode reduces fps so disable WIREFRAME_MODE if required)

use std::ffi::CStr;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 4 : Part 4 : Face Culling, Space - Toggle face culling, M - Toggle Wireframe mode";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Face Culling";
const WIREFRAME_MODE : bool = true; // diable default wireframe mode here

const MODEL_POSITION: glm::Vec3 = glm::Vec3::new(0.,0.,0.);

pub fn main_4_4() {

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
    let default_shader = Shader::new("./src/_3_model_loading/shaders/1_default.vert","./src/_4_advanced_opengl/shaders/4_default.frag");
    
    // Loading models
    let mut model = Model::new();
    //model.load_model("./resources/models/unecessarily_detailed_torus.obj"); // just a torus from blender with max vertices subdivided 
    model.load_model(MODEL_PATH);   
    model.subdivide_meshes(SUBDIVIDE_MODEL);

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
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(0.,0.5,5.), 0., -5., -90., WINDOW_WIDTH, WINDOW_HEIGHT);

    // Viewport
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as GLint, WINDOW_HEIGHT as GLint);
    }

    // Wireframe mode - optional
    if WIREFRAME_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
    }

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
    let mut frame_counter = 0;

    // Get all model matrices
    let identity = glm::Mat4::identity();
    let model_matrix = glm::translate(&identity, &MODEL_POSITION);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(0)); // disable vsync to uncap fps

    // Variables changed by user for demo
    let mut cull_face = false;
    let mut wireframe_mode = WIREFRAME_MODE;
    println!("Face culling : {}", cull_face);
    println!("Wireframe Mode : {}", wireframe_mode);

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

            window.set_title(format!("{} Face Culling {} FPS : {} / MS : {}",WINDOW_TITLE,cull_face,(1./time_delta as f32)*(frame_counter as f32) , (time_delta as f32/(frame_counter as f32)*1000.)).as_str());
            
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
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
                    cull_face = !cull_face;
                    println!("Face culling : {}", cull_face);
                    if cull_face {
                        unsafe {
                            gl::Enable(gl::CULL_FACE);
                            gl::CullFace(gl::FRONT);
                            gl::FrontFace(gl::CW);
                        }
                    }
                    else {
                        unsafe {
                            gl::Disable(gl::CULL_FACE);
                        }
                    }
                }
                glfw::WindowEvent::Key(glfw::Key::M, _, glfw::Action::Press, _) => {
                    wireframe_mode = !wireframe_mode;
                    println!("Wireframe Mode : {}", wireframe_mode);
                    if wireframe_mode {
                        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
                    }
                    else {
                        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
                    }
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
            default_shader.set_vec3_values(c_str!("dirLight.ambient"),  0.2, 0.2, 0.2);
            default_shader.set_vec3_values(c_str!("dirLight.diffuse"),  1.0, 1.0, 1.0);
            default_shader.set_vec3_values(c_str!("dirLight.specular"),  1.0,1.0,1.0);
            // rotating the directional light
            //let light_dir = glm::rotate_vec3(&glm::vec3(0.,1.,0.),glfw.get_time() as f32, &glm::Vec3::z_axis());
            let light_dir = glm::vec3(-1.,-1.,0.);
            default_shader.set_vec3(c_str!("dirLight.direction"), light_dir);

            // Drawing a lot of models, - 
            default_shader.set_mat4(c_str!("model"), model_matrix);
            model.draw(&default_shader);
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