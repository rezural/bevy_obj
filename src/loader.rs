use anyhow::Result;
use bevy::prelude::Vec3;
use bevy_asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy_render::{
    mesh::{Indices, Mesh, VertexAttributeValues},
    pipeline::PrimitiveTopology,
};
use bevy_utils::BoxedFuture;
use obj::Vertex;
use thiserror::Error;

#[derive(Default)]
pub struct ObjLoader;

impl AssetLoader for ObjLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy_asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move { Ok(load_obj(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["obj"];
        EXTENSIONS
    }
}

#[derive(Error, Debug)]
pub enum ObjError {
    #[error("Invalid OBJ file.")]
    Gltf(#[from] obj::ObjError),
}
async fn load_obj<'a, 'b>(

    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), ObjError> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    load_obj_pnt(obj::load_obj(bytes)?, &mut mesh);
    load_context.set_default_asset(LoadedAsset::new(mesh));
    Ok(())
}

fn load_obj_pnt(obj: obj::Obj<obj::Position, u32>, mesh: &mut Mesh) {
    let positions =
        VertexAttributeValues::Float3(obj.vertices.iter().map(|v| v.position).collect());
    let normals =VertexAttributeValues::Float3( normals_for_positions(&obj));
    let uvs = uvs_for_positions(&obj);

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    set_mesh_indices(mesh, obj);
}

fn uvs_for_positions(positions: &obj::Obj<obj::Position, u32>) -> Vec<[f32; 3]> {

    let uvs = positions
        .vertices
        .iter()
        .map(|_| [0.; 3])
        .collect();

    uvs
}

fn normals_for_positions(positions: &obj::Obj<obj::Position, u32>) -> Vec<[f32; 3]> {
    let vertices = positions.vertices.clone();
    let indexes = positions.indices.clone();
    let mut normals: Vec<Vec3> = Vec::new();
    let zero = Vec3::new(0., 0., 0.);
    for _ in &vertices {
        normals.push(zero);
    }
    for indexes in indexes.windows(3) {
        let v0 = vertices.get(indexes[0] as usize).unwrap();
        let v1 = vertices.get(indexes[1] as usize).unwrap();
        let v2 = vertices.get(indexes[2] as usize).unwrap();
        let v0 = bevy_vec3_from_position(v0);
        let v1 = bevy_vec3_from_position(v1);
        let v2 = bevy_vec3_from_position(v2);
        //     let normal = (v0 - v1).cross(&(v2 - v1));
        let normal: Vec3 = (v0 - v1).cross(v2 - v1);
        // let normal = Vec3::new(-normal.x, -normal.y, -normal.z);
        let normal = normal.normalize();
        *normals.get_mut(indexes[0] as usize).unwrap() += normal;
    }

    normals
        .iter()
        .map(|normal| {
            let mut slice = [0.; 3];
            normal.write_to_slice_unaligned(&mut slice);
            slice
        })
        .collect()
}

#[inline]
fn bevy_vec3_from_position(position: &obj::Position) -> Vec3 {
    Vec3::from(position.position)
    // Vec3::new(position.position[0], position.position[1], position.position[2])
}

fn set_mesh_indices<T>(mesh: &mut Mesh, obj: obj::Obj<T, u32>) {
    mesh.set_indices(Some(Indices::U32(
        obj.indices.iter().map(|i| *i as u32).collect(),
    )));
}
    // fn normals_from_positions_mesh(positions: VertexAttributeValues) -> VertexAttributeValues {

        // match positions {
        //     VertexAttributeValues::Float3(vec) => {
        //         let v1 = vec[0];
        //         let v2 = vec[1];
        //         let v3 = vec[2];

        //     }
        //     _ => panic!("unimplemented")
        // }
        // assert_eq!(normal_directions.len(), self.vertices.len());

        // for tri_verts in self.triangles.iter() {
        //     let v0 = &self.vertices[tri_verts[0]];
        //     let v1 = &self.vertices[tri_verts[1]];
        //     let v2 = &self.vertices[tri_verts[2]];
        //     let normal = (v0 - v1).cross(&(v2 - v1));

        //     normal_directions[tri_verts[0]] += normal;
        //     normal_directions[tri_verts[1]] += normal;
        //     normal_directions[tri_verts[2]] += normal;
        // }
    // }
