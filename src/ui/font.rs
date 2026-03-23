use bevy::prelude::*;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FontAssets>();

        let assets = app.world().resource::<AssetServer>();
        let default_font: Handle<Font> = assets.load("PressStart2P-Regular.ttf");

        app.insert_resource(FontAssets {
            default: default_font,
        });
    }
}

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub(crate) struct FontAssets {
    pub default: Handle<Font>,
}
