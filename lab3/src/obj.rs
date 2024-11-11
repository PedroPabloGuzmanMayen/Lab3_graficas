// obj.rs
use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], -v[1], -v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], -n[1], -n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], 1.0-t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }
    
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertexes = Vec::with_capacity(self.indices.len());

        for &i in &self.indices {
            let position = self.vertices[i as usize];
            let normal = if !self.normals.is_empty() {
                self.normals[i as usize]
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };
            let tex_coord = if !self.texcoords.is_empty() {
                self.texcoords[i as usize]
            } else {
                Vec2::new(0.0, 0.0)
            };

            vertexes.push(Vertex::new(position, normal, tex_coord));
        }

        vertexes
    }
}