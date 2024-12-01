mod game_menus;

use bevy::{prelude::*, state::state::FreelyMutableState};

#[derive(Resource)]
pub(crate) struct MenuResource<T> {
    pub(crate) menu_state: T,
    pub(crate) game_start_state: T,
    pub(crate) game_end_state: T,
}

pub struct GameStatePlugin<T>
where T: States + FreelyMutableState + Copy {
    preload: T,
    menu_state: T,
    game_start_state: T,
    game_end_state: T
}

impl <T> GameStatePlugin<T>
where T: States + FreelyMutableState + Copy
{
    #[allow(clippy::new_without_default)]
    pub fn new(preload: T, menu_state: T, game_start_state: T, game_end_state: T) -> Self
    {
        Self { preload, menu_state, game_start_state, game_end_state }
    }
}

impl<T: States + FreelyMutableState + Copy> Plugin for GameStatePlugin<T>
{
    fn build(&self, app: &mut App) {
        let start = MenuResource {
            menu_state: self.menu_state,
            game_start_state: self.game_start_state,
            game_end_state: self.game_end_state
        };

        app
            .insert_resource(start)
            .insert_state(self.preload)
            .add_systems(Startup, setup_menus::<T>);

        app
            .add_systems(OnEnter(self.menu_state), game_menus::setup::<T>)
            .add_systems(Update, game_menus::run::<T>.run_if(in_state(self.menu_state)))
            .add_systems(OnExit(self.menu_state), cleanup::<game_menus::MenuElement>);

        app
            .add_systems(OnEnter(self.game_end_state), game_menus::setup::<T>)
            .add_systems(Update, game_menus::run::<T>.run_if(in_state(self.game_end_state)))
            .add_systems(OnExit(self.game_end_state), cleanup::<game_menus::MenuElement>);
    }
}

pub fn cleanup<T>(
    query: Query<Entity, With<T>>,
    mut commands: Commands
) 
where T: Component
{
    query.iter().for_each(|entity| commands.entity(entity).despawn());
}

#[derive(Resource)]
pub(crate) struct MenuAssets {
    pub(crate) main_menu: Handle<Image>,
    pub(crate) game_over: Handle<Image>
}

fn setup_menus<T>(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<NextState<T>>, states: Res<MenuResource<T>>) 
where T: States + FreelyMutableState + Copy
{
    let assets = MenuAssets {
        main_menu: asset_server.load("main_menu.png"),
        game_over: asset_server.load("game_over.png")
    };
    commands.insert_resource(assets);
    state.set(states.menu_state);
}

#[macro_export]
macro_rules! add_phase {
    (
        $app:expr, $type:ty, $phase:expr,
        start => [ $($start:expr),* ],
        run => [ $($run:expr),* ],
        exit => [ $($exit:expr),* ]
    ) => { 
        $($app.add_systems(
            bevy::prelude::OnEnter::<$type>($phase),
            $start
        );)*

        $($app.add_systems(
            bevy::prelude::OnExit::<$type>($phase),
            $exit
        );)*

        $($app.add_systems(
            bevy::prelude::Update, $run.run_if(in_state($phase))
        );)*
    }
}