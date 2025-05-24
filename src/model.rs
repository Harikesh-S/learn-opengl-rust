use std::mem;
use std::path::Path;
use std::ptr;
use std::os::raw::c_void;
use std::str::FromStr;

use tobj;
use gl::{self,types::*};
use nalgebra_glm as glm;

use crate::mesh::Mesh;
use crate::mesh::Texture;
use crate::mesh::TextureType;
use crate::mesh::Vertex;
use crate::shader::Shader;

/// Struct to hold all models for an object
pub struct Model {
    pub models : Vec<tobj::Model>,  // contains indices
    pub materials : Vec<tobj::Material>,
    pub meshes : Vec<Mesh>, // custom mesh obj
    textures_loaded : Vec<Texture>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            
            models : Vec::new(),
            materials : Vec::new(),
            meshes : Vec::new(),
            textures_loaded : Vec::new()
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Model {..Default::default()}
    }

    /// Function to load a 3D model from path using tobj
    pub fn load_model(&mut self, path : &str) {
        println!("Loading model from {}", path);

        let object = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);
        let (models, materials) = object.expect("Failed to load OBJ file");
        let materials = materials.expect("Failed to load MTL file");
        self.models = models;
        //self.materials = materials;

        println!("# of models: {}", self.models.len());
        println!("# of materials: {}", self.materials.len());

        

        for j in 0..self.models.len() {
            let model = &self.models[j];
            let num_vertices = model.mesh.positions.len() / 3;
            let mut vertices = Vec::with_capacity(num_vertices);

            for i in 0..num_vertices { 
                vertices.push(Vertex { 
                    position: glm::vec3(model.mesh.positions[i*3],model.mesh.positions[i*3+1],model.mesh.positions[i*3+2]), 
                    normal: glm::vec3(model.mesh.normals[i*3],model.mesh.normals[i*3+1],model.mesh.normals[i*3+2]), 
                    //tex_coords: glm::vec2(model.mesh.positions[i*2],model.mesh.positions[i*2+1])
                    tex_coords: glm::vec2(model.mesh.texcoords[i*2],model.mesh.texcoords[i*2+1])
                    //tex_coords: glm::vec2(i as f32,i as f32)
                });
                //println!("{:?} {:?}", vertices[i].position, vertices[i].tex_coords);
            }

            let mut indices = model.mesh.indices.clone();
            // textures
            let mut textures: Vec<Texture> = Vec::new();
            if let Some(material_id) = model.mesh.material_id {
                let material = &materials[material_id].clone();
                
                if let Some(path) = &material.diffuse_texture {
                   textures.push(self.load_texture_if_required(path.as_str(), TextureType::Diffuse));
                }
                if let Some(path) = &material.specular_texture {
                  textures.push(self.load_texture_if_required(path.as_str(), TextureType::Specular));
                }

                for (k, v) in &material.unknown_param {
                    if k.to_string() == "map_Ke" {
                           textures.push(self.load_texture_if_required(v.as_str(), TextureType::Emissive));
                    }
                }
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    pub fn draw(&self, shader : &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }

    fn load_texture_if_required(&mut self,path: &str, typeName: TextureType) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }
        println!("Loading texture from path {}", path);

        let texture = Texture {
            id: unsafe { load_texture(path) },
            tex_type: typeName,
            path: path.into()
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
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
        
        println!("Loading image from path {}", path);
        // Loading image from file
        let img = image::open(&Path::new(path)).expect("Failed to load texture").flipv().into_rgba8();

        println!("Storing texture data");
        // Store data into texture
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32,img.width() as i32,img.height() as i32,0,gl::RGBA as u32,gl::UNSIGNED_BYTE,img.as_ptr() as *const u8 as *const c_void);

        println!("Generating mip maps");
        // Generate mip maps for the texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

    };
    texture
}