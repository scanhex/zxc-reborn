use bevy::{prelude::*, utils::Duration};
use bevy_stl::StlPlugin;
use core::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(StlPlugin)
        .insert_resource(SpinTimer(Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, spin_disc)
        .run();
}

#[derive(Component)]
struct Disc {
    angle: f32,
    speed: f32,
}

#[derive(Resource)]
struct SpinTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: asset_server.load("nevermore.stl"),
            material: materials.add(Color::rgb(1.0, 0.4, 0.9).into()),
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            ..Default::default()
        },
        Disc { angle: 0.0, speed: 0.5 }
    ));
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 0.0, 20.0),
        point_light: PointLight {
            range: 40.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -5.0, 5.0))
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn spin_disc(
    time: Res<Time>,
    mut timer: ResMut<SpinTimer>,
    mut query: Query<(&mut Disc, &mut Transform)>,
) {
    if timer
        .0
        .tick(Duration::from_secs_f32(time.delta_seconds()))
        .just_finished()
    {
        for (mut disc, mut transform) in query.iter_mut() {
            disc.angle += disc.speed * 2.0 * PI / 60.0;
            disc.speed += 0.001;
            println!("{}", disc.speed);
            *transform = Transform::from_rotation(Quat::from_rotation_z(disc.angle));
        }
    }
}
