use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum Force {
    #[default]
    Immune,
    Infect,
}

impl std::str::FromStr for Force {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Immune System:" => Ok(Force::Immune),
            "Infection:" => Ok(Force::Infect),
            _ => anyhow::bail!("force {}", s),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum Damage {
    #[default]
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl std::str::FromStr for Damage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bludgeoning" => Ok(Damage::Bludgeoning),
            "cold" => Ok(Damage::Cold),
            "fire" => Ok(Damage::Fire),
            "radiation" => Ok(Damage::Radiation),
            "slashing" => Ok(Damage::Slashing),
            _ => anyhow::bail!("unknown damage type {}", s),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Squad {
    force: Force,
    units: usize,
    hitpoints: usize,
    weak: Vec<Damage>,
    immune: Vec<Damage>,
    attack: usize,
    damage: Damage,
    initiative: usize,
}

impl std::str::FromStr for Squad {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
          static ref RE: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points( \((.*?)\))? with an attack that does (\d+) (\S+) damage at initiative (\d+)").unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("{} regex", s))?;
        // dbg!(&caps);
        let mut squad = Self {
            units: caps.get(1).unwrap().as_str().parse()?,
            hitpoints: caps.get(2).unwrap().as_str().parse()?,
            attack: caps.get(5).unwrap().as_str().parse()?,
            damage: caps.get(6).unwrap().as_str().parse()?,
            initiative: caps.get(7).unwrap().as_str().parse()?,
            ..Default::default()
        };
        if let Some(modifiers) = caps.get(4) {
            modifiers.as_str().split("; ").for_each(|chunk| {
                if let Some(effects) = chunk.strip_prefix("weak to ") {
                    squad.weak = effects
                        .split(", ")
                        .map(|d| d.parse().unwrap())
                        .collect_vec()
                }
                if let Some(effects) = chunk.strip_prefix("immune to ") {
                    squad.immune = effects
                        .split(", ")
                        .map(|d| d.parse().unwrap())
                        .collect_vec()
                }
            });
        }
        Ok(squad)
    }
}

#[cfg(test)]
mod squad_fromstr {
    use super::*;

    #[test]
    fn immune_1() {
        assert_eq!(
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2"
                .parse::<Squad>()
                .unwrap(),
            Squad {
                units: 17,
                hitpoints: 5390,
                weak: vec![Damage::Radiation, Damage::Bludgeoning],
                attack: 4507,
                damage: Damage::Fire,
                initiative: 2,
                ..Default::default()
            }
        )
    }

    #[test]
    fn immune_2() {
        assert_eq!(
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3"
                .parse::<Squad>()
                .unwrap(),
            Squad {
                units: 989,
                hitpoints: 1274,
                immune: vec![Damage::Fire],
                weak: vec![Damage::Bludgeoning, Damage::Slashing],
                attack: 25,
                damage: Damage::Slashing,
                initiative: 3,
                ..Default::default()
            }
        )
    }

    #[test]
    fn plain() {
        assert_eq!(
          "1898 units each with 4940 hit points with an attack that does 25 slashing damage at initiative 19"
            .parse::<Squad>()
            .unwrap(),
          Squad {
            units: 1898,
            hitpoints: 4940,
            attack: 25,
            damage: Damage::Slashing,
            initiative: 19,
            ..Default::default()
          }
        )
    }
}

impl Squad {
    fn dead(&self) -> bool {
        self.units == 0
    }

    fn effective_power(&self) -> usize {
        self.units * self.attack
    }

    fn would_take(&self, attack: usize, damage: Damage) -> usize {
        if self.immune.contains(&damage) {
            0
        } else if self.weak.contains(&damage) {
            attack * 2
        } else {
            attack
        }
    }

    fn take_damage(&mut self, amount: usize) {
        let kill_units = amount / self.hitpoints;
        self.units = self.units.saturating_sub(kill_units);
        // println!("      {} are killed, leaving {}", kill_units, self.units);
    }
}

#[derive(Debug, Clone)]
struct Battlefield {
    squads: Vec<Squad>,
}

impl std::str::FromStr for Battlefield {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut squads = vec![];
        let mut force: Force = Force::default();
        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with('I') {
                force = line.parse().unwrap();
            } else {
                let mut squad: Squad = line.parse().unwrap();
                squad.force = force;
                squads.push(squad);
            }
        }

