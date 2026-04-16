use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AetherEngine {
    player_hp: i32,
    enemy_hp: i32,
    chaos_level: f64,
}

#[wasm_bindgen]
impl AetherEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AetherEngine {
        AetherEngine { player_hp: 100, enemy_hp: 100, chaos_level: 0.0 }
    }

    pub fn update_seismic(&mut self, x: f64, y: f64, z: f64) {
        let mag = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        self.chaos_level = (self.chaos_level * 0.9) + (mag * 0.1);
    }

    pub fn handle_combat(&mut self, correct: bool) -> String {
        if correct {
            let dmg = 15 + (self.chaos_level * 2.5) as i32;
            self.enemy_hp -= dmg;
            format!("WIN|{}|{}", dmg, self.enemy_hp)
        } else {
            self.player_hp -= 10;
            format!("LOSS|10|{}", self.player_hp)
        }
    }

    pub fn get_chaos(&self) -> f64 { self.chaos_level }
}