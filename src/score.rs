use bevy::prelude::*;
use bevy::{app::Plugin, ecs::component::Component};

pub struct ScorePlugin;

#[derive(Component, Resource, Debug, Reflect)]
pub struct Score(usize);

impl Score {
    pub fn score(&mut self, by: usize) {
        self.0 += by;
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Score(0));
    }
}
