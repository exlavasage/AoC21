use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as read_u32};
use std::collections::HashMap;
use std::fmt;
use std::fs;

fn read(input: &str) -> nom::IResult<&str, (u32, u32)> {
    let (input, _) = tag("Player 1 starting position: ")(input)?;
    let (input, one) = read_u32(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Player 2 starting position: ")(input)?;
    let (input, two) = read_u32(input)?;

    Ok((input, (one, two)))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Player {
    pub pos: u32,
    pub score: u32,
    pub num: u32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player{}: Pos: {} Score: {}",
            self.num, self.pos, self.score
        )
    }
}

impl Player {
    fn new(pos: u32, score: u32, num: u32) -> Self {
        Self { pos, score, num }
    }

    fn advance(&mut self, roll: u32) {
        self.pos += roll;
        if self.pos > 10 {
            self.pos %= 10;
        }

        self.score += self.pos;
    }

    fn won(&self, goal: u32) -> bool {
        self.score >= goal
    }
}

static mut ROLLS: u32 = 0;
static mut NEXT_ROLL: u32 = 1;
fn next_roll() -> u32 {
    unsafe {
        ROLLS += 1;
        let ret = NEXT_ROLL;
        NEXT_ROLL += 1;
        if NEXT_ROLL > 10 {
            NEXT_ROLL %= 10;
        }
        ret
    }
}

fn play(one_pos: u32, two_pos: u32) -> usize {
    let mut player_one = Player::new(one_pos, 0, 1);
    let mut player_two = Player::new(two_pos, 0, 2);
    while !player_one.won(1000) && !player_two.won(1000) {
        player_one.advance(next_roll() + next_roll() + next_roll());
        if player_one.won(1000) {
            break;
        }
        player_two.advance(next_roll() + next_roll() + next_roll());
    }

    let loser = if player_one.won(1000) {
        player_two.score
    } else {
        player_one.score
    };
    unsafe {
        println!("{} {} {}", loser, ROLLS, loser * ROLLS);
        (loser as usize) * (ROLLS as usize)
    }
}

#[test]
fn test() {
    assert_eq!(play(4, 8), 739785);

    unsafe {
        ROLLS = 0;
        NEXT_ROLL = 1;
    }
    assert_eq!(play(7, 9), 679329);
}

fn play_multiverse(
    games: &mut HashMap<(Player, Player), (usize, usize)>,
    player1: Player,
    player2: Player,
) -> (usize, usize) {
    if player1.won(21) {
        (1, 0)
    } else if player2.won(21) {
        (0, 1)
    } else {
        let rolls = vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];
        let mut wins = (0, 0);
        for &(c1, roll1) in &rolls {
            let mut new_player1 = player1;
            new_player1.advance(roll1);
            if new_player1.won(21) {
                wins.0 += c1;
                continue;
            }
            for &(c2, roll2) in &rolls {
                let worlds = c1 * c2;
                let mut new_player2 = player2;
                new_player2.advance(roll2);
                if let Some((w1, w2)) = games.get(&(new_player1, new_player2)) {
                    wins.0 += worlds * w1;
                    wins.1 += worlds * w2;
                    continue;
                }

                let (w1, w2) = play_multiverse(games, new_player1, new_player2);
                games.insert((new_player1, new_player2), (w1, w2));

                wins.0 += c1 * c2 * w1;
                wins.1 += c1 * c2 * w2;
            }
        }
        wins
    }
}

fn main() {
    let contents = fs::read_to_string("input/day21.txt").expect("Failed to read file");
    let (_, (one_pos, two_pos)) = read(&contents).expect("Failed to read input");

    println!("{}", play(one_pos, two_pos));

    let mut games: HashMap<(Player, Player), (usize, usize)> = HashMap::new();
    let (w1, w2) = play_multiverse(
        &mut games,
        Player::new(one_pos, 0, 1),
        Player::new(two_pos, 0, 2),
    );
    println!("{} {}: {}", w1, w2, std::cmp::max(w1, w2));
}
