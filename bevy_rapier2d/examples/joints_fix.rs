use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    let parent = commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 25.0),
        TransformBundle::from(Transform::from_translation(Vec3::new(150.0,0.0,0.0))),
        Velocity{
            linvel: Vec2 { x: 0.0, y: 0.0 },
            angvel: 0.0,
        },
    )).id();

    let joint_builder = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(150.0, 0.0));
    let mut joint = ImpulseJoint::new(parent, joint_builder);
    
    joint.data.set_motor(
        rapier2d::prelude::JointAxis::AngX,
        135f32.to_radians(),
        0.0,
        10.0,
        3.0);

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 25.0),
        TransformBundle::from(Transform::from_translation(Vec3::new(0.0,0.0,0.0))),
        joint,
    ));
}