        Ok(Self { squads })
    }
}

impl Battlefield {
    fn targets(&self) -> Vec<(usize, usize)> {
        let mut targets = vec![];
        let mut targeted: HashSet<usize> = HashSet::new();
        for (i, squad) in self
            .squads
            .iter()
            .enumerate()
            .filter(|squad| !squad.1.dead())
            .sorted_by(
                |a, b| match Ord::cmp(&b.1.effective_power(), &a.1.effective_power()) {
                    Ordering::Equal => Ord::cmp(&b.1.initiative, &a.1.initiative),
                    cmp => cmp,
                },
            )
        {
            let squad_power = squad.effective_power();
            if let Some((j, target)) = self
                .squads
                .iter()
                .enumerate()
                .filter(|other| {
                    !other.1.dead() && other.1.force != squad.force && !targeted.contains(&other.0)
                })
                .sorted_by(|a, b| {
                    match Ord::cmp(
                        &b.1.would_take(squad_power, squad.damage),
                        &a.1.would_take(squad_power, squad.damage),
                    ) {
                        Ordering::Equal => {
                            match Ord::cmp(&b.1.effective_power(), &a.1.effective_power()) {
                                Ordering::Equal => Ord::cmp(&b.1.initiative, &a.1.initiative),
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                })
                .next()
            {
                if target.would_take(squad_power, squad.damage) > 0 {
                    targeted.insert(j);
                    targets.push((i, j));
                }
            }
        }
        targets
            .into_iter()
            .sorted_by_key(|&(i, _)| self.squads[i].initiative)
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod battlefield_targets {
    use super::*;

    #[test]
    fn example_1() {
        let battlefield = generate(include_str!("day24_example.txt"));
        assert_eq!(battlefield.targets(), vec![(3, 1), (1, 2), (0, 3), (2, 0)])
    }
}

impl Battlefield {
    fn damage_round(&mut self) {
        for (attacker, defender) in self.targets() {
            if self.squads[attacker].dead() {
                continue;
            }
            let damage = self.squads[defender].would_take(
                self.squads[attacker].effective_power(),
                self.squads[attacker].damage,
            );
            // println!("   {} attacks {} for {}", attacker, defender, damage);
            self.squads[defender].take_damage(damage);
        }
    }

    fn immune_score(&self) -> usize {
        self.squads
            .iter()
            .filter(|s| s.force == Force::Immune)
            .map(|s| s.units)
            .sum()
    }

    fn infect_score(&self) -> usize {
        self.squads
            .iter()
            .filter(|s| s.force == Force::Infect)
            .map(|s| s.units)
            .sum()
    }

    fn winning_score(&self) -> Option<usize> {
        match (self.immune_score(), self.infect_score()) {
            (0, infect) => Some(infect),
            (immune, 0) => Some(immune),
            _ => None,
        }
    }

    fn play_to_win(&mut self) -> Option<usize> {
        let mut last_immune = self.immune_score();
        let mut last_infect = self.infect_score();
        loop {
            self.damage_round();

            let immune = self.immune_score();
            let infect = self.infect_score();
            if last_immune == immune && last_infect == infect {
                return None;
            }
            last_immune = immune;
            last_infect = infect;
            if let Some(score) = self.winning_score() {
                return Some(score);
            }
        }
    }

    fn boost(&mut self, boost: usize) {
        for squad in &mut self.squads {
            if squad.force == Force::Immune {
                squad.attack += boost;
            }
        }
    }
}

#[aoc_generator(day24)]
fn generate(s: &str) -> Battlefield {
    s.parse().unwrap()
}

#[aoc(day24, part1)]
fn solve(battlefield: &Battlefield) -> usize {
    let mut battlefield = (*battlefield).clone();
    battlefield.play_to_win().unwrap()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day24_example.txt"))), 5216)
}

#[cfg(test)]
#[test]
fn test_boosted() {
    let mut battlefield = generate(include_str!("day24_example.txt"));
    battlefield.boost(1570);
    assert_eq!(battlefield.play_to_win().unwrap(), 51);
}

#[aoc(day24, part2)]
fn solve2(battlefield: &Battlefield) -> usize {
    for boost in 1.. {
        let mut game = (*battlefield).clone();
        game.boost(boost);
        if let Some(score) = game.play_to_win() {
            if game.immune_score() > 0 {
                return score;
            }
        }
    }
    unreachable!()
}
