use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::sprite::Mesh2dHandle;
use vsvg_core::DocumentTrait;

pub struct SvgExportPlugin;

impl Plugin for SvgExportPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SvgExportSettings::default())
            .add_systems(Update, (keyboard_system,))
            .add_systems(Last, (svg_export_system,));
    }
}

#[derive(Resource, Debug, Default)]
pub struct SvgExportSettings {
    pub export_path: String,

    /// Flag to indicate that the export should be run.
    pub run_export: bool,
}

fn keyboard_system(
    mut svg_export_settings: ResMut<SvgExportSettings>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::E) {
        svg_export_settings.run_export = true;
    }
}

fn svg_export_system(
    meshes: Res<Assets<Mesh>>,
    mut svg_export_settings: ResMut<SvgExportSettings>,
    query: Query<(&GlobalTransform, &Mesh2dHandle)>,
) {
    if svg_export_settings.run_export {
        svg_export_settings.run_export = false;

        let mut doc = vsvg_core::Document::default();
        let mut layer = vsvg_core::Layer::new();

        for (transform, Mesh2dHandle(mesh_handle)) in query.iter() {
            let Some(mesh) = meshes.get(mesh_handle) else {
                continue;
            };

            let Some(vertex_attrib) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
                continue;
            };

            let VertexAttributeValues::Float32x3(vertex_data) = &vertex_attrib else {
                continue;
            };

            let affine = transform.affine();
            let vertex_data = vertex_data.chunks(2).map(|vs| {
                let v1: [f32; 2] = affine.transform_point3(Vec3::from(vs[0])).truncate().into();
                let v2: [f32; 2] = affine.transform_point3(Vec3::from(vs[1])).truncate().into();
                kurbo::PathSeg::Line(kurbo::Line::new(
                    (v1[0] as f64, v1[1] as f64),
                    (v2[0] as f64, v2[1] as f64),
                ))
            });

            let path = kurbo::BezPath::from_path_segments(vertex_data);

            layer.paths.push(path.into());
        }

        doc.layers.insert(1, layer);

        // TODO: support wasm32
        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = std::io::BufWriter::new(std::fs::File::create("/tmp/output.svg").unwrap());
            doc.to_svg(file).unwrap();
        }
    }
}
