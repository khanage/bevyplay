use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::application::AppState;

#[derive(Resource, Debug, AssetCollection)]
pub struct SceneAssets {
    #[asset(path = "ultimate-space-kit/Rock.glb#Scene0")]
    pub asteroids: Handle<Scene>,
    #[asset(path = "ultimate-space-kit/Spaceship.glb#Scene0")]
    pub spaceship: Handle<Scene>,
    #[asset(path = "ultimate-space-kit/Bullets Pickup.glb#Scene0")]
    pub missiles: Handle<Scene>,
    #[asset(path = "8bit-explosion.ogg")]
    pub explosion: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<SceneAssets>(),
        );
    }
}
