use bevy::prelude::*;
use bevy::window::WindowResolution;
use tests_and_ideas::tests::trail::Trail;


/// 主函数
fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Tests and Ideas".to_owned(),
                    resolution: WindowResolution::new(1600.0, 900.0),

                    ..default()
                }),

                ..default()
            }
        )
    );

    app.add_systems(Startup, setup_graphics);

    Trail::setup_trail_test(&mut app);

    app.run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default()
    );
}
