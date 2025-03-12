// Our team's match results are recorded in a collection of strings. Each match is represented by a string in the format "x:y", where x is our team's score and y is our opponents score.

// For example: ["3:1", "2:2", "0:1", ...]

// Points are awarded for each match as follows:

// if x > y: 3 points (win)
// if x < y: 0 points (loss)
// if x = y: 1 point (tie)
// We need to write a function that takes this collection and returns the number of points our team (x) got in the championship by the rules given above.

// Notes:

// our team always plays 10 matches in the championship
// 0 <= x <= 4
// 0 <= y <= 4

// -------------------------------------------------------------------------------------------

fn points(games: &[String]) -> u32 {
    let mut total_point: u32 = 0;
    for one_match in games.iter() {
        let match_score: Vec<&str> = one_match.split(":").collect();
        let x = match_score[0].parse::<u8>().unwrap();
        let y = match_score[1].parse::<u8>().unwrap();
        match x {
            t if t > y => total_point += 3,
            t if t == y => total_point += 1,
            _ => (),
        }
    }
    return total_point;
}

