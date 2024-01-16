use bevy::{prelude::*, window::*};
use bevy_screen_diagnostics::*;
mod arrow;
mod background;
mod note;
mod provatheus;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "PleaseGiveMeEXPAboutBevy".into(), //タイトル
                        resolution: (1280.0, 720.0).into(),       //ウィンドウサイズ
                        position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                        present_mode: PresentMode::AutoVsync,                          //Vsync有効
                        resizable: false, //サイズ変更不可
                        enabled_buttons: bevy::window::EnabledButtons {
                            minimize: false, //最小化無効
                            maximize: false, //最大化無効
                            close: true,     //閉じる有効
                        },
                        visible: false, //非表示
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //異方性レンダリングを無効化する
        )
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0))) //デフォルトの背景色を設定
        .insert_resource(Msaa::Off) //Multi-Sample Anti-Aliasing off
        .add_systems(Update, (provatheus::enable_visible, provatheus::gizmos_xyz)) //Provatheus用の開発用ライブラリ
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        //以上が固定用
        .add_systems(
            Startup,
            (
                setcamera,
                background::background_image,
                arrow::spawn_arrow,
                note::spawn_note,
            ),
        )
        .add_systems(
            Update,
            (
                arrow::arrow_button,
                note::change_note_color,
                note::play_note,
                note::clear_note,
            ),
        )
        .run();
}
#[derive(Component)]
pub struct Arrow {
    line: i32,
}

#[derive(Component)]
pub struct Note {
    line: i32,
}

fn setcamera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
