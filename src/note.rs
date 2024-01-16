use bevy::prelude::*;
use crate::Note;

pub fn spawn_note(mut commands: Commands) {
    let note_size = Vec2::new(50., 50.);
    for i in 0..4 {
        commands.spawn((
            Note { line: i },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(0, 0, 0),
                    custom_size: Some(note_size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    -105. + (i as f32 * 70.),
                    390. + (i as f32 * 75.),
                    1.,
                )),
                ..default()
            },
        ));
    }
}

pub fn change_note_color(mut note_color_query: Query<(&mut Sprite, &Note)>) {
    for (mut note_color, note_line) in note_color_query.iter_mut() {
        if note_line.line == 0 {
            note_color.color = Color::rgb_u8(255, 0, 255)
        } else if note_line.line == 1 {
            note_color.color = Color::rgb_u8(0, 255, 255)
        } else if note_line.line == 2 {
            note_color.color = Color::rgb_u8(0, 255, 0)
        } else if note_line.line == 3 {
            note_color.color = Color::rgb_u8(255, 0, 0)
        }
    }
}

pub fn play_note(keyboard: Res<Input<KeyCode>>, mut note_query: Query<&mut Transform, With<Note>>) {
    for mut note in note_query.iter_mut() {
        if keyboard.pressed(KeyCode::Space) {
            note.translation.y -= 1.;
        }
    }
}

pub fn clear_note(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut note_query: Query<(&Transform, Entity, &Note)>,
) {
    for (note_pos, note_entity, note_line) in note_query.iter_mut() {
        let line = (note_pos.translation.y - (-300.)).abs() < 15.;
        let pos = note_pos.translation.y < -350.;
        if (keyboard.just_pressed(KeyCode::D) && note_line.line == 0 && line) || pos {
            commands.entity(note_entity).despawn();
        } else if (keyboard.just_pressed(KeyCode::F) && note_line.line == 1 && line) || pos {
            commands.entity(note_entity).despawn();
        } else if (keyboard.just_pressed(KeyCode::K) && note_line.line == 2 && line) || pos {
            commands.entity(note_entity).despawn();
        } else if (keyboard.just_pressed(KeyCode::L) && note_line.line == 3 && line) || pos {
            commands.entity(note_entity).despawn();
        }
    }
}
