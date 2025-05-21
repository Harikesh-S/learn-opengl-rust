// Ex1 - Invert only the second texture's orientation (done in the shader)

use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::path::Path;
use std::os::raw::c_void;

use gl::{self, types::*};
use glfw::{self, Context};
use image;

use crate::shader::Shader;

const MESSAGE : &str = "Chapter 1 : Part 4 : Ex 1 : Invert only the second texture through the shader";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Textures - Ex1";

const VERTICES: [GLfloat; 32] = [
    // Vertices       // Colors       // TexCoord
    -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0, // bottom left
     0.5, -0.5, 0.0,  0.0, 1.0, 1.0,  1.0, 0.0, // bottom right
     0.5,  0.5, 0.0,  1.0, 1.0, 1.0,  1.0, 1.0, // top right
    -0.5,  0.5, 0.0,  1.0, 0.0, 1.0,  0.0, 1.0, // top left 
];

const INDICES: [GLint; 6] = [
    0, 1, 3,
    1, 2, 3
];

pub fn main_1_4_1() {

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
    let (mut window, _events) = glfw
        .create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window!");

    // Set current context , enable polling
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load open gl functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // --Creating OpenGL Obhjects-------------------------------------------------------------------------------------------------- //

    // Shader Program

    let default_shader = Shader::new("./src/_1_getting_started/shaders/4_0_default.vert","./src/_1_getting_started/shaders/4_1_default.frag");

    // VAO, VBO, EBO

    let (vao, vbo, ebo) = unsafe {

        // Generate Vertex array object, vertex buffer object and elemenet array buffer objects
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        
        // Bind VAO to make it active
        gl::BindVertexArray(vao);

        // Bind VBO and store vertex data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &VERTICES[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        // Bind EBO and store index data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDICES.len() * mem::size_of::<GLint>()) as GLsizeiptr, &INDICES[0] as *const i32 as *const c_void, gl::STATIC_DRAW);

        // Link vertex attributes 
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as i32, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as i32, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);

        // Note: we can safely unbind VBO since it is bound to the VAO's vertex attribute from VertedAttribPointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // we CANNOT unbind the EBO while the VAO is active
        // Unbinding VAO/VBO is usually not required, since we will always use BindVertexArray

        (vao, vbo, ebo)
    };

    // Texture

    let brick_texture = unsafe {

        // Generate Texture
        let mut texture: GLuint = 0;
        gl::GenTextures(1, &mut texture);

        // Bind texture to target and texture unit
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Interpolation
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // Border wrap
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
        // Only required for CLAMP_TO_BORDER
        //gl::TexParameterfv(gl::TEXTURE_2D,gl::TEXTURE_BORDER_COLOR,[1.0, 1.0, 1.0, 1.0].as_ptr());
        
        // Loading image from file
        let img = image::open(&Path::new("./resources/textures/wall.jpg")).expect("Failed to load texture").flipv().into_rgb8();

        // Store data into texture
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGB as i32,img.width() as i32,img.height() as i32,0,gl::RGB as u32,gl::UNSIGNED_BYTE,img.as_ptr() as *const u8 as *const c_void);

        // Generate mip maps for the texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

        texture
    };

    let face_texture = unsafe {

        // Generate Texture
        let mut texture: GLuint = 0;
        gl::GenTextures(1, &mut texture);

        // Bind texture to target and texture unit
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Interpolation
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // Border wrap
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
        // Only required for CLAMP_TO_BORDER
        //gl::TexParameterfv(gl::TEXTURE_2D,gl::TEXTURE_BORDER_COLOR,[1.0, 1.0, 1.0, 1.0].as_ptr());
        
        // Loading image from file
        let img = image::open(&Path::new("./resources/textures/ferris.png")).expect("Failed to load texture").flipv().into_rgba8();

        // Store data into texture
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32,img.width() as i32,img.height() as i32,0,gl::RGBA as u32,gl::UNSIGNED_BYTE,img.as_ptr() as *const u8 as *const c_void);

        // Generate mip maps for the texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

        texture
    };
    
    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Viewport
    unsafe {
        // Setting the viewport
        gl::Viewport(0, 0, WINDOW_WIDTH as GLint, WINDOW_HEIGHT as GLint);

        // Register frame buffer size callback
        // This can be done with events instead glfw::WindowEvent::FrameBufferSize
        window.set_framebuffer_size_callback(frame_buffer_size_callback);
    }

    // Wireframe mode - optional
    // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

    // Set texture unit uniform in shader
    unsafe { 
        default_shader.use_program();
        default_shader.set_int(c_str!("tex0"), 0); 
        default_shader.set_int(c_str!("tex1"), 1); 
    }
    
    // --Render loop--------------------------------------------------------------------------------------------------------------- //

    while !window.should_close() {

        // Process inputs
        process_input(&mut window);

        // Rendering
        unsafe {

            // Clearing the screen
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Drawing the triangle
            default_shader.use_program();

            // Bind Textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, brick_texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, face_texture);
            
            // Bind VAO
            gl::BindVertexArray(vao);
            
            // Draw
            gl::DrawElements(gl::TRIANGLES,INDICES.len() as i32, gl::UNSIGNED_INT, ptr::null());

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
        gl::DeleteBuffers(1, &ebo);
        default_shader.delete();
        // Terminate GLFW
        // glfwTerminate(); // Not required, included in Glfw Drop
    }
}

/// Function to process input
fn process_input(window: &mut glfw::PWindow) {
    // Inputs can be processed by going through events instead
    // using for (_, event) in glfw::flush_messages(&events) { match event ... }
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}

/// Callback function for change in frame buffer size
fn frame_buffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}