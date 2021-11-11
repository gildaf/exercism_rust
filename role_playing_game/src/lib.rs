// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

const NEW_PLAYER_HEALTH: u32 = 100;

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else {
            let health = NEW_PLAYER_HEALTH;
            let level = self.level;
            let mana = if level >= 10 { Some(100) } else { None };
            Some(Player {
                health,
                mana,
                ..*self
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let is_cast = match self.mana {
            None => {
                // If the player does not have access to a mana pool,
                // attempting to cast the spell must decrease their
                //  health by the mana cost of the spell.
                // The damage returned must be 0
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                false
            }
            Some(mana) if mana < mana_cost => {
                // If the player has a mana pool but insufficient mana,
                // the method should not affect the pool,
                // but instead return 0
                false
            }
            Some(mana) => {
                // the mana_cost should be deducted from the Player's mana pool
                // and the appropriate amount of damage should be returned.
                self.mana = Some(mana - mana_cost);
                true
            }
        };
        let spell_cost = if is_cast { mana_cost * 2 } else { 0 };
        return spell_cost;
    }
}

#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_revive_not_dead() {
        let dead_player = Player {
            health: 11,
            mana: None,
            level: 2,
        };
        let new_player = dead_player.revive();
        if let Some(_) = new_player {
            assert!(false);
        } else {
            // all good
        }
        // assert!(dead_player.revive() == None);
    }
    #[test]
    fn test_revive_level_2() {
        let dead_player = Player {
            health: 0,
            mana: None,
            level: 2,
        };
        let revived = dead_player.revive().unwrap();

        assert_eq!(revived.health, 100);
        assert_eq!(revived.mana, None);
        assert_eq!(revived.level, 2);
    }

    #[test]
    fn test_revive_level_12() {
        let dead_player = Player {
            health: 0,
            mana: None,
            level: 12,
        };
        let revived = dead_player.revive().unwrap();

        assert_eq!(revived.health, 100);
        assert_eq!(revived.mana, Some(100 as u32));
        assert_eq!(revived.level, 12);
    }

    #[test]
    fn test_cast_spell_no_mana() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: None,
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
        assert_eq!(not_a_wizard_yet.health, 74);
        assert_eq!(not_a_wizard_yet.mana, None);
    }

    #[test]
    fn test_cast_spell_no_mana_large_cost() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: None,
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(250), 0);
        assert_eq!(not_a_wizard_yet.health, 0);
        assert_eq!(not_a_wizard_yet.mana, None);
    }

    #[test]
    fn test_cast_spell_insuffient_mana() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: Some(4),
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
        assert_eq!(not_a_wizard_yet.health, 79);
        assert_eq!(not_a_wizard_yet.mana, Some(4));
    }

    #[test]
    fn test_cast_spell_success() {
        let mut not_a_wizard_yet = Player {
            health: 79,
            mana: Some(23),
            level: 9,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(10), 20);
        assert_eq!(not_a_wizard_yet.health, 79);
        assert_eq!(not_a_wizard_yet.mana, Some(13));
    }
}
