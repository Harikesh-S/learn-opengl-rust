use std::ffi::{CString,CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;

use gl;
use gl::types::*;

// use cgmath::Matrix4;
// use cgmath::prelude::*;

use nalgebra_glm as glm;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    /// Function to create a new shader program from files
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // Opening files using the provided paths
        let mut v_shader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open vertex shader {}.", vertex_path));
        let mut f_shader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open fragment shader {}.", vertex_path));

        // Reading shader data from files
        let mut v_shader_data = String::new();
        let mut f_shader_data = String::new();
        v_shader_file.read_to_string(&mut v_shader_data).expect("Failed to read vertex shader.");
        f_shader_file.read_to_string(&mut f_shader_data).expect("Failed to read fragment shader.");
        let v_shader_code = CString::new(v_shader_data.as_bytes()).unwrap();
        let f_shader_code = CString::new(f_shader_data.as_bytes()).unwrap();

        unsafe {
            // Create shader
            let v_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let f_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            // Set source code for shaders
            gl::ShaderSource(v_shader, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::ShaderSource(f_shader, 1, &f_shader_code.as_ptr(), ptr::null());

            // Compiling shaders
            gl::CompileShader(v_shader);
            gl::CompileShader(f_shader);
            
            // Creating shader program and linking
            let id = gl::CreateProgram();
            gl::AttachShader(id, v_shader);
            gl::AttachShader(id, f_shader);
            gl::LinkProgram(id);

            // Check for compile or linking errors
            shader.check_compile_errors(v_shader, "VERTEX");
            shader.check_compile_errors(f_shader, "FRAGMENT");
            shader.check_compile_errors(id, "PROGRAM");
            
            // Deleting shaders since they are already linked to the program
            gl::DeleteShader(v_shader);
            gl::DeleteShader(f_shader);
            
            shader.id = id;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub unsafe fn delete(&self) {
        unsafe { gl::DeleteProgram(self.id); }
    }

    pub unsafe fn set_int(&self, name: &CStr, int: GLint) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), int);
        }
    }

    pub unsafe fn set_float(&self, name: &CStr, float: GLfloat) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), float);
        }
    }

    pub unsafe fn set_vec2(&self, name: &CStr, vec2: glm::Vec2) {
        unsafe {
            gl::Uniform2fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, vec2.as_ptr() as *const GLfloat);
        }
    }
    
    pub unsafe fn set_vec3(&self, name: &CStr, vec3: glm::Vec3) {
        unsafe {
            gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, vec3.as_ptr() as *const GLfloat);
        }
    }

    pub unsafe fn set_vec3_values(&self, name: &CStr, x: GLfloat, y: GLfloat, z: GLfloat) {
        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
        }
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_slice().as_ptr() as *const GLfloat);
        }
    }

    unsafe fn check_compile_errors(&self, shader: u32, shader_type: &str) {
        let mut success = gl::FALSE as GLint;
        let mut log = Vec::with_capacity(1024);
        unsafe {
            log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
            if shader_type != "PROGRAM" {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
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
                    gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
                    println!(
                        "ERROR: PROGRAM_LINKING_ERROR of type {}\n{:?}\n\
                            -- ------------------------- --",
                        shader_type,
                        std::str::from_utf8_mut(&mut log));
                }
            }
        }
    }
}
