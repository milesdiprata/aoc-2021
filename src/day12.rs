use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

#[derive(Debug)]
struct Path {
    from: Cave,
    to: Cave,
}

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<Cave, Vec<Cave>>,
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(cave: &str) -> Result<Self> {
        match cave {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            cave if cave.chars().all(char::is_lowercase) => Ok(Self::Small(cave.to_string())),
            cave if cave.chars().all(char::is_uppercase) => Ok(Self::Big(cave.to_string())),
            _ => bail!("invalid cave '{cave}'"),
        }
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        let (from, to) = path
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid path '{path}'"))?;

        Ok(Self {
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

impl From<Vec<Path>> for Graph {
    fn from(paths: Vec<Path>) -> Self {
        let mut graph = Self {
            adj_list: HashMap::new(),
        };

        for Path { from, to } in paths {
            graph
                .adj_list
                .entry(from.clone())
                .or_default()
                .push(to.clone());
            graph.adj_list.entry(to).or_default().push(from);
        }

        graph
    }
}

impl Cave {
    const fn is_small(&self) -> bool {
        matches!(self, Self::Small(_))
    }
}

impl Graph {
    fn unique_paths(&self) -> usize {
        fn dfs<'a>(graph: &'a Graph, current: &'a Cave, visited: &mut HashSet<&'a Cave>) -> usize {
            if current == &Cave::End {
                return 1;
            }

            visited.insert(current);

            let mut count = 0;

            for next in &graph.adj_list[current] {
                if next == &Cave::Start {
                    continue;
                }

                if next.is_small() && visited.contains(next) {
                    continue;
                }

                count += dfs(graph, next, visited);

                if next.is_small() {
                    visited.remove(next);
                }
            }

            count
        }

        dfs(self, &Cave::Start, &mut HashSet::new())
    }

    fn unique_paths2(&self) -> usize {
        fn dfs<'a>(
            graph: &'a Graph,
            current: &'a Cave,
            visited: &mut HashSet<&'a Cave>,
            visited_twice: bool,
        ) -> usize {
            if current == &Cave::End {
                return 1;
            }

            visited.insert(current);

            let mut count = 0;

            for next in &graph.adj_list[current] {
                if next == &Cave::Start {
                    continue;
                }

                let visited_already = next.is_small() && visited.contains(next);
                if visited_already && visited_twice {
                    continue;
                }

                count += dfs(graph, next, visited, visited_twice || visited_already);

                if next.is_small() && !visited_already {
                    visited.remove(next);
                }
            }

            count
        }

        dfs(self, &Cave::Start, &mut HashSet::new(), false)
    }
}

fn main() -> Result<()> {
    let graph = fs::read_to_string("in/day12.txt")?
        .lines()
        .map(Path::from_str)
        .collect::<Result<Vec<_>>>()
        .map(Graph::from)?;

    {
        let start = Instant::now();
        let part1 = graph.unique_paths();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_252);
    };

    {
        let start = Instant::now();
        let part2 = graph.unique_paths2();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 147_784);
    };

    Ok(())
}
