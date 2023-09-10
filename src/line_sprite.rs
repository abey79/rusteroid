use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::{MeshVertexBufferLayout, PrimitiveTopology};
use bevy::render::render_resource::{
    AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle};

pub struct LineSpritePlugin;

impl Plugin for LineSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LineMaterial>::default());
    }
}

pub struct LineSprintBundleBuilder {
    lines: Vec<(Vec2, Vec2)>,
    transform: Transform,
}

impl LineSprintBundleBuilder {
    pub fn from_vertices(vertices: impl IntoIterator<Item = Vec2>, close: bool) -> Self {
        let vertices: Vec<Vec2> = vertices.into_iter().collect();
        let segment_iter = vertices.windows(2).map(|w| (w[0], w[1]));

        if close {
            Self::from_segment(segment_iter.chain(if vertices.len() > 1 {
                Some((vertices[vertices.len() - 1], vertices[0]))
            } else {
                None
            }))
        } else {
            Self::from_segment(segment_iter)
        }
    }

    pub fn from_segment(segment: impl IntoIterator<Item = (Vec2, Vec2)>) -> Self {
        Self {
            lines: segment.into_iter().collect(),
            transform: Transform::default(),
        }
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<LineMaterial>>,
    ) -> MaterialMesh2dBundle<LineMaterial> {
        let lines = self
            .lines
            .into_iter()
            .map(|(a, b)| (a.extend(0.0), b.extend(0.0)))
            .collect::<Vec<_>>();
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(LineList { lines }).into()).into(),
            material: materials.add(LineMaterial {
                color: Color::WHITE,
            }),
            transform: self.transform,
            ..default()
        }
    }
}

// ================

#[derive(Default, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8a"]
pub struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material2d for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}
