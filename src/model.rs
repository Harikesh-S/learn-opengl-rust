use std::path::Path;
use std::os::raw::c_void;

use tobj;
use gl::{self,types::*};
use nalgebra_glm as glm;

use crate::mesh::Mesh;
use crate::mesh::Material;
use crate::mesh::MaterialType;
use crate::mesh::Vertex;
use crate::shader::Shader;

/// Struct that represents a model with multiple meshes
pub struct Model {
    pub meshes : Vec<Mesh>, // custom mesh obj
    textures_loaded : Vec<Material>,
    directory : String
}

impl Model {
    pub fn new() -> Self {
        Model {
            meshes : Vec::new(),
            textures_loaded : Vec::new(),
            directory : String::new()
        }
    }

    /// Function to load a 3D model from path using tobj
    pub fn load_model(&mut self, path : &str) {
        println!("Loading model from {}", path);
        let p = Path::new(path);
        self.directory = p.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();

        let object = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);
        let (models, materials) = object.expect("Failed to load OBJ file");
        let materials = materials.expect("Failed to load MTL file");
        //self.materials = materials;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", materials.len());

        

        for j in 0..models.len() {
            let model = &models[j];
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

            let indices = model.mesh.indices.clone();
            
            // materials - textures and properties
            let mut textures: Vec<Material> = Vec::new();
            if let Some(material_id) = model.mesh.material_id {
                let material = &materials[material_id].clone();
                
                if let Some(path) = &material.diffuse_texture {
                    textures.push(self.load_texture_if_required(path.as_str(), MaterialType::DiffuseTex));
                }
                if let Some(path) = &material.specular_texture {
                    textures.push(self.load_texture_if_required(path.as_str(), MaterialType::SpecularTex));
                }
                if let Some(shininess) = material.shininess {
                    textures.push(Material::Property { value: (shininess*128./1000.), type_: MaterialType::Shininess })
                }

                for (k, v) in &material.unknown_param {
                    if k.to_string() == "map_Ke" {
                           textures.push(self.load_texture_if_required(v.as_str(), MaterialType::EmissiveTex));
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

    fn load_texture_if_required(&mut self,path: &str, type_name: MaterialType) -> Material {
        {
            let texture = self.textures_loaded.iter().find(|t| t.is_path_eq(path));
            if let Some(texture) = texture {
                return texture.clone();
            }
        }
        
        //println!("Loading texture from path {}", path);

        let texture = Material::Texture {
            id: unsafe { load_texture(path, &self.directory) },
            type_: type_name,
            path: path.into()
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}


unsafe fn load_texture(path: &str, directory: &str) -> GLuint {
    let absolute_path = Path::new(directory).join(path);
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
        
        //println!("Loading image from path {}", absolute_path.to_str().unwrap());
        // Loading image from file
        let img = image::open(&absolute_path).expect("Failed to load texture").flipv().into_rgba8();

        //println!("Storing texture data");
        // Store data into texture
        gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32,img.width() as i32,img.height() as i32,0,gl::RGBA as u32,gl::UNSIGNED_BYTE,img.as_ptr() as *const u8 as *const c_void);

        //println!("Generating mip maps");
        // Generate mip maps for the texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);

    };
    texture
}

impl Drop for Model {
    fn drop(&mut self) {
        for texture in &self.textures_loaded {
            match texture {
                Material::Texture { id, path: _, type_: _ } => {
                    //println!("Deleting texture {} : {}",id, path);
                    unsafe {
                        gl::DeleteTextures(1, id as *const u32);
                    }},
                _ => {},
            }
        }
    }
}