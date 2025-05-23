// Ex 1 : Recreating environments

use std::ffi::CString;
use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::path::Path;
use std::os::raw::c_void;

use gl::{self, types::*};
use glfw::{self, Context};
use nalgebra_glm as glm;

use crate::shader::Shader;
use crate::camera::{FreeCamera, Camera};

const MESSAGE : &str = "Chapter 2 : Part 6 : Ex 1 : Recreating environments, N - next environment";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Multiple lights";

// Cube vertices - with TexCoords
const VERTICES: [GLfloat; 288] = [
    // positions          // normals           // texture coords
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
     0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
     0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
     0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
     0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
];

const LIGHT_LOCATIONS : [glm::Vec3;4] = [
    glm::Vec3::new(5.0, 0.0, -4.0),
    glm::Vec3::new(-2.0, -2.0, 0.0),
    glm::Vec3::new(-6.0, 1.0, -6.0),
    glm::Vec3::new(2.0, 2.0, 2.0)];
const LIGHT_SCALE : glm::Vec3 = glm::Vec3::new(0.2, 0.2, 0.2);
struct EnvColors<'a>{
    bg : glm::Vec3,
    dir : glm::Vec3,
    point: glm::Vec3,
    spot : glm::Vec3,
    name : &'a str,
}

const ENVIRONMENTS : [EnvColors;4] = [
    EnvColors{ name : "Desert",
        bg : glm::Vec3::new(0.74, 0.51, 0.29),
        dir : glm::Vec3::new(0.74, 0.51, 0.29),
        point : glm::Vec3::new(1.0, 0.0, 0.0),
        spot : glm::Vec3::new(0.0, 0.0, 0.0)},  
    EnvColors{ name : "Factory",
        bg : glm::Vec3::new(0.2, 0.2, 0.2),
        dir : glm::Vec3::new(0.2, 0.2, 0.2),
        point : glm::Vec3::new(0.2, 0.2, 0.5),
        spot : glm::Vec3::new(0.7, 0.7, 0.9)},  
    EnvColors{ name : "Horror",
        bg : glm::Vec3::new(0.0, 0.0, 0.0),
        dir : glm::Vec3::new(0.0, 0.0, 0.0),
        point : glm::Vec3::new(0.15, 0.0, 0.0),
        spot : glm::Vec3::new(0.3, 0.3, 0.3)},
    EnvColors{ name : "Lab",
        bg : glm::Vec3::new(0.7, 0.7, 0.7),
        dir : glm::Vec3::new(0.9, 0.9, 0.9),
        point : glm::Vec3::new(0.4, 0.69, 0.2),
        spot : glm::Vec3::new(0.9, 0.9, 0.9)},
    ];

const CUBE_POSITIONS: [glm::Vec3; 10] = [
    glm::Vec3::new(0.0, 0.0, 0.0),
    glm::Vec3::new(2.0, 5.0, -15.0),
    glm::Vec3::new(-1.5, -2.2, -2.5),
    glm::Vec3::new(-3.8, -2.0, -12.3),
    glm::Vec3::new(2.4, -0.4, -3.5),
    glm::Vec3::new(-1.7, 3.0, -7.5),
    glm::Vec3::new(1.3, -2.0, -2.5),
    glm::Vec3::new(1.5, 2.0, -2.5),
    glm::Vec3::new(1.5, 0.2, -1.5),
    glm::Vec3::new(-1.3, 1.0, -1.5),
];

