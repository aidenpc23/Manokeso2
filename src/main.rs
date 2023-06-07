use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
    sprite::MaterialMesh2dBundle,
    render::mesh::Indices,
};

pub const CLEAR: Color =  Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size: f32 = 50.0;
    
    // Arbitrary points for the quad
    let p1 = [0.0, 0.0, 0.0];
    let p2 = [size, 0.0, 0.0];
    let p3 = [size, size, 0.0];
    let p4 = [0.0, size, 0.0];

    let mesh = create_quad(p1, p2, p3, p4);

    commands.spawn( MaterialMesh2dBundle  {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });
    
    commands.spawn(Camera2dBundle::default());
}

/// Creates a 2D quadrilateral mesh from four specified points.
///
/// This function takes four points in 3D space, each represented as a 3-element array of `f32`, 
/// and uses them to create a 2D quadrilateral mesh.
/// 
/// # Returns
///
/// A `Mesh` object representing a quadrilateral with the specified vertices. The mesh has a `TriangleList` topology, 
/// meaning it's made of two triangles defined by the vertex indices `[0, 1, 2]` and `[0, 2, 3]`.
///
/// # Example
///
/// ```rust
/// let p1 = [0.0, 0.0, 0.0];
/// let p2 = [1.0, 0.0, 0.0];
/// let p3 = [1.0, 1.0, 0.0];
/// let p4 = [0.0, 1.0, 0.0];
/// let quad_mesh = create_quad(p1, p2, p3, p4);
/// ```
fn create_quad(p1: [f32; 3], p2: [f32; 3], p3: [f32; 3], p4: [f32; 3]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![p1, p2, p3, p4]);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
    mesh
}