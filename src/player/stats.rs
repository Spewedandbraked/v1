pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub stamina_regen_rate: f32,
    pub stamina_drain_rate: f32,
    pub is_dead: bool,
    pub respawn_timer: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            stamina_regen_rate: 25.0,
            stamina_drain_rate: 20.0,
            is_dead: false,
            respawn_timer: 0.0,
        }
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, is_sprinting: bool, delta_time: f32) {
        if self.is_dead { return; }

        if is_sprinting && self.stamina > 0.0 {
            self.stamina -= self.stamina_drain_rate * delta_time;
            self.stamina = self.stamina.max(0.0);
        } else if !is_sprinting && self.stamina < self.max_stamina {
            self.stamina += self.stamina_regen_rate * delta_time;
            self.stamina = self.stamina.min(self.max_stamina);
        }
    }

    pub fn can_sprint(&self) -> bool {
        !self.is_dead && self.stamina > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        if self.is_dead { return; }
        self.health -= amount;
        if self.health <= 0.0 {
            self.health = 0.0;
            self.is_dead = true;
            self.respawn_timer = 3.0; // 3 секунды до респавна
        }
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn update_respawn(&mut self, delta_time: f32) {
        if self.is_dead {
            self.respawn_timer -= delta_time;
            if self.respawn_timer <= 0.0 {
                self.respawn();
            }
        }
    }

    pub fn respawn(&mut self) {
        self.health = self.max_health;
        self.stamina = self.max_stamina;
        self.is_dead = false;
        self.respawn_timer = 0.0;
    }
}