// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match (self.health, self.level) {
            (0, 0..=9) => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            (0, 10..) => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health -= std::cmp::min(self.health, mana_cost);
                return 0;
            },
            Some(current_mana) => {
                if current_mana < mana_cost {
                    return  0;
                } else {
                    self.mana = Some(current_mana - mana_cost);
                    return  2 * mana_cost;
                }
            }
        }
    }
}
