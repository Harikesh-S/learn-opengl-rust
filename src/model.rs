use assimp::{self, import};
use nalgebra_glm as glm;
use gl::{types::*};

use crate::{mesh::{Mesh, Texture, Vertex}, shader::Shader};

pub struct Model {
    meshes : Vec<Mesh>
}

impl Model {
    pub fn new(model_path : &str) -> Model {
        let mut model = Model {
            meshes : Vec::new()
        };
        println!("Loading model");
        model.load_model(model_path);
        model
    }

    pub fn new_test() -> Model {
        let model = Model {
            meshes : Vec::from([
                Mesh::new_empty()
            ])
        };
        model
    }
    fn load_model(&mut self, path : &str) {
        let mut importer= assimp::Importer::new();
        importer.triangulate(true);
        importer.flip_uvs(true);

        let scene = importer.read_file(path);

        match scene {
            Ok( scene) => { self.processNode(scene.root_node(), &scene);}
            Err(e) => { println!("Error while importing model from path {} : {}",path,e)}
        }
    }

    /// Function to recursively load model data from nodes - really should just use tobj :(
    fn processNode(&mut self, node : assimp::Node, scene : &assimp::Scene) {
        println!("Node {:?} meshes {:?}", node.name, node.num_meshes);
        // process all the node's meshes (if any)
        if node.num_meshes > 0 {
            for mesh_index in node.meshes() {
                println!("Getting mesh from index {}", mesh_index);
                let mesh = scene.mesh(*mesh_index as usize).unwrap();
                self.meshes.push(Self::process_mesh(mesh,scene));
            }
        }
        // then do the same for each of its children
        for child in node.child_iter() {
            self.processNode(child, scene);
        }
        
    }

    fn process_mesh(mesh : assimp::Mesh,scene : &assimp::Scene) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<GLuint> = Vec::new();
        let mut textures: Vec<Texture> = Vec::new();

        // Vertices
        // println!("Loading vertices");
        let mut i = 0;
        while i < mesh.num_vertices() {
            let mesh_vertex = mesh.get_vertex(i).unwrap();
            let mesh_index = mesh.get_normal(i).unwrap();
            let vertex = Vertex{
                position : glm::vec3(mesh_vertex.x, mesh_vertex.y, mesh_vertex.z),
                normal : glm::vec3(mesh_index.x, mesh_index.y, mesh_index.z),
                tex_coords : if mesh.has_texture_coords(0) 
                    { glm::vec2(
                        mesh.get_texture_coord(0, i).unwrap().x,
                        mesh.get_texture_coord(0, i).unwrap().y)
                    }  else {glm::Vec2::zeros()}
            };
            vertices.push(vertex);
            i += 1;
        }
        
        // Indices
        // println!("Loading indices");
        // println!("Num faces {}", mesh.num_faces());
        for face in mesh.face_iter() {
            let mut j = 0;
            while j < face.num_indices {
                //let index = unsafe {face.indices.offset(j as isize ).read()};
                let index = face[j as isize]; // using implemented Index<isize>
                indices.push(index);
                j += 1;
            }
        }  

        // TODO process textures
        // nah i'm giving up
        let mesh = Mesh::new(
            vertices,
            indices,
            textures);

        // for vert in &mesh.vertices {
        //     println!("Vertex {:?} {:?} {:?}", vert.position, vert.normal, vert.tex_coords);
        // }
        //  println!("Indices{:?}",mesh.indices);
        mesh
    }

    pub fn draw(&self,shader : &Shader)
    {
        for mesh in  &self.meshes {
            mesh.draw(&shader);
        }
    }  
}