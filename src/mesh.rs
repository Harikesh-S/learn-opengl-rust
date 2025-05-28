use std::collections::HashMap;
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
#[derive(Clone)]
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

            // Texture counts - to avoid branching in the shader
            shader.set_int(c_str!("material.use_texture_diff"), (diff_num>0) as i32);
            shader.set_int(c_str!("material.use_texture_spec"), (spec_num>0) as i32);
            shader.set_int(c_str!("material.use_texture_emis"), (emis_num>0) as i32);

            // Fallback color for diffuse and specular lighting
            if diff_num == 0 {
                shader.set_vec4_values(c_str!("material.fallback_color"), 1., 1., 1., 1.);
            }

            // Draw the mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }

    /// Subdivides the mesh, used in 4_4 to reuse the same model
    pub fn subdivide(&mut self, n : u32) {
        // Vecs to store intermediate results
        let mut final_vertices: Vec<Vertex> = self.vertices.clone();
        let mut final_indices: Vec<GLuint> = self.indices.clone();

        println!("Mesh before subdivision - Vertices {} , Triangles {}", self.vertices.len(), self.indices.len()/3);

        for i in 0..n {
            println!("Running Subdivision level {}", i+1);

            // Vecs to store current result
            let mut new_vertices: Vec<Vertex> = Vec::with_capacity(final_vertices.len()*2);
            let mut new_indices: Vec<GLuint> = Vec::with_capacity(final_indices.len()*2);
            // Hashmap to avoid duplicating vertices
            let mut new_vertex_hash: HashMap<String, GLuint> = HashMap::new();

            // For each triangle
            for j in 0..(final_indices.len() / 3) {
                // Get the vertices
                let vert1 = &final_vertices[final_indices[j*3] as usize];
                let vert2 = &final_vertices[final_indices[j*3+1] as usize];
                let vert3 = &final_vertices[final_indices[j*3+2] as usize];

                //println!("Face {} - {} {} {}", j, vert1, vert2, vert3);

                // Add existing vertices to new vertices and get the index
                let new_ver1 = Self::get_vertex(&mut new_vertices,&mut new_vertex_hash, vert1);
                let new_ver2 = Self::get_vertex(&mut new_vertices,&mut new_vertex_hash, vert2);
                let new_ver3 = Self::get_vertex(&mut new_vertices,&mut new_vertex_hash, vert3);
                // Add mid point vertices to new vertices and get the index
                let new_vera = Self::get_center_vertex(&mut new_vertices,&mut new_vertex_hash, vert1, vert2);
                let new_verb = Self::get_center_vertex(&mut new_vertices,&mut new_vertex_hash, vert2, vert3);
                let new_verc = Self::get_center_vertex(&mut new_vertices,&mut new_vertex_hash, vert3, vert1);

                //println!("Face {} - {} {} {}", j, new_ver1, new_ver2, new_ver3);

                // Add new triangles
                let triangles = [
                    new_ver1, new_vera, new_verc,
                    new_ver2, new_verb, new_vera,
                    new_ver3, new_verc, new_verb,
                    new_vera, new_verb, new_verc,
                ];
                for point in triangles {
                    new_indices.push(point as GLuint)
                }
            }

            // Overwrite intermediate results
            final_vertices = new_vertices;
            final_indices = new_indices;

        }
        
        // Delete old buffers - probably better to subdivide before creating the buffers
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
        
        self.vertices = final_vertices;
        self.indices = final_indices;
        //self.materials = t;
        self.vao = 0;
        self.vbo = 0;
        self.ebo = 0;

        println!("Mesh before subdivision - Vertices {} , Triangles {}", self.vertices.len(), self.indices.len()/3);

        self.setup_mesh();
    }

    fn get_center_vertex(vertices : &mut Vec<Vertex>, vertex_hash : &mut HashMap<String, GLuint>, v1 : &Vertex, v2 : &Vertex) -> GLuint {
        let (p1,p2) = (&v1.position, &v2.position);
        let (n1, n2) = (&v1.normal, &v2.normal);
        let (t1, t2) = (&v1.tex_coords, &v2.tex_coords);
        let center_vertex = Vertex{
            position : glm::vec3((p1.x+p2.x)/2.,(p1.y+p2.y)/2.,(p1.z+p2.z)/2.),
            normal : glm::vec3((n1.x+n2.x)/2.,(n1.y+n2.y)/2.,(n1.z+n2.z)/2.),
            tex_coords : glm::vec2((t1.x+t2.x)/2.,(t1.y+t2.y)/2.)
        };

        Self::get_vertex(vertices,vertex_hash, &center_vertex)
    }

    fn get_vertex(vertices : &mut Vec<Vertex>, vertex_hash : &mut HashMap<String, GLuint>, new_vertex : &Vertex) -> GLuint {
        let hash = Self::get_vertex_hash(&new_vertex);
        match vertex_hash.get(&hash) {
            Some(position) => {
                *position
            },
            None => {
                // adding vertex and return new vert
                vertices.push(new_vertex.clone());
                let position = (vertices.len()-1) as GLuint;
                vertex_hash.insert(hash, position);
                position
            },
        }

    }

    fn get_vertex_hash(vertex : &Vertex) -> String { // probably better to use a struct for the hash key https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
        format!("{:?}", vertex.position)
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