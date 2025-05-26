use std::ffi::CString;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use nalgebra_glm as glm;
use gl::{self, types::*};

use crate::shader::Shader;

/// Struct to store vertex data
#[repr(C)] // align struct like C/C++
pub struct Vertex {
    pub position : glm::Vec3,
    pub normal : glm::Vec3,
    pub tex_coords : glm::Vec2,
}

/// Enum to store all material data
#[derive(Clone)]
pub enum Material {
    Texture{ id : u32, path : String, type_ : MaterialType},
    Property{ value : f32, type_ : MaterialType}
}

// Types for shader textures and properties
#[derive(Clone)]
pub enum MaterialType {
    DiffuseTex,
    SpecularTex,
    EmissiveTex,
    Shininess
}

impl Material {
    /// Function to compare the texture's stored path with other
    /// 
    /// Returns false if used on anything other than Material::Texture
    pub fn is_path_eq(&self, other : &str) -> bool {
        match self {
            Material::Texture { id: _, path, type_ : _ } => path==other,
            _ => false,
        }
    }
}

pub struct Mesh {
    pub vertices : Vec<Vertex>,
    pub indices : Vec<GLuint>,
    pub materials : Vec<Material>,
    vao : GLuint,
    vbo : GLuint,
    ebo : GLuint
}

impl Mesh {
    pub fn new(v: Vec<Vertex>, i : Vec<GLuint>, t : Vec<Material>) -> Mesh {
        let mut mesh = Mesh {
            vertices : v,
            indices : i,
            materials : t,
            vao : 0,
            vbo : 0,
            ebo : 0
        };
        mesh.setup_mesh();
        mesh
    }

    /// Function to create vao, vbo and ebo
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

    /// Function to draw the current mesh using the provided shader program
    /// 
    /// **Assumes that the shader program is activated**
    pub fn draw(&self,shader : &Shader) {
        // Bind textures and set shader uniforms for all textures
        unsafe {
            // Reset uniforms to 0
            // texture unit 0 is not used
            // models can have varying textures e.g., no emissive etc
            // not unsetting this results in other meshes using the textures from a previous call
            shader.set_int(c_str!("material.texture_diffuse0"), 0);
            shader.set_int(c_str!("material.texture_specular0"),0);
            shader.set_int(c_str!("material.texture_emissive0"), 0);
            shader.set_float(c_str!("material.shininess"), 32.);

            // Set shader uniforms - textures and other material properties
            let mut diff_num = 0;
            let mut spec_num = 0;
            let mut emis_num = 0;
            let mut texture_unit = 1;
            for material in &self.materials {
                match material {
                    Material::Texture { id, path: _, type_ } => {

                        // Get the current texture count based on the type, and update the mutable count
                        let (tex_type , tex_num) = match type_ {
                            MaterialType::DiffuseTex => { 
                                let num = diff_num;
                                diff_num += 1;
                                ("texture_diffuse",num)
                            },
                            MaterialType::SpecularTex => {
                                let num = spec_num;
                                spec_num += 1;
                                ("texture_specular",num)
                            },
                            MaterialType::EmissiveTex => {
                                let num = emis_num;
                                emis_num += 1;
                                ("texture_emissive",num)
                            },
                            _ => {("",0)} // This should not happen, maybe add panic!
                        };

                        // Activate the current texture unit
                        gl::ActiveTexture(gl::TEXTURE0 + texture_unit as u32);

                        // Bind the current texture
                        gl::BindTexture(gl::TEXTURE_2D, *id);
                        
                        // Update the texture uniform
                        shader.set_int( &CString::new(format!("material.{}{}",tex_type, tex_num)).unwrap(), texture_unit as i32);

                        // Incrementing texture unit count
                        texture_unit += 1;
                    },
                    Material::Property { value, type_ } => {
                        shader.set_float( &CString::new(format!("material.{}",match type_ {
                                MaterialType::Shininess => "shininess",
                                _ => ""
                            })).unwrap(), *value);
                    },
                }
            }

            // Draw the mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            //println!("Deleting mesh buffers : vao:{}, vbo : {}, ebo : {}", self.vao, self.vbo, self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}