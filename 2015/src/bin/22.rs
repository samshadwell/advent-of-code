use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use std::{collections::VecDeque, time::Instant};

const DAY: &str = "22";

#[derive(Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const ALL_SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

impl Spell {
    fn can_cast(&self, player: &Stats, boss: &Stats) -> bool {
        player.mana >= self.cost() && !self.has_active_effect(player, boss)
    }

    const fn cost(&self) -> u16 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn has_active_effect(&self, player: &Stats, boss: &Stats) -> bool {
        match self {
            Self::MagicMissile | Self::Drain => false,
            Self::Shield => player.effects.iter().any(|e| match e {
                Effect::Shield(_) => true,
                Effect::Poison(_) | Effect::Recharge(_) => false,
            }),
            Self::Poison => boss.effects.iter().any(|e| match e {
                Effect::Poison(_) => true,
                Effect::Recharge(_) | Effect::Shield(_) => false,
            }),
            Self::Recharge => player.effects.iter().any(|e| match e {
                Effect::Recharge(_) => true,
                Effect::Poison(_) | Effect::Shield(_) => false,
            }),
        }
    }

    fn cast(&self, player: &mut Stats, boss: &mut Stats) -> Result<()> {
        if !self.can_cast(player, boss) {
            return Err(anyhow!("cannot cast spell {self:?}"));
        }
        player.mana -= self.cost();
        match self {
            Self::MagicMissile => {
                boss.hit_points = boss.hit_points.saturating_sub(4);
            }
            Self::Drain => {
                boss.hit_points = boss.hit_points.saturating_sub(2);
                player.hit_points += 2;
            }
            Self::Shield => {
                player.armor += 7;
                player.effects.push(Effect::Shield(6));
            }
            Self::Poison => {
                boss.effects.push(Effect::Poison(6));
            }
            Self::Recharge => {
                player.effects.push(Effect::Recharge(5));
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Effect {
    Poison(usize),
    Shield(usize),
    Recharge(usize),
}

impl Effect {
    const fn apply(self, target: &mut Stats) -> Option<Self> {
        match self {
            Self::Poison(n) => {
                target.hit_points = target.hit_points.saturating_sub(3);
                if n > 1 {
                    Some(Self::Poison(n - 1))
                } else {
                    None
                }
            }
            Self::Shield(n) => {
                if n > 1 {
                    Some(Self::Shield(n - 1))
                } else {
                    target.armor -= 7;
                    None
                }
            }
            Self::Recharge(n) => {
                target.mana += 101;
                if n > 1 {
                    Some(Self::Recharge(n - 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Default)]
struct Stats {
    hit_points: u16,
    mana: u16,
    damage: u16,
    armor: u16,
    effects: Vec<Effect>,
}

impl Stats {
    fn apply_effects(&mut self) {
        let existing = std::mem::take(&mut self.effects);
        self.effects = existing.into_iter().filter_map(|e| e.apply(self)).collect();
    }
}

enum Mode {
    Normal,
    Hard,
}

fn play_round(spell: &Spell, player: &Stats, boss: &Stats, mode: &Mode) -> Result<(Stats, Stats)> {
    let mut next_player = player.to_owned();
    let mut next_boss = boss.to_owned();

    match mode {
        Mode::Hard => {
            next_player.hit_points -= 1;
            if next_player.hit_points == 0 {
                return Ok((next_player, next_boss));
            }
        }
        Mode::Normal => {}
    }

    // Player turn
    next_player.apply_effects();
    next_boss.apply_effects();
    spell.cast(&mut next_player, &mut next_boss)?;

    // Boss turn
    next_player.apply_effects();
    next_boss.apply_effects();
    next_player.hit_points = next_player
        .hit_points
        .saturating_sub(1.max(next_boss.damage.saturating_sub(next_player.armor)));

    Ok((next_player, next_boss))
}

fn best_solution(player: &Stats, boss: &Stats, mode: &Mode) -> Option<u16> {
    struct GameState {
        mana_used: u16,
        player: Stats,
        boss: Stats,
    }

    let mut queue = VecDeque::new();
    queue.push_back(GameState {
        mana_used: 0,
        player: player.to_owned(),
        boss: boss.to_owned(),
    });

    let mut best_so_far: Option<u16> = None;
    while let Some(GameState {
        mana_used,
        player,
        boss,
    }) = queue.pop_front()
    {
        // Don't keep going if we've otherwise found a way to a better answer
        if let Some(best) = best_so_far
            && best <= mana_used
        {
            continue;
        }

        if boss.hit_points == 0 {
            best_so_far = Some(mana_used);
            continue;
        } else if player.hit_points == 0 {
            continue;
        }

        for spell in &ALL_SPELLS {
            if let Ok((next_player, next_boss)) = play_round(spell, &player, &boss, mode) {
                queue.push_back(GameState {
                    mana_used: mana_used + spell.cost(),
                    player: next_player,
                    boss: next_boss,
                });
            }
        }
    }

    best_so_far
}

fn part1(player: &Stats, boss: &Stats) -> Option<u16> {
    best_solution(player, boss, &Mode::Normal)
}

fn part2(player: &Stats, boss: &Stats) -> Option<u16> {
    best_solution(player, boss, &Mode::Hard)
}

fn main() -> Result<()> {
    start_day(DAY);

    let player = Stats {
        hit_points: 50,
        mana: 500,
        ..Stats::default()
    };
    let boss = Stats {
        hit_points: 71,
        damage: 10,
        ..Stats::default()
    };

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result =
        part1(&player, &boss).ok_or_else(|| anyhow!("did not find any way to defeat boss"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result =
        part2(&player, &boss).ok_or_else(|| anyhow!("did not find any way to defeat boss"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let player = Stats {
            hit_points: 10,
            mana: 250,
            ..Stats::default()
        };
        let boss = Stats {
            hit_points: 13,
            damage: 8,
            ..Stats::default()
        };
        assert_eq!(part1(&player, &boss), Some(226));
    }

    #[test]
    fn example_2() {
        let player = Stats {
            hit_points: 10,
            mana: 250,
            ..Stats::default()
        };
        let boss = Stats {
            hit_points: 14,
            damage: 8,
            ..Stats::default()
        };
        assert_eq!(part1(&player, &boss), Some(641));
    }
}
