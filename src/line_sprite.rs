use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use geo::coord;

/// Describes the exterior shape of a sprite, for the purpose of rendering and collision detection.
#[derive(Component, Debug)]
pub enum Shape {
    Polygon(Vec<Vec2>),
    LineString(Vec<Vec2>),
}

impl Shape {
    pub fn from_vertices(vertices: impl IntoIterator<Item = Vec2>, close: bool) -> Self {
        let mut vertices = vertices.into_iter().collect::<Vec<_>>();

        if close && vertices.len() > 1 {
            if vertices.last() != vertices.first() {
                vertices.push(vertices[0]);
            }
            Self::Polygon(vertices)
        } else {
            Self::LineString(vertices)
        }
    }

    pub fn as_geometry(&self, transform: &Transform) -> Option<geo::Geometry<f32>> {
        match self {
            Self::Polygon(vertices) => {
                let line_string = geo::LineString::new(vertices_to_coords(vertices, transform));

                // no need to close the line_string, it's already been done in `from_vertices`
                Some(geo::Polygon::new(line_string, vec![]).into())
            }
            Self::LineString(vertices) => {
                Some(geo::LineString::new(vertices_to_coords(vertices, transform)).into())
            }
        }
    }
}

fn vertices_to_coords(vertices: &[Vec2], transform: &Transform) -> Vec<geo::Coord<f32>> {
    vertices
        .iter()
        .map(|v| {
            let v = transform.transform_point(v.extend(0.0));
            coord! {x: v.x, y:v.y}
        })
        .collect()
}

// ================

pub struct LineSpritePlugin;

impl Plugin for LineSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LineMaterial>::default());
    }
}

#[derive(Bundle)]
pub struct LineSpriteBundle {
    shape: Shape,
    material: MaterialMesh2dBundle<LineMaterial>,
}

pub struct LineSpriteBundleBuilder {
    shape: Shape,
    segments: Vec<(Vec2, Vec2)>,
    transform: Transform,
}

impl LineSpriteBundleBuilder {
    /// Create a new line sprite bundle builder with the provided shape.
    pub fn new(shape: Shape) -> Self {
        let segments = match &shape {
            Shape::Polygon(vertices) => line_to_segment(vertices, true),
            Shape::LineString(vertices) => line_to_segment(vertices, false),
        };

        Self {
            shape,
            segments,
            transform: Transform::default(),
        }
    }

    #[allow(dead_code)]
    pub fn add_line_string(mut self, line: impl IntoIterator<Item = Vec2>) -> Self {
        self.segments.extend(line_to_segment(
            &line.into_iter().collect::<Vec<_>>(),
            false,
        ));
        self
    }

    pub fn add_segments(mut self, segments: impl IntoIterator<Item = (Vec2, Vec2)>) -> Self {
        self.segments.extend(segments);
        self
    }

    /// Add a transform to the sprite.
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<LineMaterial>>,
    ) -> LineSpriteBundle {
        let lines = self
            .segments
            .into_iter()
            .map(|(a, b)| (a.extend(0.0), b.extend(0.0)))
            .collect::<Vec<_>>();
        let material_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(LineList { lines })).into(),
            material: materials.add(LineMaterial {
                color: Color::WHITE,
            }),
            transform: self.transform,
            ..default()
        };

        LineSpriteBundle {
            shape: self.shape,
            material: material_bundle,
        }
    }
}

fn line_to_segment(line: &[Vec2], close: bool) -> Vec<(Vec2, Vec2)> {
    let segment_iter = line.windows(2).map(|w| (w[0], w[1]));

    if close && line.len() > 1 {
        segment_iter
            .chain([(line[line.len() - 1], line[0])])
            .collect()
    } else {
        segment_iter.collect()
    }
}

// ================

#[derive(Default, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "0aa4e64b-9a35-4cad-a698-33d02503169e"]
pub struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material2d for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
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
