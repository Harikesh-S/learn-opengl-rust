// Frame Buffers
// Note: frame buffer texture size is not changed with window size

const MODEL_PATH : &str = "./resources/models/ferris3d_v1.0.obj";

use std::ffi::CStr;
use std::ptr;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::model::Model;
use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 4 : Part 5 : Frame Buffers, N - Change frame buffer shader";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Frame Buffers";
const WIREFRAME_MODE : bool = false; // diable default wireframe mode here

const MODEL_POSITION: glm::Vec3 = glm::Vec3::new(0.,0.,0.);

pub fn main_4_5() {

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
    // Shaders for frame buffer
    let screen_shaders = [
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer.frag"),
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer_inversion.frag"),
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer_grayscale.frag"),
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer_kernel_sharpen.frag"),
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer_kernel_blur.frag"),
        Shader::new("./src/_4_advanced_opengl/shaders/5_frame_buffer.vert","./src/_4_advanced_opengl/shaders/5_frame_buffer_kernel_edge.frag"),
    ];
    let shader_names = [
        "Default","Inversion","Grayscale","Kernel - Sharpen","Kernel - Blur","Kernel - Edge Detection"
    ];
    let mut current_shader = 0;
    println!("Current frame buffer shader : {}", shader_names[current_shader]);

    // Loading models
    let mut model = Model::new();
    let mut frame_buffer_quad = Model::new();
    model.load_model(MODEL_PATH);   
    frame_buffer_quad.load_plane_blank(1.0);
    
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

    // Frame Buffers
    let mut frame_buffer : GLuint = 0;
    let mut texture_color_buffer: GLuint = 0;
    let mut rbo : GLuint = 0;
    unsafe {
        // Generating and binding the frame buffer
        gl::GenFramebuffers(1, &mut frame_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);

        // Generating the texture used for the buffer
        gl::GenTextures(1, &mut texture_color_buffer);
        gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_INT, ptr::null());

        // Other parameters are not used in this example
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

        // Attaching the texture to the bound frame buffer
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture_color_buffer, 0);

        // Creating a render buffer for depth/stencil since we are not sampling them
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        // Attaching the render buffer to the bound frame buffer
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        // Check if the frame buffer is compiled
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("ERROR::FRAMEBUFFER::Framebuffer is not complete!");
        }

        // Unbind the buffer to not accidentally render to the wrong buffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
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

            window.set_title(format!("{} FPS : {} / MS : {}",WINDOW_TITLE,(1./time_delta as f32)*(frame_counter as f32) , (time_delta as f32/(frame_counter as f32)*1000.)).as_str());
            
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
                glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                }
                glfw::WindowEvent::Key(glfw::Key::N, _, glfw::Action::Press, _) => {
                    current_shader = (current_shader + 1) % screen_shaders.len();
                    camera.force_set_cam_matrix(&screen_shaders[current_shader]);
                    println!("Current frame buffer shader : {}", shader_names[current_shader]);
                }
                _ => {}
            }
        }

        // Rendering
        unsafe {
            // First pass
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);
            gl::Enable(gl::DEPTH_TEST);
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

            // Second pass
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            screen_shaders[current_shader].use_program();
            screen_shaders[current_shader].set_int(c_str!("screenTexture"), 0 as i32);
            gl::ActiveTexture(gl::TEXTURE0 as u32);
            gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);
            frame_buffer_quad.draw(&screen_shaders[current_shader]);

        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for window events
        glfw.poll_events();
    }
    
    // --Terminate----------------------------------------------------------------------------------------------------------------- //

    unsafe {
    //     // gl::DeleteVertexArrays(1, &vao);
    //     // gl::DeleteBuffers(1, &vbo);
    //     // gl::DeleteVertexArrays(1, &light_vao);
    //     // gl::DeleteBuffers(1, &light_vbo);
    //     gl::DeleteTextures(1, &blank_texture);
        gl::DeleteFramebuffers(1, &frame_buffer);
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