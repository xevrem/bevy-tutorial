use bevy::prelude::*;

#[derive(Component)]
pub enum Npc {
    Healer,
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}
