use bevy::gizmos::gizmos::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Component)]
struct Pendulum;

#[derive(Resource, Default)]
struct Angle {
    last_time: f32,
    last_angle: f32,
    velocity: f32,
    last_velocity: f32,
    acceleration: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(Angle::default())
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

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
    commands.spawn((Text2dBundle {
        text: Text::from_section(
            "translation",
            TextStyle {
                font_size: 48.,
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::Y * 200.),
        text_anchor: Anchor::TopCenter,
        ..default()
    },));
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut pendulum: Query<(&mut ExternalForce, &Rotation), With<Pendulum>>,
    mut gizmos: Gizmos,
    mut query_text: Query<&mut Text>,
    mut angle: ResMut<Angle>,
) {
    for (mut external_force, rotation) in &mut pendulum {
        let side = if rotation.as_degrees() >= 0. { 1. } else { -1. };
        let current_angle = side * (180. - rotation.as_degrees().abs());
        angle.acceleration = angle.velocity - angle.last_velocity;
        angle.last_velocity = angle.velocity;
        angle.velocity = current_angle - angle.last_angle;
        angle.last_angle = current_angle;
        let velocity_sign = if angle.velocity >= 0. { 1. } else { -1. };
        let acceleration_sign = if angle.acceleration >= 0. { 1. } else { -1. };

        const P_GAIN: f32 = 1. * 250000.;
        const I_GAIN: f32 = 1. * 1300000.;
        const D_GAIN: f32 = 0. * 2000000.;
        let proportional = P_GAIN * current_angle;
        let integral = I_GAIN * angle.velocity;
        let derivative = D_GAIN * angle.acceleration;
        let sum = 1. * proportional + 1. * integral + 1. * derivative;
        query_text.single_mut().sections =
            vec![TextSection::new((current_angle).to_string(), default())];
        gizmos.line_gradient_2d(
            Vec2 { x: 0., y: 100. },
            Vec2 {
                x: 0.0001 * -sum,
                y: 100.,
            },
            Color::RED,
            Color::GREEN,
        );
        gizmos.line_2d(
            Vec2 { x: 0., y: 120. },
            Vec2 {
                x: 0.0001 * -proportional,
                y: 120.,
            },
            Color::RED,
        );
        gizmos.line_2d(
            Vec2 { x: 0., y: 140. },
            Vec2 {
                x: 0.0001 * -integral,
                y: 140.,
            },
            Color::YELLOW,
        );
        gizmos.line_2d(
            Vec2 { x: 0., y: 160. },
            Vec2 {
                x: 0.0001 * -derivative,
                y: 160.,
            },
            Color::GREEN,
        );

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            external_force.x += rotation.cos() * sum;
            external_force.y += rotation.sin() * sum;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            external_force.x += rotation.cos() * 1000000.;
            external_force.y += rotation.sin() * 1000000.;
            gizmos.line_2d(
                Vec2 {
                    x: rotation.sin() * 50.,
                    y: rotation.cos() * -50.,
                },
                Vec2 {
                    x: rotation.sin() * 50. + rotation.cos() * 50.,
                    y: rotation.cos() * -50. + rotation.sin() * 50.,
                },
                Color::WHITE,
            );
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            external_force.x += rotation.cos() * -1000000.;
            external_force.y += rotation.sin() * -1000000.;
            gizmos.line_2d(
                Vec2 {
                    x: rotation.sin() * 50.,
                    y: rotation.cos() * -50.,
                },
                Vec2 {
                    x: rotation.sin() * 50. - rotation.cos() * 50.,
                    y: rotation.cos() * -50. - rotation.sin() * 50.,
                },
                Color::WHITE,
            );
        }
    }
}
