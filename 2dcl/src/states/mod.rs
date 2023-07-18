use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
  #[default]
    MetamaskLogin,
    InGame,
}
