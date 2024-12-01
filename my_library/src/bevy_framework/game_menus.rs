use super::{MenuAssets, MenuResource};
use bevy::{app::AppExit, prelude::*, state::state::FreelyMutableState};

#[derive(Component)]
pub(crate) struct MenuElement;

pub(crate) fn setup<T>(
    state: Res<State<T>>,
    mut commands: Commands<'_, '_>,
    menu_assets: Res<MenuAssets>,
    menu_resource: Res<MenuResource<T>>,
) where
    T: States,
{
    let current_state = state.get();
    let menu_graphic = match current_state {
        current_state if menu_resource.menu_state == *current_state => 
            menu_assets.main_menu.clone(),
        current_state if menu_resource.game_end_state == *current_state =>
            menu_assets.game_over.clone(),
        _ => panic!("Unknown menu state"),
    };

    commands
        .spawn(Camera2d)
        .insert(MenuElement);

    commands
        .spawn((
            Sprite::from_image(menu_graphic),
            Transform::from_xyz(0.0, 0.0, 1.0)
        ))
        .insert(MenuElement);
}

pub(crate) fn run<T>(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    current_state: Res<State<T>>,
    mut state: ResMut<NextState<T>>,
    menu_state: Res<MenuResource<T>>,
) where
    T: States + FreelyMutableState,
{
    let current_state = current_state.get().clone();
    if current_state == menu_state.menu_state {
        if keyboard.just_pressed(KeyCode::KeyP) {
            state.set(menu_state.game_start_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    } else if current_state == menu_state.game_end_state {
        if keyboard.just_pressed(KeyCode::KeyM) {
            state.set(menu_state.menu_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}