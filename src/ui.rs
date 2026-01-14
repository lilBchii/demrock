use bevy::prelude::*;

use crate::common::{AppState, MultiplayerMode};

#[derive(Component)]
pub struct OnStartMenuScreen;

#[derive(Component)]
pub struct MultiplayerFlag;

// pub fn start_game(keyboard_input: Res<SystemInput<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
//     if keyboard_input.any_just_pressed([KeyCode::Enter, KeyCode::Space]) {
//         app_state.set(AppState::Playing);
//     }
// }

// pub fn setup_start_menu(mut commands: Commands,
//                         asset_server: Res<AssetServer>,)
// {
//     commands
//         .spawn((
//             NodeBundle{
//                 style: Style {
//                     size: Size::new(Val::Percent(100.), Val::Percent(100.)),
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             OnStartMenuScreen,
//         ))
//         .with_children(|parent| {
//             parent.spawn(
//                 ImageBundle{
//                     image: asset_server.load("menu.png").into(),
//                     ..default()
//                 }
//             );
//             parent.spawn((
//                 ImageBundle{
//                     image: asset_server.load("car.png").into(),
//                     transform: Transform::from_rotation(Quat::from_xyzw(0.,0.,0.707,0.707)),
//                     style: Style {
//                         size: Size::new(Val::Px(50.), Val::Px(50.)),
//                         position_type: PositionType::Absolute,
//                         position: UiRect {
//                             top: Val::Px(345.),
//                             left: Val::Px(450.),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 MultiplayerFlag,
//             ));
//         });
// }

// pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>,
//                                     mut commands: Commands)
// {
//     for entity in &to_despawn {
//         commands.entity(entity).despawn_recursive();
//     }
// }

// pub fn switch_multiplayer_mode(kb: Res<Input<KeyCode>>,
//                                mut multiplayer_mode: ResMut<MultiplayerMode>,
//                                mut flag: Query<&mut Style, With<MultiplayerFlag>>)
// {
//     if kb.any_just_pressed([KeyCode::Up, KeyCode::Down]) {
//         for mut style in &mut flag {
//             match *multiplayer_mode {
//                 MultiplayerMode::SinglePlayer => {
//                     style.position.top = Val::Px(420.);
//                     *multiplayer_mode = MultiplayerMode::TwoPlayers;
//                 },
//                 MultiplayerMode::TwoPlayers => {
//                     style.position.top = Val::Px(345.);
//                     *multiplayer_mode = MultiplayerMode::SinglePlayer;
//                 },
//             }
//         }
//     }
// }
