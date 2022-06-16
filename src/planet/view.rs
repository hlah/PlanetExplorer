use super::{graphics::PlanetMaterial, Planet, PlanetMaterials};
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColorMode {
    Real = 0,
    Normal = 1,
    Inclination = 2,
    Altitude = 3,
}

impl ColorMode {
    pub fn is_custom(&self) -> bool {
        if let Self::Real = *self {
            false
        } else {
            true
        }
    }
}

impl ColorMode {
    fn cycle(&mut self) -> Self {
        *self = match self {
            ColorMode::Real => ColorMode::Normal,
            ColorMode::Normal => ColorMode::Inclination,
            ColorMode::Inclination => ColorMode::Altitude,
            ColorMode::Altitude => ColorMode::Real,
        };
        self.clone()
    }
}

pub fn cycle_color_mode(keyboard: Res<Input<KeyCode>>, mut color_mode: ResMut<ColorMode>) {
    if keyboard.just_pressed(KeyCode::F6) {
        color_mode.cycle();
    }
}

pub fn update_color_mode(
    mut commands: Commands,
    color_mode: Res<ColorMode>,
    mut planet_materials: ResMut<Assets<PlanetMaterial>>,
    planets: Query<(Entity, &PlanetMaterials), With<Planet>>,
) {
    if color_mode.is_changed() {
        if color_mode.is_custom() {
            for (entity, materials) in planets.iter() {
                commands
                    .entity(entity)
                    .remove::<Handle<StandardMaterial>>()
                    .insert(materials.custom.clone());
                let planet_material = planet_materials.get_mut(materials.custom.clone()).unwrap();
                planet_material.color_mode = color_mode.clone() as u32;
            }
        } else {
            for (entity, materials) in planets.iter() {
                commands
                    .entity(entity)
                    .remove::<Handle<PlanetMaterial>>()
                    .insert(materials.standard.clone());
            }
        }
    }
}
