use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    ecs::system::{Res, ResMut},
    DefaultPlugins,
};
use bevy_window_management::{WindowManagement, WindowManagementPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((WindowManagementPlugin, DefaultPlugins))
        .add_systems(
            Startup,
            |assets: Res<AssetServer>, mut window: ResMut<WindowManagement>| {
                window.window_icon = Some(assets.load("LowerLeft.png"));
                window.taskbar_progress = Some(bevy_window_management::TaskbarProgress {
                    progress: 30,
                    max: 100,
                });
            },
        )
        .add_systems(Update, |mut window: ResMut<WindowManagement>| {
            window.taskbar_progress =
                window
                    .taskbar_progress
                    .as_ref()
                    .map(|p| bevy_window_management::TaskbarProgress {
                        progress: p.progress + 1,
                        max: 100,
                    });
        });
    app.run();
}
