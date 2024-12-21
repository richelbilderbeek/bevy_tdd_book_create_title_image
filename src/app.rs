use bevy::prelude::*;

#[derive(Component)]
pub struct Ferris;

#[derive(Component)]
pub struct TddCircle;


pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    //
    // The function 'try_add_plugins' 
    // (https://github.com/bevyengine/bevy/discussions/15802#discussioncomment-10898148)
    // will make this if obsolete and increase code coverage.
    // Thanks mgi388 for pointing this out
    if cfg!(test) {
        // Keyboard input
        app.add_plugins(bevy::input::InputPlugin);

        // Assets
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(TaskPoolPlugin::default());
        app.init_asset::<bevy::image::Image>();

    }
    //app.insert_resource(Msaa::Sample4);
    //app.add_plugins(ShapePlugin); PANICS

    let add_ferris_fn = move |/* no mut? */ commands: Commands,
                              asset_server: Res<AssetServer>| {
        add_ferris(commands, asset_server);
    };
    app.add_systems(Startup, (add_camera, add_ferris_fn, add_tdd_circles));
    app.add_systems(Update, respond_to_keyboard);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(
        Camera2d::default()
    );
}

fn add_ferris(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("ferris.png")),
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        Ferris,
    ));
}

fn index_to_filename(i: i32) -> String {
    assert!(i >= 0 && i <= 2);
    match i {
        0 => String::from("red_circle.png"),
        1 => String::from("green_circle.png"),
        _ => String::from("blue_circle.png"),
    }
}

fn add_tdd_circles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let n_circles = 3;
    let delta_angle = std::f32::consts::TAU / n_circles as f32;
    let radius = 100.0;

    for i in 0..n_circles {
        let angle = i as f32 * delta_angle;
        let x = f32::sin(angle) * radius;
        let y = f32::cos(angle) * radius;
        let filename = index_to_filename(i);
        //assert!(asset_server.load(filename).is_strong());
        commands.spawn((
            Sprite::from_image(asset_server.load(&filename)),
            Transform {
                translation: Vec3::new(x, y, 1.0),
                ..default()
            },
            TddCircle,
        ));
/*
        commands.spawn(
            (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                ..default()
            },
            Fill::color(color),
            Stroke::new(Color::srgb(1.0, 1.0, 1.0), 10.0),
            TddCircle
          )
        );
*/
        /*
        commands.spawn((
            Sprite {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::new(100.0, 100.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                     color,
                    ..default()
                },
                ..default()
            },
            TddCircle,
        ));
        */
    }
}

#[cfg(test)]
fn get_camera_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_camera_rotation(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.rotation.z
}

#[cfg(test)]
fn get_camera_zoom(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<(&OrthographicProjection, &Camera)>();
    let (projection, _) = query.single(app.world());
    projection.scale
}


#[cfg(test)]
fn get_ferris_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Ferris)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_ferris_size(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Ferris)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn count_n_cameras(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Camera>();
    query.iter(app.world()).len()
}

fn respond_to_keyboard(
    mut query: Query<(&mut Transform, &mut OrthographicProjection, &Camera)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut projection, _) = query.single_mut();
    use bevy::input::keyboard::KeyCode;
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.rotate_z(-0.1);
    }
    if input.pressed(KeyCode::KeyE) {
        transform.rotate_z(0.1);
    }
    if input.pressed(KeyCode::KeyW) {
        projection.scale /= 1.1;
    }
    if input.pressed(KeyCode::KeyS) {
        projection.scale *= 1.1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_no_cameras() {
        let mut app = App::new();
        assert_eq!(count_n_cameras(&mut app), 0);
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_create_app_has_a_moving_camera() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_cameras(&mut app), 1);
    }

    #[test]
    fn test_ferris_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_ferris_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_ferris_has_a_default_size() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_ferris_size(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_camera_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_camera_is_not_rotated_at_start() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_is_not_zoomed_in_or_out_at_start() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_zoom(&mut app), 1.0);
    }

    #[test]
    fn test_camera_moves_when_pressed_up() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_right() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_down() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowDown);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }
    #[test]
    fn test_camera_moves_when_pressed_left() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);
        app.update();assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

    }

    #[test]
    fn test_camera_rotates_when_pressed_q() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyQ);
        app.update();

        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_rotates_when_pressed_e() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();

        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_zooms_in_when_pressed_w() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_zoom(&mut app), 1.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();

        assert!(get_camera_zoom(&mut app) < 1.0);
    }
    #[test]
    fn test_camera_zoom_out_when_pressed_s() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_zoom(&mut app), 1.0);

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyS);
        app.update();

        assert!(get_camera_zoom(&mut app) > 1.0);
    }
    #[test]
    fn test_index_to_filename() {
        assert_ne!(index_to_filename(0), index_to_filename(1));
        assert_ne!(index_to_filename(0), index_to_filename(2));
        assert_ne!(index_to_filename(1), index_to_filename(2));
    }

}
