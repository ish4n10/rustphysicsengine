use bevy::{math::VectorSpace, prelude::*};
use bevy_xpbd::*;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(2.0).mesh().ico(10).unwrap());
    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });
    commands
        .spawn(PbrBundle {
            mesh: sphere_mesh.clone(),
            material: white.clone(),
            ..Default::default()
        })
        .insert(ParticleBundle::new_with_pos_and_vel(
            Vec2::new(-2., 0.), 
            Vec2::new(2., 0.),
        ));
    
        commands
        .spawn(PbrBundle {
            mesh: sphere_mesh.clone(),
            material: white.clone(),
            ..Default::default()
        })
        .insert(ParticleBundle::new_with_pos_and_vel(
            Vec2::new(2., 0.), 
            Vec2::new(-2., 0.),
        ));


    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
            ..Default::default()
        },
        OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        },
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(BasePlugin::default())
        .run();
}
