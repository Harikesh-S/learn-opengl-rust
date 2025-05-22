// Ex 2 - Experimenting with various materials
// emerald, obsidian, chrome
//          light
// gold, green rubber, white plastic
use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::os::raw::c_void;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 2 : Part 3 : Ex2 : Experimenting with materials";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Materials";

// Cube vertices - without TexCoord
const VERTICES: [GLfloat; 216] = [
    // Vertices         // Normals
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
     0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 

    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
];

const LIGHT_LOCATION : glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);
const LIGHT_SCALE : glm::Vec3 = glm::Vec3::new(0.2, 0.2, 0.2);

const CUBE_POSITIONS : [(glm::Vec3, glm::Vec3, glm::Vec3, glm::Vec3, GLfloat);6] = [
    (glm::Vec3::new(-2., 1.,0.), glm::Vec3::new(0.0215,0.1745,0.0215), glm::Vec3::new(0.07568,0.61424,0.07568), glm::Vec3::new(0.633,0.727811,0.633), 0.6),  // emerald
    (glm::Vec3::new(0., 1.,0.), glm::Vec3::new(0.05375,0.05,0.06625), glm::Vec3::new(0.18275,0.17,0.22525), glm::Vec3::new(0.332741,0.328634,0.346435), 0.3),  // obsidian
    (glm::Vec3::new(2., 1.,0.), glm::Vec3::new(0.25,0.25,0.25), glm::Vec3::new(0.4,0.4,0.4), glm::Vec3::new(0.774597,0.774597,0.774597), 0.6),  // chrome
    (glm::Vec3::new(-2.,-1.,0.), glm::Vec3::new(0.24725,0.1995,0.0745), glm::Vec3::new(0.75164,0.60648,0.22648), glm::Vec3::new(0.628281,0.555802,0.366065), 0.4),  // gold
    (glm::Vec3::new(0.,-1.,0.), glm::Vec3::new(0.0,0.0,0.0), glm::Vec3::new(0.1,0.35,0.1), glm::Vec3::new(0.45,0.55,0.45), 0.25),  // green rubber
    (glm::Vec3::new(2.,-1.,0.), glm::Vec3::new(0.05,0.05,0.05), glm::Vec3::new(0.5,0.5,0.5), glm::Vec3::new(0.7,0.7,0.7), 0.078125),  // white plastic
]; // ! note shininess has to be multiplied by 128

pub fn main_2_3_2() {

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

    // --Creating OpenGL Obhjects-------------------------------------------------------------------------------------------------- //

    // Shader Program
    let default_shader = Shader::new("./src/_2_lighting/shaders/2_0_default.vert","./src/_2_lighting/shaders/3_0_default.frag");
    let light_shader = Shader::new("./src/_2_lighting/shaders/1_0_default.vert","./src/_2_lighting/shaders/3_1_light.frag");

    // VAO, VBO, EBO
    let (vao, vbo) = unsafe {

        // Generate Vertex array object, vertex buffer object and elemenet array buffer objects
        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        // Bind VAO to make it active
        gl::BindVertexArray(vao);

        // Bind VBO and store vertex data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &VERTICES[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        // Link vertex attributes 
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as i32, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // Note: we can safely unbind VBO since it is bound to the VAO's vertex attribute from VertedAttribPointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // we CANNOT unbind the EBO while the VAO is active
        // Unbinding VAO/VBO is usually not required, since we will always use BindVertexArray

        (vao, vbo)
    };

    
    let (light_vao, light_vbo) = unsafe {
        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &VERTICES[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (vao, vbo)
    };

    // No Textures - yet
    
    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Camera
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(0.,0.,4.), 0., 0., -90., WINDOW_WIDTH, WINDOW_HEIGHT);

    // Viewport
    unsafe {
        // Setting the viewport
        gl::Viewport(0, 0, WINDOW_WIDTH as GLint, WINDOW_HEIGHT as GLint);

        // Register frame buffer size callback
        // This can be done with events instead glfw::WindowEvent::FrameBufferSize
        // window.set_framebuffer_size_callback(frame_buffer_size_callback);
    }

    // Wireframe mode - optional
    // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

    // Set texture unit uniform in shader
    unsafe { 
        default_shader.use_program();
        default_shader.set_int(c_str!("tex0"), 0); 
        default_shader.set_int(c_str!("tex1"), 1); 
    }

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
            gl::BindVertexArray(vao);
            
            // Set transformation matrices
            camera.set_cam_matrix(&default_shader);
            // Light position and colors
            default_shader.set_vec3_values(c_str!("light.ambient"),  1.,1.,1.);      // material table expects 1.0 for all 
            default_shader.set_vec3_values(c_str!("light.diffuse"),  1.,1.,1.);      
            default_shader.set_vec3_values(c_str!("light.specular"),  1.,1.,1.);     
            default_shader.set_vec3(c_str!("light.position"), LIGHT_LOCATION);          // Setting light position to the fragment shader - this can be done outside loop
            // View position for specular highlights based on viewer
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            let mut i = 0;
            while i < CUBE_POSITIONS.len() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &CUBE_POSITIONS[i].0);
                
                // Setting model, and uniforms for fragment shader
                default_shader.set_mat4(c_str!("model"),  model); 
                // Set material
                default_shader.set_vec3(c_str!("material.ambient"),  CUBE_POSITIONS[i].1);
                default_shader.set_vec3(c_str!("material.diffuse"),  CUBE_POSITIONS[i].2);
                default_shader.set_vec3(c_str!("material.specular"),  CUBE_POSITIONS[i].3);
                default_shader.set_float(c_str!("material.shininess"),  CUBE_POSITIONS[i].4 * 128.0 as GLfloat);

                gl::DrawArrays(gl::TRIANGLES,0, 36);

                i += 1;
            }

            gl::DrawArrays(gl::TRIANGLES,0, 36);

            // Drawing the light source
            light_shader.use_program();
            gl::BindVertexArray(light_vao);
            
            let mut light_model = glm::Mat4::identity();
            light_model = glm::translate(&light_model, &LIGHT_LOCATION);
            light_model = glm::scale(&light_model, &LIGHT_SCALE);

            camera.set_cam_matrix(&light_shader);
            light_shader.set_mat4(c_str!("model"),  light_model); 

            light_shader.set_vec3_values(c_str!("lightColor"),  1.,1.,1.);     // Materials Ex1

            gl::DrawArrays(gl::TRIANGLES,0, 36);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for window events
        glfw.poll_events();
    }
    
    // --Terminate----------------------------------------------------------------------------------------------------------------- //

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &light_vbo);
        default_shader.delete();
        light_shader.delete();
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