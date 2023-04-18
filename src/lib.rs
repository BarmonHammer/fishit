#![allow(dead_code)]

use std::collections::{HashSet, HashMap};

use num_enum::IntoPrimitive;










// static: Mutex<>


struct Player {
    membership_id: u64,
}

struct Team (Player, Player, Player);

struct Game {
    team_a: Team,
    team_b: Team,
    cfg: dyn Config,
}
impl Game{
    fn evaluate(&self) -> bool {
        self.cfg.evaluate(&self.team_a) && self.cfg.evaluate(&self.team_b)
    }
}

trait Config {
    fn evaluate(&self, player: &Team) -> bool;
}
trait Ban: 'static + Sized{
    fn count(&self) -> u32;
    fn hash(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive)]
#[repr(u8)]
pub enum WeaponType {
    AUTORIFLE = 6,
    BOW = 31,
    FUSIONRIFLE = 11,
    GLAIVE = 33,
    GRENADELAUNCHER = 23,
    HANDCANNON = 9,
    LINEARFUSIONRIFLE = 22,
    MACHINEGUN = 8,
    PULSERIFLE = 13,
    ROCKET = 10,
    SCOUTRIFLE = 14,
    SHOTGUN = 7,
    SIDEARM = 17,
    SNIPER = 12,
    SUBMACHINEGUN = 24,
    SWORD = 18,
    TRACERIFLE = 25,
    UNKNOWN = 0,
}

#[derive(Debug, Clone)]
struct WeaponBan {
    wep_hash: u32,
    count: u32
}

#[derive(Debug, Clone)]
struct ArmorBan {
    armor_hash: u32,
    count: u32
}

#[derive(Debug, Clone)]
struct SubclassCompBan {
    subclass_hash: u32,
    count: u32
}

#[derive(Debug, Clone)]
struct StatDistribution {
    mob: u8,
    res: u8,
    rec: u8,
    dis: u8,
    int: u8,
    str: u8,
}

struct BanArray {
    set: HashSet<u32>,
    map: HashMap<u32, u32>,
}
impl From<Vec<Box<dyn Ban>>> for BanArray {

}

#[derive(Debug, Clone)]
struct FaceitConfig {
    banned_weapons: Vec<WeaponBan>,
    banned_armor: Vec<ArmorBan>,
    duplicate_grenades: bool,
    titan_count: u8,
    warlock_count: u8,
    hunter_count: u8,
    stat_distribution: StatDistribution,
    subclass_comp_bans: Vec<SubclassCompBan>,
    banned_subclasses: HashSet<u32>,
}
// impl Config for FaceitConfig {

// }







