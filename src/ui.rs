use std::borrow::Cow;

use bevy::{input_focus::InputFocus, math::CompassOctant, prelude::*};
use bevy_enhanced_input::prelude::*;
use bevy_ui::auto_directional_navigation::{AutoDirectionalNavigation, AutoDirectionalNavigator};

pub mod font;
pub mod in_game;
pub mod menu;

pub const MENU_BUTTON_SIZE: (f32, f32) = (400.0, 80.0);

pub(crate) const H1_SIZE: f32 = 80.0;
pub(crate) const H2_SIZE: f32 = 60.0;

const SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect::all(9.0),
    center_scale_mode: SliceScaleMode::Stretch,
    sides_scale_mode: SliceScaleMode::Stretch,
    max_corner_scale: 1.0,
};

#[derive(Component)]
#[require(NavInteraction)]
pub struct NavButton;

#[derive(Component, PartialEq, Eq, Default)]
pub enum NavInteraction {
    Select,
    Click,
    #[default]
    None,
}

impl NavInteraction {
    fn is_none(&self) -> bool {
        matches!(self, NavInteraction::None)
    }
}

#[derive(Component)]
#[component(immutable, storage = "SparseSet")]
pub struct RedrawRequested;

// Updates the NavInteraction components of the NavButton entities
pub fn nav_interaction(
    clicks: Single<&ActionEvents, With<Action<ClickUI>>>,
    mut interactions: Query<(Entity, &mut NavInteraction)>,
    input_focus: Res<InputFocus>,
) {
    for (entity, mut interaction) in &mut interactions {
        if input_focus.get() == Some(entity) {
            if clicks.contains(ActionEvents::FIRE) {
                *interaction = NavInteraction::Click;
            } else if !matches!(*interaction, NavInteraction::Select) {
                *interaction = NavInteraction::Select;
            }
        } else if !interaction.is_none() {
            *interaction = NavInteraction::None;
        }
    }
}

// Updates navigation from inputs
pub fn navigate(
    navigate: On<Fire<NavigateUI>>,
    mut auto_directional_navigator: AutoDirectionalNavigator,
) {
    // Convert input to direction, then convert to CompassOctant
    let maybe_direction = Dir2::from_xy(navigate.value.x, navigate.value.y)
        .ok()
        .map(CompassOctant::from);

    maybe_direction.and_then(|dir| auto_directional_navigator.navigate(dir).ok());
}

// Applies style to the button
pub fn button_style(
    mut interaction_query: Query<
        (&NavInteraction, &mut ImageNode),
        (Changed<NavInteraction>, With<NavButton>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        match *interaction {
            NavInteraction::Click => {
                image.color = Color::srgb(0.3, 0.13, 0.9);
            }
            NavInteraction::Select => {
                image.color = Color::WHITE;
            }
            NavInteraction::None => {
                image.color = Color::NONE;
            }
        }
    }
}

// --------- Helper Functions ---------- //

pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        Pickable::IGNORE,
        UI,
        actions!(
            UI[
            (
                Action::<ClickUI>::new(),
                Down::new(0.9),
                bindings![KeyCode::Space, KeyCode::Enter, GamepadButton::South]
            ),
            (
                Action::<NavigateUI>::new(),
                Pulse::new(0.4),
                Bindings::spawn((
                    Cardinal::arrows(),
                    Cardinal::dpad()
                ))
            ),
            (
                Action::<Back>::new(),
                Down::new(0.9),
                bindings![KeyCode::Backspace, KeyCode::KeyB, GamepadButton::East]
            )
        ]),
    )
}

pub fn header(title: &'static str, height: Val, font_size: f32, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            top: Val::ZERO,
            ..default()
        },
        children![(
            Text::new(title),
            TextFont::from_font_size(font_size).with_font(font),
            TextLayout::justify(Justify::Center),
        )],
    )
}

pub fn nav_button(
    text: &'static str,
    width: Val,
    height: Val,
    font: Handle<Font>,
    border_image: Handle<Image>,
    action: impl Component,
) -> impl Bundle {
    (
        AutoDirectionalNavigation::default(),
        NavButton,
        NavInteraction::None,
        action,
        ImageNode {
            image: border_image,
            image_mode: NodeImageMode::Sliced(SLICER.clone()),
            ..default()
        },
        Node {
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(5)),
            ..default()
        },
        children![(
            Text::new(text),
            TextFont::from_font_size(30.0).with_font(font),
            TextColor(Color::WHITE),
        )],
    )
}

#[derive(Component)]
pub struct UI;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct NavigateUI;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ClickUI;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Back;
