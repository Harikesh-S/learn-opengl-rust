// Ex1 - draw two triangles with mulitple VAOs and shaders

use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::raw::c_void;

use gl::{self, types::*};
use glfw::{self, Context};

const MESSAGE : &str = "Chapter 1 : Part 2 : Ex 3 : Two triangles (multiple VAOs and shaders)";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Hello Triangle - Ex3";

const VERTICES: [GLfloat; 9] = [
    // Vertices
    -0.5, -0.5, 0.0,  // 1 - left
     0.0, -0.5, 0.0,  // 1 - right
    -0.5,  0.0, 0.0,  // 1 - top
];

const VERTICES_2: [GLfloat; 9] = [
     0.0,  0.5, 0.0,  // 2 - left
     0.5,  0.5, 0.0,  // 2 - right
     0.5, -0.0, 0.0,  // 2 - bottom
];

const VERTEX_SHADER : &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos, 1.0);
}
"#;

const FRAGMENT_SHADER : &str = r#"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
"#;

// Second shader code, reusing vertex shader code
const FRAGMENT_SHADER_2 : &str = r#"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
} 
"#;

pub fn main_1_2_3() {

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
    let default_shader = unsafe {

        // Creating shaders
        let vertex_shader: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_shader: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Setting shader souce
        let vertex_shader_source = CString::new(VERTEX_SHADER.as_bytes()).unwrap();
        let fragment_shader_source = CString::new(FRAGMENT_SHADER.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), ptr::null());
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), ptr::null());

        // Compiling shaders
        gl::CompileShader(vertex_shader);
        gl::CompileShader(fragment_shader);

        // Creating shader program and linking
        let shader_program: GLuint = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Errors
        check_compile_errors(vertex_shader, "VERTEX");
        check_compile_errors(fragment_shader, "FRAGMENT");
        check_compile_errors(shader_program, "PROGRAM");

        // Deleting shaders since they are already attached to program
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };

    // Second shader program
    let default_shader_2 = unsafe {

        // Creating shaders
        let vertex_shader: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_shader: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Setting shader souce
        let vertex_shader_source = CString::new(VERTEX_SHADER.as_bytes()).unwrap();
        let fragment_shader_source = CString::new(FRAGMENT_SHADER_2.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), ptr::null());
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), ptr::null());

        // Compiling shaders
        gl::CompileShader(vertex_shader);
        gl::CompileShader(fragment_shader);

        // Creating shader program and linking
        let shader_program: GLuint = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Errors
        check_compile_errors(vertex_shader, "VERTEX");
        check_compile_errors(fragment_shader, "FRAGMENT");
        check_compile_errors(shader_program, "PROGRAM");

        // Deleting shaders since they are already attached to program
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };


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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Note: we can safely unbind VBO since it is bound to the VAO's vertex attribute from VertedAttribPointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // we CANNOT unbind the EBO while the VAO is active
        // Unbinding VAO/VBO is usually not required, since we will always use BindVertexArray

        (vao, vbo)
    };

    // Creating second vao using VERTICES_2
    let (vao2, vbo2) = unsafe {

        // Generate Vertex array object, vertex buffer object and elemenet array buffer objects
        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        // Bind VAO to make it active
        gl::BindVertexArray(vao);

        // Bind VBO and store vertex data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (VERTICES_2.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &VERTICES_2[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        // Link vertex attributes 
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        // Note: we can safely unbind VBO since it is bound to the VAO's vertex attribute from VertedAttribPointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // we CANNOT unbind the EBO while the VAO is active
        // Unbinding VAO/VBO is usually not required, since we will always use BindVertexArray

        (vao, vbo)
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
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
    
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
            gl::UseProgram(default_shader);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::UseProgram(default_shader_2);
            gl::BindVertexArray(vao2);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

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
        gl::DeleteVertexArrays(1, &vao2);
        gl::DeleteBuffers(1, &vbo2);
        gl::DeleteShader(default_shader);
        gl::DeleteShader(default_shader_2);
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

/// Function to verify shader compilation
unsafe fn check_compile_errors(shader: GLuint, shader_type: &str) {
    let mut success = gl::FALSE as GLint;
    let mut log = Vec::with_capacity(512);
    unsafe {
        log.set_len(512 - 1); // subtract 1 to skip the trailing null character~
        if shader_type != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
                println!(
                    "ERROR: SHADER_COMPILATION_ERROR of type {}\n{:?}\n\
                        -- ------------------------- --",
                    shader_type,
                    std::str::from_utf8_mut(&mut log));
            }
        }
        else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 512, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
                println!(
                    "ERROR: PROGRAM_LINKING_ERROR of type {}\n{:?}\n\
                        -- ------------------------- --",
                    shader_type,
                    std::str::from_utf8(&log));
            }
        }
    }
}