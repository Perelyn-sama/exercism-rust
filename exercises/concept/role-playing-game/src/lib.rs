// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        match self.health {
            0 => {  
                if(self.level >= 10){

                    Some(Player {health: 100, mana: Some(100), level: self.level}) 
                }else {
                    Some(Player {health: 100, mana: None, level: self.level}) 

                }
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        match self.mana{
            None => {
                if(mana_cost < self.health){

                    self.health = self.health - mana_cost;
                }else {
                    self.health = 0;
                }
                0
            }, 
            //    Some(_) => todo!(), 
            Some(u32) => {
                if(u32 < mana_cost){
                    0
                }else{
                    self.mana = Some(u32 - mana_cost);
                    // u32 - mana_cost
                    mana_cost * 2
                }
            }
        }
    }
}

fn main(){
    const MANA_COST: u32 = 30;

    let mut underleveled_player = Player {
        health: 20,
        mana: None,
        level: 6,
    };

    underleveled_player.cast_spell(MANA_COST);
}