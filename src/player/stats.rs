pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub stamina_regen_rate: f32,
    pub stamina_drain_rate: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            stamina_regen_rate: 25.0,  // единиц в секунду
            stamina_drain_rate: 20.0,  // единиц в секунду при спринте
        }
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, is_sprinting: bool, delta_time: f32) {
        if is_sprinting && self.stamina > 0.0 {
            self.stamina -= self.stamina_drain_rate * delta_time;
            self.stamina = self.stamina.max(0.0);
        } else if !is_sprinting && self.stamina < self.max_stamina {
            self.stamina += self.stamina_regen_rate * delta_time;
            self.stamina = self.stamina.min(self.max_stamina);
        }
    }

    pub fn can_sprint(&self) -> bool {
        self.stamina > 0.0
    }

    // // pub fn take_damage(&mut self, amount: f32) {
    // //     self.health -= amount;
    // //     self.health = self.health.max(0.0);
    // // }

    // pub fn is_dead(&self) -> bool {
    //     self.health <= 0.0
    // }
}