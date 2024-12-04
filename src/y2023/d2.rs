const INPUT: &str = include_str!("../../inputs/y2023/d2.txt");

struct Game {
    pub id: u32,
    pub groups: Vec<CountGroup>,
}

impl Game {
    pub fn new(id: u32, groups: Vec<CountGroup>) -> Self {
        Self { id, groups }
    }
}

#[derive(Debug)]
struct CountGroup {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl CountGroup {
    pub fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

fn load_games() -> Vec<Game> {
    let mut res = Vec::new();
    for line in INPUT.lines() {
        // * Skip the 'Game ' prefix of every line.
        let mut chars = line.chars().skip(5);

        // * Read game id.
        let id = chars
            .by_ref()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let mut groups = Vec::new();
        let mut group = CountGroup::new();
        // * Skip ' ' at the start of each count.
        while let Some(_) = chars.next() {
            // * Read a count for this group.
            let count = chars
                .by_ref()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            // * Read and set the count's color.
            let color = chars.next().unwrap();
            match color {
                'r' => group.red = count,
                'g' => group.green = count,
                'b' => group.blue = count,
                _ => panic!("Unexpected character while trying to parse color!"),
            }

            let mut game_finished = true;
            while let Some(c) = chars.next() {
                match c {
                    // * Start parsing the next group.
                    ';' => {
                        groups.push(group);
                        group = CountGroup::new();
                        game_finished = false;
                        break;
                    }
                    // * Start parsing the next count.
                    ',' => {
                        game_finished = false;
                        break;
                    }
                    _ => continue,
                }
            }

            // * Start parsing the next game.
            if game_finished {
                groups.push(group);
                res.push(Game::new(id, groups));
                break;
            }
        }
    }
    return res;
}

pub fn solve_pt1() -> u32 {
    const REQUIRED_RED: u32 = 12;
    const REQUIRED_GREEN: u32 = 13;
    const REQUIRED_BLUE: u32 = 14;

    let mut res = 0;

    let games = load_games();
    for game in games {
        // * Find the maximum counts for each color from this game's groups.
        let mut max_counts = CountGroup::new();
        for group in game.groups {
            max_counts.red = max_counts.red.max(group.red);
            max_counts.green = max_counts.green.max(group.green);
            max_counts.blue = max_counts.blue.max(group.blue);
        }

        // * Add the game's id to the result if it meets the required amounts.
        if max_counts.red <= REQUIRED_RED
            && max_counts.green <= REQUIRED_GREEN
            && max_counts.blue <= REQUIRED_BLUE
        {
            res += game.id;
        }
    }

    return res;
}

pub fn solve_pt2() -> u32 {
    let mut res = 0;

    let games = load_games();
    for game in games {
        // * Find the maximum counts for each color from this game's groups.
        let mut max_counts = CountGroup::new();
        for group in game.groups {
            max_counts.red = max_counts.red.max(group.red);
            max_counts.green = max_counts.green.max(group.green);
            max_counts.blue = max_counts.blue.max(group.blue);
        }

        // * Add the 'power' (red * green * blue) of each game to the result.
        res += max_counts.red * max_counts.green * max_counts.blue;
    }

    return res;
}