pub fn main_2_6_1() {
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
    let default_shader = Shader::new("./src/_2_lighting/shaders/4_0_default.vert","./src/_2_lighting/shaders/6_0_default.frag");
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

        (vao, vbo)
    };

    
    let (light_vao, light_vbo) = unsafe {
        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &VERTICES[0] as *const f32 as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (vao, vbo)
    };

    // Lighting maps
    let (diffuse_map,specular_map) = unsafe {(
        load_texture("./resources/textures/container_diffuse.png"),
        load_texture("./resources/textures/container_specular.png")
    )};
    
    // --Initial Config - Viewport------------------------------------------------------------------------------------------------- //

    // Camera
    let mut camera : FreeCamera = FreeCamera::new(glm::vec3(-2.,0.,2.), 0., 0., -40., WINDOW_WIDTH, WINDOW_HEIGHT);

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

    // Enable depth testing to put display top most primitives
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Time
    let mut prev_time = glfw.get_time();

    // Current env
    let mut curr_env = 0;
    
    println!("Current Environment : {}", ENVIRONMENTS[curr_env].name);

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
                glfw::WindowEvent::Key(glfw::Key::N, _, glfw::Action::Press, _) => {
                    curr_env = (curr_env + 1) % ENVIRONMENTS.len();
                    println!("New Environment : {}", ENVIRONMENTS[curr_env].name);
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
            gl::ClearColor(ENVIRONMENTS[curr_env].bg.x, ENVIRONMENTS[curr_env].bg.y, ENVIRONMENTS[curr_env].bg.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Drawing the object
            default_shader.use_program();
            gl::BindVertexArray(vao);
            
            // Set transformation matrices
            camera.set_cam_matrix(&default_shader);
            // Set material
            default_shader.set_int(c_str!("material.diffuse"), 0);  // Using texture unit 0 for diffuse map
            default_shader.set_int(c_str!("material.specular"), 1);  // Using texture unit 1 for specular map
            default_shader.set_float(c_str!("material.shininess"),  32.0); 
            // View position for specular highlights based on viewer
            default_shader.set_vec3(c_str!("viewPos"), camera.position);    // View position for specular highlights
            
            // Used by directional light
            default_shader.set_vec3(c_str!("dirLight.ambient"), ENVIRONMENTS[curr_env].dir * 0.2);
            default_shader.set_vec3(c_str!("dirLight.diffuse"),  ENVIRONMENTS[curr_env].dir * 0.5);
            default_shader.set_vec3(c_str!("dirLight.specular"),  ENVIRONMENTS[curr_env].dir * 1.0);
            default_shader.set_vec3_values(c_str!("dirLight.direction"), -0.2, -1.0, -0.3); // direction that the light is pointing to

            // Used by point lights
            let mut i = 0;
            while i < LIGHT_LOCATIONS.len() {    
                gl::Uniform3fv(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].ambient", i)).unwrap().as_ptr()), 1, (ENVIRONMENTS[curr_env].point * 0.2).as_ptr() as *const GLfloat); 
                gl::Uniform3fv(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].diffuse", i)).unwrap().as_ptr()),  1, (ENVIRONMENTS[curr_env].point * 0.5).as_ptr() as *const GLfloat); 
                gl::Uniform3fv(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].specular", i)).unwrap().as_ptr()),  1, (ENVIRONMENTS[curr_env].point * 1.0).as_ptr() as *const GLfloat); 
                gl::Uniform3fv(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].position", i)).unwrap().as_ptr()),1, LIGHT_LOCATIONS[i].as_ptr() as *const GLfloat);   
                
                gl::Uniform1f(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].constant", i)).unwrap().as_ptr()), 1.0);
                gl::Uniform1f(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].linear", i)).unwrap().as_ptr()), 0.09);
                gl::Uniform1f(gl::GetUniformLocation(default_shader.id, CString::new(format!("pointLights[{}].quadratic", i)).unwrap().as_ptr()), 0.032);

                i += 1;
            }

            // Spot Light
            
            default_shader.set_vec3(c_str!("spotLight.ambient"),  ENVIRONMENTS[curr_env].spot * 0.2);
            default_shader.set_vec3(c_str!("spotLight.diffuse"),  ENVIRONMENTS[curr_env].spot * 0.5);
            default_shader.set_vec3(c_str!("spotLight.specular"),  ENVIRONMENTS[curr_env].spot * 1.0);
            default_shader.set_vec3(c_str!("spotLight.position"), camera.position);
            default_shader.set_vec3(c_str!("spotLight.direction"), camera.direction);
            default_shader.set_float(c_str!("spotLight.cutOff"), f32::cos(f32::to_radians(12.5)));
            default_shader.set_float(c_str!("spotLight.outerCutOff"), f32::cos(f32::to_radians(17.5)));


            // Set the maps (textures)
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            let mut i = 0;
            while i < CUBE_POSITIONS.len() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &CUBE_POSITIONS[i]);
                model = glm::rotate(&model, 20. + i as f32, &glm::vec3(1.0, 0.3, 0.5));
                default_shader.set_mat4(c_str!("model"),  model); 
                // Draw
                gl::DrawArrays(gl::TRIANGLES,0, 36);
                i+=1;
            }

            let mut i = 0;
            light_shader.use_program();
            gl::BindVertexArray(light_vao);
            camera.set_cam_matrix(&light_shader);
            while i < LIGHT_LOCATIONS.len() {
                // Drawing the light source only for point source
                let mut light_model = glm::Mat4::identity();
                light_model = glm::translate(&light_model, &LIGHT_LOCATIONS[i]);
                light_model = glm::scale(&light_model, &LIGHT_SCALE);

                light_shader.set_mat4(c_str!("model"),  light_model); 

                light_shader.set_vec3(c_str!("lightColor"), ENVIRONMENTS[curr_env].point * 1.0);    // Materials Ex1

                gl::DrawArrays(gl::TRIANGLES,0, 36);

                i += 1;
            }
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

unsafe fn load_texture(path: &str) -> GLuint {
    let mut texture: GLuint = 0;
    unsafe {
        // Generate Texture
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
        let img = image::open(&Path::new(path)).expect("Failed to load texture").flipv().into_rgba8();

        // Store data into texture
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32,img.width() as i32,img.height() as i32,0,gl::RGBA as u32,gl::UNSIGNED_BYTE,img.as_ptr() as *const u8 as *const c_void);

        // Generate mip maps for the texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

    };
    texture
}