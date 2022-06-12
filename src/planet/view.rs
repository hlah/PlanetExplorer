use super::graphics::PlanetMaterial;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum ColorMode {
    Normal = 1,
    Inclination = 2,
    Altitude = 3,
}

impl ColorMode {
    fn cycle(&mut self) -> Self {
        *self = match self {
            ColorMode::Normal => ColorMode::Inclination,
            ColorMode::Inclination => ColorMode::Altitude,
            ColorMode::Altitude => ColorMode::Normal,
        };
        self.clone()
    }
}

pub fn update_color_mode(
    keyboard: Res<Input<KeyCode>>,
    mut color_mode: ResMut<ColorMode>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    planet: Query<&Handle<PlanetMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::F6) {
        let planet_material_handler = planet.single();
        let planet_material = materials.get_mut(planet_material_handler.clone()).unwrap();
        planet_material.color_mode = color_mode.cycle() as u32;
    }
}
