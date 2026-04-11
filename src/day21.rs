use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Player {
    pos: u16,
    score: u16,
}

#[derive(Clone, Copy, Debug)]
struct Game {
    players: [Player; 2],
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(player: &str) -> Result<Self> {
        let (_, pos) = player
            .split_once(": ")
            .ok_or_else(|| anyhow!("invalid player '{player}'"))?;

        let pos = pos.parse()?;
        if pos == 0 || pos > 10 {
            bail!("invalid starting position '{pos}' for player");
        }

        Ok(Self { pos, score: 0 })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(players: &str) -> Result<Self> {
        let players = players
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|players: Vec<_>| anyhow!("expected 2 players, parse {}", players.len()))?;

        Ok(Self { players })
    }
}

impl Game {
    fn play_deterministic(&mut self) -> u64 {
        let mut die = (1u16..=100).cycle();
        let mut roll = || -> u16 { die.by_ref().take(3).sum() };
        let mut rolls = 0;

        loop {
            for player in &mut self.players {
                player.pos = ((player.pos - 1 + roll()) % 10) + 1;
                player.score += player.pos;

                rolls += 3;

                if player.score >= 1_000 {
                    return rolls;
                }
            }
        }
    }

    fn play_quantum(self) -> (u64, u64) {
        fn count_wins(
            player1: Player,
            player2: Player,
            cache: &mut HashMap<(Player, Player), (u64, u64)>,
        ) -> (u64, u64) {
            // (sum, count): all outcomes of rolling 3d3
            const DIRAC_ROLLS: [(u16, u64); 7] =
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

            if player2.score >= 21 {
                return (0, 1);
            }

            if let Some(&cached) = cache.get(&(player1, player2)) {
                return cached;
            }

            let mut wins = (0, 0);
            for &(roll, freq) in &DIRAC_ROLLS {
                let mut next = player1;
                next.pos = ((next.pos - 1 + roll) % 10) + 1;
                next.score += next.pos;

                // Player 2 becomes the active player next turn
                let (wins2, wins1) = count_wins(player2, next, cache);
                wins.0 += wins1 * freq;
                wins.1 += wins2 * freq;
            }

            cache.insert((player1, player2), wins);
            wins
        }

        count_wins(self.players[0], self.players[1], &mut HashMap::new())
    }
}

fn part1(mut game: Game) -> u64 {
    let rolls = game.play_deterministic();
    let loser = game
        .players
        .iter()
        .find(|&&player| player.score < 1_000)
        .unwrap();

    rolls * u64::from(loser.score)
}

fn part2(game: Game) -> u64 {
    let (wins1, wins2) = game.play_quantum();
    wins1.max(wins2)
}

fn main() -> Result<()> {
    let game = Game::from_str(&fs::read_to_string("in/day21.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(game);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 604_998);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(game);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 157_253_621_231_420);
    };

    Ok(())
}
