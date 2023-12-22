use std::fs;

struct Game {
    score: Score,
    rounds: Vec<Round>,
}

enum GameResult {
    Tie,
    PlayerWins,
    OpponentWins,
}

struct Round {
    choice_player: String,
    choice_opponent: String,
}

#[derive(Debug)]
struct Score {
    player: u32,
    opponent: u32,
}

impl Score {
    fn new(player: u32, opponent: u32) -> Self {
        Score { player, opponent }
    }
}

fn get_choice_value(choice: &str) -> u8 {
    match choice {
        "R" => 1,
        "P" => 2,
        "S" => 3,
        _ => 0,
    }
}

fn format_choice(choice: &str) -> String {
    match choice {
        "A" => "R",
        "X" => "R",
        "B" => "P",
        "Y" => "P",
        "C" => "S",
        "Z" => "S",
        _ => "R",
    }
    .to_string()
}

fn main() {
    let file = "./input.txt";
    println!("Reading file {}", file);

    let lines = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to read file: {}", error);
            return;
        }
    };

    let mut game = Game {
        score: Score {
            player: 0,
            opponent: 0,
        },
        rounds: Vec::new(),
    };

    game.rounds = lines.lines().map(|line| prepare_round(line)).collect();
    calculate_all_scores(&mut game);

    // println!("{} {}", game.score.player, game.score.opponent);
    println!("{:?}", game.score);
}

fn prepare_round(line: &str) -> Round {
    let choices: Vec<&str> = line.split_whitespace().collect();

    Round {
        choice_player: format_choice(&choices[1]),
        choice_opponent: format_choice(&choices[0]),
    }
}

fn calculate_all_scores(game: &mut Game) {
    game.score =
        game.rounds
            .iter()
            .map(|round| play_round(&round))
            .fold(Score::new(0, 0), |acc, curr| Score {
                player: acc.player + curr.player,
                opponent: acc.opponent + curr.opponent,
            });
}

fn play_round(turn: &Round) -> Score {
    const TIE_BONUS: u8 = 3;
    const WIN_BONUS: u8 = 6;
    let mut player_score: u8 = get_choice_value(&turn.choice_player);
    let mut opponent_score: u8 = get_choice_value(&turn.choice_opponent);

    let winner: GameResult = calculate_winner(&turn);

    if let GameResult::PlayerWins = winner {
        player_score += WIN_BONUS;
    }
    if let GameResult::OpponentWins = winner {
        opponent_score += WIN_BONUS;
    }
    if let GameResult::Tie = winner {
        player_score += TIE_BONUS;
        opponent_score += TIE_BONUS;
    }

    Score {
        player: player_score.into(),
        opponent: opponent_score.into(),
    }
}

fn calculate_winner(turn: &Round) -> GameResult {
    match (turn.choice_player.as_str(), turn.choice_opponent.as_str()) {
        ("R", "P") | ("P", "S") | ("S", "R") => GameResult::OpponentWins,
        ("R", "S") | ("P", "R") | ("S", "P") => GameResult::PlayerWins,
        _ => GameResult::Tie,
    }
}

#[test]
fn paper_vs_rock() {
    let round = Round {
        choice_player: format_choice("Y"),
        choice_opponent: format_choice("A"),
    };

    let returned_score = play_round(&round);

    assert_eq!(returned_score.player, 8);
    assert_eq!(returned_score.opponent, 1);
}

#[test]
fn rock_vs_paper() {
    let round = Round {
        choice_player: format_choice("X"),
        choice_opponent: format_choice("B"),
    };

    let returned_score = play_round(&round);

    assert_eq!(returned_score.player, 1);
    assert_eq!(returned_score.opponent, 8);
}

#[test]
fn scissors_vs_scissors() {
    let round = Round {
        choice_player: format_choice("Z"),
        choice_opponent: format_choice("C"),
    };

    let returned_score = play_round(&round);

    assert_eq!(returned_score.player, 6);
    assert_eq!(returned_score.opponent, 6);
}

#[test]
fn test_ties() {
    let mut game = Game {
        score: Score {
            player: 0,
            opponent: 0,
        },
        rounds: Vec::new(),
    };

    let round_one = Round {
        choice_player: "R".to_string(),
        choice_opponent: "R".to_string(),
    };

    let round_two = Round {
        choice_player: "P".to_string(),
        choice_opponent: "P".to_string(),
    };

    let round_three = Round {
        choice_player: "S".to_string(),
        choice_opponent: "S".to_string(),
    };

    game.rounds.push(round_one);
    game.rounds.push(round_two);
    game.rounds.push(round_three);

    calculate_all_scores(&mut game);

    assert_eq!(game.score.player, 15);
    assert_eq!(game.score.opponent, 15);
}

#[test]
fn test_wins() {
    let mut game = Game {
        score: Score {
            player: 0,
            opponent: 0,
        },
        rounds: Vec::new(),
    };

    let round_one = Round {
        choice_player: "R".to_string(),
        choice_opponent: "S".to_string(),
    };

    let round_two = Round {
        choice_player: "P".to_string(),
        choice_opponent: "R".to_string(),
    };

    let round_three = Round {
        choice_player: "S".to_string(),
        choice_opponent: "P".to_string(),
    };

    game.rounds.push(round_one);
    game.rounds.push(round_two);
    game.rounds.push(round_three);

    calculate_all_scores(&mut game);

    assert_eq!(game.score.player, 24);
    assert_eq!(game.score.opponent, 6);
}

#[test]
fn test_losses() {
    let mut game = Game {
        score: Score {
            player: 0,
            opponent: 0,
        },
        rounds: Vec::new(),
    };

    let round_one = Round {
        choice_player: "R".to_string(),
        choice_opponent: "P".to_string(),
    };

    let round_two = Round {
        choice_player: "P".to_string(),
        choice_opponent: "S".to_string(),
    };

    let round_three = Round {
        choice_player: "S".to_string(),
        choice_opponent: "R".to_string(),
    };

    game.rounds.push(round_one);
    game.rounds.push(round_two);
    game.rounds.push(round_three);

    calculate_all_scores(&mut game);

    assert_eq!(game.score.player, 6);
    assert_eq!(game.score.opponent, 24);
}

#[test]
fn full_game() {
    let mut game = Game {
        score: Score {
            player: 0,
            opponent: 0,
        },
        rounds: Vec::new(),
    };

    // Win
    let round_one = Round {
        choice_player: "R".to_string(),
        choice_opponent: "S".to_string(),
    };

    // Loss
    let round_two = Round {
        choice_player: "P".to_string(),
        choice_opponent: "S".to_string(),
    };

    // Tie
    let round_three = Round {
        choice_player: "P".to_string(),
        choice_opponent: "P".to_string(),
    };

    // Win
    let round_four = Round {
        choice_player: "S".to_string(),
        choice_opponent: "P".to_string(),
    };

    game.rounds.push(round_one);
    game.rounds.push(round_two);
    game.rounds.push(round_three);
    game.rounds.push(round_four);

    calculate_all_scores(&mut game);

    assert_eq!(game.score.player, 23);
    assert_eq!(game.score.opponent, 19);
}
