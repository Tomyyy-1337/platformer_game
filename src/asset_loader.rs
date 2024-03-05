use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct FontAssets {
    pub menu_font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            .add_systems(PreStartup, (
                load_font_assets,
        ));
    }
}

fn load_font_assets(
    mut assets: ResMut<FontAssets>,
    asset_server: Res<AssetServer>
) {
    *assets = FontAssets {
        menu_font: asset_server.load("fonts/LonelyNight.otf")
    };
}

