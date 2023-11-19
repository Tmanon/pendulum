use bevy::prelude::*;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Pendulum;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let anchor = commands.spawn(RigidBody::Kinematic).id();
    let object = commands
        .spawn((
            Pendulum,
            RigidBody::Dynamic,
            Collider::cuboid(5., 100.),
            DebugRender::default(),
            ExternalForce::new(Vec2::ZERO).with_persistence(false),
        ))
        .id();
    commands.spawn(
        RevoluteJoint::new(anchor, object).with_local_anchor_2(Vector::Y * 50.0), //.with_angle_limits(-1.0, 1.0),
    );
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut pendulum: Query<(&mut ExternalForce, &Rotation), With<Pendulum>>,
) {
    for (mut external_force, rotation) in &mut pendulum {
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            external_force.x = rotation.cos() * 1000000.;
            external_force.y = rotation.sin() * 1000000.;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            external_force.x = rotation.cos() * -1000000.;
            external_force.y = rotation.sin() * -1000000.;
        }
    }
}
