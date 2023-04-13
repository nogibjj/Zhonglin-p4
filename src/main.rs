use std::io;

fn main() {
    let mut scores: Vec<i32> = vec![]; // create an empty vector to store the scores

    loop {
        println!("Enter a score (or 'q' to quit): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            break; // exit the loop if the user enters 'q'
        }

        let score: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid score. Please enter a number.");
                continue;
            }
        };

        scores.push(score); // add the entered score to the vector
    }

    let highest_score: i32 = match scores.iter().max() {
        Some(&score) => score,
        None => 0, // return 0 if there are no scores entered yet
    };

    println!("The highest score ever entered is {}", highest_score);
}
