use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::{iter::once, time::Instant};

const DAY: &str = "21";

struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Shop {
    fn make_shop() -> Self {
        Self {
            weapons: vec![
                Item {
                    cost: 8,
                    damage: 4,
                    armor: 0,
                },
                Item {
                    cost: 10,
                    damage: 5,
                    armor: 0,
                },
                Item {
                    cost: 25,
                    damage: 6,
                    armor: 0,
                },
                Item {
                    cost: 40,
                    damage: 7,
                    armor: 0,
                },
                Item {
                    cost: 74,
                    damage: 8,
                    armor: 0,
                },
            ],
            armor: vec![
                Item {
                    cost: 13,
                    damage: 0,
                    armor: 1,
                },
                Item {
                    cost: 31,
                    damage: 0,
                    armor: 2,
                },
                Item {
                    cost: 53,
                    damage: 0,
                    armor: 3,
                },
                Item {
                    cost: 75,
                    damage: 0,
                    armor: 4,
                },
                Item {
                    cost: 102,
                    damage: 0,
                    armor: 5,
                },
            ],
            rings: vec![
                Item {
                    cost: 25,
                    damage: 1,
                    armor: 0,
                },
                Item {
                    cost: 50,
                    damage: 2,
                    armor: 0,
                },
                Item {
                    cost: 100,
                    damage: 3,
                    armor: 0,
                },
                Item {
                    cost: 20,
                    damage: 0,
                    armor: 1,
                },
                Item {
                    cost: 40,
                    damage: 0,
                    armor: 2,
                },
                Item {
                    cost: 80,
                    damage: 0,
                    armor: 3,
                },
            ],
        }
    }
}

struct Item {
    cost: u16,
    damage: u16,
    armor: u16,
}

struct Loadout<'a> {
    weapon: &'a Item,
    armor: Option<&'a Item>,
    ring_1: Option<&'a Item>,
    ring_2: Option<&'a Item>,
}

impl Loadout<'_> {
    fn cost(&self) -> u16 {
        self.weapon.cost
            + self.armor.map(|a| a.cost).unwrap_or_default()
            + self.ring_1.map(|r1| r1.cost).unwrap_or_default()
            + self.ring_2.map(|r2| r2.cost).unwrap_or_default()
    }
}

struct Stats {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

impl Stats {
    fn make_player(loadout: &Loadout) -> Self {
        let damage = loadout.weapon.damage
            + loadout.ring_1.map(|r1| r1.damage).unwrap_or_default()
            + loadout.ring_2.map(|r2| r2.damage).unwrap_or_default();
        let armor = loadout.armor.map(|a| a.armor).unwrap_or_default()
            + loadout.ring_1.map(|r1| r1.armor).unwrap_or_default()
            + loadout.ring_2.map(|r2| r2.armor).unwrap_or_default();

        Self {
            hit_points: 100,
            damage,
            armor,
        }
    }
}

fn damage_dealt(attacker: &Stats, defender: &Stats) -> u16 {
    attacker.damage.saturating_sub(defender.armor).max(1)
}

fn player_wins(player: &Stats, boss: &Stats) -> bool {
    let player_turns_to_0 = player.hit_points.div_ceil(damage_dealt(boss, player));
    let boss_turns_to_0 = boss.hit_points.div_ceil(damage_dealt(player, boss));
    player_turns_to_0 >= boss_turns_to_0
}

fn all_loadouts(shop: &Shop) -> impl Iterator<Item = Loadout<'_>> {
    let armor_choices = shop.armor.iter().map(Some).chain(once(None));
    let ring_choices = (0..=2).flat_map(|n| shop.rings.iter().combinations(n));
    // Oh yeah that's an iterator chain
    shop.weapons
        .iter()
        .cartesian_product(armor_choices)
        .cartesian_product(ring_choices)
        .map(|((weapon, armor), rings)| Loadout {
            weapon,
            armor,
            ring_1: rings.first().copied(),
            ring_2: rings.get(1).copied(),
        })
}

fn part1(shop: &Shop, boss_stats: &Stats) -> Option<u16> {
    all_loadouts(shop)
        .filter(|l| player_wins(&Stats::make_player(l), boss_stats))
        .map(|l| l.cost())
        .min()
}

fn part2(shop: &Shop, boss_stats: &Stats) -> Option<u16> {
    all_loadouts(shop)
        .filter(|l| !player_wins(&Stats::make_player(l), boss_stats))
        .map(|l| l.cost())
        .max()
}

fn main() -> Result<()> {
    start_day(DAY);

    let shop = Shop::make_shop();
    let boss_stats = Stats {
        hit_points: 104,
        damage: 8,
        armor: 1,
    };

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&shop, &boss_stats)
        .ok_or_else(|| anyhow!("did not find a loadout that beats boss"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&shop, &boss_stats).ok_or_else(|| anyhow!("all loadouts beat boss"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_battle() {
        let player = Stats {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Stats {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert!(player_wins(&player, &boss));
    }
}
