use std::ffi::CString;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use nalgebra_glm as glm;
use gl::{self, types::*};

use crate::shader::Shader;

#[repr(C)] // align struct like C/C++
pub struct Vertex {
    pub position : glm::Vec3,
    pub normal : glm::Vec3,
    pub tex_coords : glm::Vec2,
}

#[derive(Copy, Clone)]
pub enum TextureType {
    Diffuse,
    Specular,
    Emissive
}

impl TextureType {
    /// Function to return CString to update shader
    pub fn get_shader_name(&self, num : GLint) -> CString {
        CString::new(
            format!("material.{}{}",
            match self {
                Self::Diffuse => { "texture_diffuse"},
                Self::Specular => { "texture_specular"},
                Self::Emissive => { "texture_emissive"}
            },
            num)).unwrap()
    }
    /// Functiont to return number to use with get_shader_name
    /// Also increments the count
    pub fn update_count(&self, diffuse_number : &mut i32, specular_number : &mut i32, emissive_number : &mut i32) -> i32 {
        let num;
        match self{
            Self::Diffuse => {
                num = *diffuse_number;
                *diffuse_number += 1;
            },
            Self::Specular => {
                num = *specular_number;
                *specular_number += 1;
            },
            Self::Emissive => {
                num = *emissive_number;
                *emissive_number += 1;
            }
        }
        num
    }
}

// // This can be implemented using enum itself in rust
// enum Texture {
//     Diffuse(GLuint)
// }
// impl Texture {
//     fn do_something(&self) {
//         match self {
//             Texture::Diffuse(x) => { /*do something with x */}
//             _ => {}
//         }
//     }
// }
// But I'm keeping it similar to c++ code 
#[derive(Clone)]
pub struct Texture {
    pub id : GLuint,
    pub tex_type : TextureType,
    pub path : String
}

pub struct Mesh {
    pub vertices : Vec<Vertex>,
    pub indices : Vec<GLuint>,
    pub textures : Vec<Texture>,
    vao : GLuint,
    vbo : GLuint,
    ebo : GLuint
}

impl Mesh {
    pub fn new(v: Vec<Vertex>, i : Vec<GLuint>, t : Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices : v,
            indices : i,
            textures : t,
            vao : 0,
            vbo : 0,
            ebo : 0
        };
        mesh.setup_mesh();
        mesh
    }

    pub fn new_empty () -> Mesh {
        let v = Vec::from([
                Vertex{
                    position : glm::Vec3::new(-0.5,-0.5,0.),
                    normal : glm::Vec3::new(0.,0.,1.),
                    tex_coords : glm::Vec2::zeros()},
                Vertex{
                    position : glm::Vec3::new(0.5,-0.5,0.),
                    normal :  glm::Vec3::new(0.,0.,1.),
                    tex_coords : glm::Vec2::zeros()},
                Vertex{
                    position : glm::Vec3::new(0.,0.5,0.),
                    normal :  glm::Vec3::new(0.,0.,1.),
                    tex_coords : glm::Vec2::zeros()},
                ]);
        let i: Vec<GLuint> = Vec::from([0,1,2]);
        let mut mesh = Mesh {
            vertices : v,
            indices : i,
            textures : Vec::new(),
            vao : 0,
            vbo : 0,
            ebo : 0
        };
        mesh.setup_mesh();
        mesh
    }

    fn setup_mesh(&mut self) {
        //println!("Setting up mesh");
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            // Bind VAO to make it active
            gl::BindVertexArray(self.vao);

            // Bind VBO and store vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr, &self.vertices[0] as *const Vertex as *const c_void, gl::STATIC_DRAW);

            // Bind EBO and store index data
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr, &self.indices[0] as *const GLuint as *const c_void, gl::STATIC_DRAW);

            // Link vertex attributes 
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, mem::offset_of!(Vertex, normal) as *const c_void);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, mem::offset_of!(Vertex, tex_coords) as *const c_void);
            gl::EnableVertexAttribArray(2);

            // Note: we can safely unbind VBO since it is bound to the VAO's vertex attribute from VertedAttribPointer
            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // we CANNOT unbind the EBO while the VAO is active
            gl::BindVertexArray(0);
            // Unbinding VAO/VBO is usually not required, since we will always use BindVertexArray
        }
    }

    pub fn draw(&self,shader : &Shader) {

        // Bind textures and set shader uniforms for all textures
        let mut diffuse_number = 0;
        let mut specular_number = 0;
        let mut emissive_number = 0;
        let mut i = 0;

        // reset uniforms
        unsafe {
            shader.set_int(c_str!("material.texture_diffuse0"), 0);
            shader.set_int(c_str!("material.texture_specular0"), 0);
            shader.set_int(c_str!("material.texture_emissive0"), 0);
        }
        while i < self.textures.len() {
            unsafe{

                // activate the i'th texture unit
                gl::ActiveTexture(gl::TEXTURE1 + i as u32);

                // Get the texture's number and update the respective count
                let num = self.textures[i].tex_type.update_count(&mut diffuse_number, &mut specular_number, &mut emissive_number);
                
                // Update the texture uniform
                shader.set_int( &self.textures[i].tex_type.get_shader_name(num), (i + 1) as i32);

                // Bind the current texture
                gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id);
            }
            i += 1;
        }

        // Draw the mesh
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            vertices : Vec::new(),
            indices : Vec::new(),
            textures : Vec::new(),
            vao : 0,
            vbo : 0,
            ebo : 0
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            println!("Deleting buffers - vao:{}, vbo - {}, ebo - {}", self.vao, self.vbo, self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}