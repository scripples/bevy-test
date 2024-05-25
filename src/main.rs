//! Create a custom material to draw basic lines in 3D

use bevy::{
    prelude::*,
    render::{
        mesh::{PlaneMeshBuilder, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    window::WindowResized,
};
use bevy_obj::ObjPlugin;
use bevy_test::materials::{
    grid1::GridMaterial,
    line_material::{LineMaterial, LineMaterialWhite},
    traits::MaterialTime,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ObjPlugin,
            MaterialPlugin::<LineMaterialWhite>::default(),
            MaterialPlugin::<GridMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, on_resize_system))
        .run();
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotation>>,
    mut mats: ResMut<Assets<LineMaterialWhite>>,
    mut grid_mats: ResMut<Assets<GridMaterial>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(0.01);
    }
    for material_handle in mats.iter_mut() {
        material_handle.1.time = time.elapsed_seconds_wrapped();
    }
    for material_handle in grid_mats.iter_mut() {
        material_handle.1.set_time(time.elapsed_seconds_wrapped());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut line_materials: ResMut<Assets<LineMaterialWhite>>,
    mut grid_materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(PlaneMeshBuilder::new(
            Direction3d::new(Vec3::new(0.0, 0.0, 1.0).normalize()).unwrap(),
            Vec2::new(10., 10.),
        )),
        material: grid_materials.add(GridMaterial::default()),
        transform: Transform::from_xyz(0.0, 0.0, -5.),
        // material: line_materials.add(LineMaterial {
        //     color: Color::WHITE,
        //     time: 0.0,
        // }),
        ..default()
    });

    commands.spawn((
        MaterialMeshBundle {
            mesh: asset_server.load("models/shivel_3d_blend_export.obj"),
            material: line_materials.add(LineMaterialWhite {
                color: Color::WHITE,
                time: 0.0,
            }),
            ..default()
        },
        Rotation,
    ));

    // commands.spawn((
    //     MaterialMeshBundle {
    //         mesh: meshes.add(Sphere { radius: 1. }),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         material: line_materials.add(LineMaterial {
    //             color: Color::WHITE,
    //             time: 0.0,
    //         }),
    //         ..default()
    //     },
    //     Rotation,
    // ));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct Rotation;

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut grid_mats: ResMut<Assets<GridMaterial>>,
) {
    for e in resize_reader.read() {
        println!("READER WIDTH: {:?}", (e.width, e.height));
        for material_handle in grid_mats.iter_mut() {
            material_handle
                .1
                .update_window_size(Vec2::new(e.width, e.height));
        }
    }
}
