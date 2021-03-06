// Copyright © 2019 Jason Bockover
// CS 510 RUST Course Project
// Connect Four

extern crate colored;
extern crate ndarray;
extern crate rand;

extern crate ears;
use ears::{AudioController, Sound};

use colored::*;
use ndarray::Array2;
use rand::Rng;
use std::io;

fn main() {
    println!(
        "{}",
        "Hello, and Welcome to Connect Four!!!".bright_yellow()
    );
    println!(
        "{}",
        "Would you like to know the rules? Use 1 for yes and 0 for no: ".green()
    );
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let response: u64 = response.trim().parse().expect("Please enter 1 or 0");
    match response {
        //check if player wants to know the rules or not
        1 => game_rules(),
        0 => println!("Onto the game!\u{1F980}"),
        777 => secret(),
        _ => println!("Error invalid input! Please try again! \u{1F6AB}"),
    };

    assert!(response == 1 || response == 0); //make sure input is valid

    let mut array = Array2::<u64>::zeros((6, 7)); //create the 6 row 7 column gameboard
    println!("{}", array); //print the gameboard

    let player1 = 1; //define the players and the counter
    let player2 = 2;

    let mut counter = 0;

    let mut cur_player; //define the current player

    //let's add a randomizer to randomize who goes first

    let mut rng = rand::thread_rng();
    let start = rng.gen_range(1, 3); //randomly generate number to decide who goes first

    match start {
        //check which player is up first
        1 => cur_player = player1, //player 1 goes first
        2 => cur_player = player2, //player 2 goes first
        _ => cur_player = 0,       //error has occurred
    };

    assert!(cur_player == 1 || cur_player == 2); //make sure cur_player is a valid number

    println!("Player {} goes first!", start);

    let mut winner = false; //set winner to false

    while counter < 42 && !winner {
        //now with connect four the checkers all go to the bottom of the gameboard unless a checker is under it
        println!("{} {}", "Turn:".yellow().bright_purple(), counter); //print the turn number
        println!("Please enter a column: "); //ask user for column
        let mut column = String::new();
        io::stdin()
            .read_line(&mut column)
            .expect("Failed to read column");
        let mut col: usize = column
            .trim()
            .parse()
            .expect("Please enter a column: 0 to 6");
        //make sure column input is valid
        let mut valid = false;
        while !valid {
            if col < 7 {
                valid = true
            } else {
                println!(
                    "{}",
                    "Invalid Column!! Please enter a different column: "
                        .red()
                        .bold()
                        .blink()
                ); //have user choose new column
                let mut column2 = String::new();
                io::stdin()
                    .read_line(&mut column2)
                    .expect("Failed to read column");
                let col2: usize = column2
                    .trim()
                    .parse()
                    .expect("Please enter a column: 0 to 6");
                col = col2;
            }
        }

        let mut row: usize = 5; //set row value to the index at the bottom of the gameboard
        let mut done = false; //space to place checker has not been found yet

        while !done {
            // check the rows to find an empty one
            while array[[row, col]] != 0 {
                println!("{:?}", row);
                //if not empty
                if row == 0 {
                    //if we reach the topmost row in the column and its not empty
                    println!(
                        "{}",
                        "Error! Space is already taken! Please try again:\n"
                            .red()
                            .bold()
                            .blink()
                    );
                    println!("Please enter a different column: "); //have user choose new column
                    let mut column2 = String::new();
                    io::stdin()
                        .read_line(&mut column2)
                        .expect("Failed to read column");
                    let col2: usize = column2
                        .trim()
                        .parse()
                        .expect("Please enter a column: 0 to 6");
                    col = col2;
                    row = 5; //reset row to start at the bottom of the new column chosen
                } else if row > 0 {
                    //if the row is not at the top of the column and is already taken
                    row -= 1; //go up to next row
                }
            }
            array[[row, col]] = cur_player; //add checker to the gameboard
            println!("{}", array); //print the array with the new checker added
            done = true //empty slot is found for input
        }

        while !winner {
            //check if we have a winner
            let mut sound = Sound::new("applause.ogg").unwrap(); //load applause sound for when player wins
                                                                 //first check if we have 4 horizontal checkers that belong to cur_player
            if ((array[[5, 0]] == cur_player)
                && (array[[5, 0]] == array[[5, 1]])
                && (array[[5, 1]] == array[[5, 2]])
                && (array[[5, 2]] == array[[5, 3]]))
                || ((array[[5, 1]] == cur_player)
                    && (array[[5, 1]] == array[[5, 2]])
                    && (array[[5, 2]] == array[[5, 3]])
                    && (array[[5, 3]] == array[[5, 4]]))
                || ((array[[5, 2]] == cur_player)
                    && (array[[5, 2]] == array[[5, 3]])
                    && (array[[5, 3]] == array[[5, 4]])
                    && (array[[5, 4]] == array[[5, 5]]))
                || ((array[[5, 3]] == cur_player)
                    && (array[[5, 3]] == array[[5, 4]])
                    && (array[[5, 4]] == array[[5, 5]])
                    && (array[[5, 5]] == array[[5, 6]]))
                || ((array[[4, 0]] == cur_player)
                    && (array[[4, 0]] == array[[4, 1]])
                    && (array[[4, 1]] == array[[4, 2]])
                    && (array[[4, 2]] == array[[4, 3]]))
                || ((array[[4, 1]] == cur_player)
                    && (array[[4, 1]] == array[[4, 2]])
                    && (array[[4, 2]] == array[[4, 3]])
                    && (array[[4, 3]] == array[[4, 4]]))
                || ((array[[4, 2]] == cur_player)
                    && (array[[4, 2]] == array[[4, 3]])
                    && (array[[4, 3]] == array[[4, 4]])
                    && (array[[4, 4]] == array[[4, 5]]))
                || ((array[[4, 3]] == cur_player)
                    && (array[[4, 3]] == array[[4, 4]])
                    && (array[[4, 4]] == array[[4, 5]])
                    && (array[[4, 5]] == array[[4, 6]]))
                || ((array[[3, 0]] == cur_player)
                    && (array[[3, 0]] == array[[3, 1]])
                    && (array[[3, 1]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[3, 3]]))
                || ((array[[3, 1]] == cur_player)
                    && (array[[3, 1]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[3, 4]]))
                || ((array[[3, 2]] == cur_player)
                    && (array[[3, 2]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[3, 5]]))
                || ((array[[3, 3]] == cur_player)
                    && (array[[3, 3]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[3, 5]])
                    && (array[[3, 5]] == array[[3, 6]]))
                || ((array[[2, 0]] == cur_player)
                    && (array[[2, 0]] == array[[2, 1]])
                    && (array[[2, 1]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[2, 3]]))
                || ((array[[2, 1]] == cur_player)
                    && (array[[2, 1]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[2, 4]]))
                || ((array[[2, 2]] == cur_player)
                    && (array[[2, 2]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[2, 5]]))
                || ((array[[2, 3]] == cur_player)
                    && (array[[2, 3]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[2, 5]])
                    && (array[[2, 5]] == array[[2, 6]]))
                || ((array[[1, 0]] == cur_player)
                    && (array[[1, 0]] == array[[1, 1]])
                    && (array[[1, 1]] == array[[1, 2]])
                    && (array[[1, 2]] == array[[1, 3]]))
                || ((array[[1, 1]] == cur_player)
                    && (array[[1, 1]] == array[[1, 2]])
                    && (array[[1, 2]] == array[[1, 3]])
                    && (array[[1, 3]] == array[[1, 4]]))
                || ((array[[1, 2]] == cur_player)
                    && (array[[1, 2]] == array[[1, 3]])
                    && (array[[1, 3]] == array[[1, 4]])
                    && (array[[1, 4]] == array[[1, 5]]))
                || ((array[[1, 3]] == cur_player)
                    && (array[[1, 3]] == array[[1, 4]])
                    && (array[[1, 4]] == array[[1, 5]])
                    && (array[[1, 5]] == array[[1, 6]]))
                || ((array[[0, 0]] == cur_player)
                    && (array[[0, 0]] == array[[0, 1]])
                    && (array[[0, 1]] == array[[0, 2]])
                    && (array[[0, 2]] == array[[0, 3]]))
                || ((array[[0, 1]] == cur_player)
                    && (array[[0, 1]] == array[[0, 2]])
                    && (array[[0, 2]] == array[[0, 3]])
                    && (array[[0, 3]] == array[[0, 4]]))
                || ((array[[0, 2]] == cur_player)
                    && (array[[0, 2]] == array[[0, 3]])
                    && (array[[0, 3]] == array[[0, 4]])
                    && (array[[0, 4]] == array[[0, 5]]))
                || ((array[[0, 3]] == cur_player)
                    && (array[[0, 3]] == array[[0, 4]])
                    && (array[[0, 4]] == array[[0, 5]])
                    && (array[[0, 5]] == array[[0, 6]]))
            {
                sound.play(); //play winning sound
                println!(
                    "Player{} wins! 4 in a Horizontal Row! \u{1F3C6} \u{1F31F} \u{1F973} \u{1F44F}",
                    cur_player
                ); //if we do then the cur_player wins!
                winner = true;
                assert!(winner);
                play_again()
            //next check if we have 4 vertical checkers that belong to cur_player
            } else if ((array[[5, 0]] == cur_player)
                && (array[[5, 0]] == array[[4, 0]])
                && (array[[4, 0]] == array[[3, 0]])
                && (array[[3, 0]] == array[[2, 0]]))
                || ((array[[4, 0]] == cur_player)
                    && (array[[4, 0]] == array[[3, 0]])
                    && (array[[3, 0]] == array[[2, 0]])
                    && (array[[2, 0]] == array[[1, 0]]))
                || ((array[[3, 0]] == cur_player)
                    && (array[[3, 0]] == array[[2, 0]])
                    && (array[[2, 0]] == array[[1, 0]])
                    && (array[[1, 0]] == array[[0, 0]]))
                || ((array[[5, 1]] == cur_player)
                    && (array[[5, 1]] == array[[4, 1]])
                    && (array[[4, 1]] == array[[3, 1]])
                    && (array[[3, 1]] == array[[2, 1]]))
                || ((array[[4, 1]] == cur_player)
                    && (array[[4, 1]] == array[[3, 1]])
                    && (array[[3, 1]] == array[[2, 1]])
                    && (array[[2, 1]] == array[[1, 1]]))
                || ((array[[3, 1]] == cur_player)
                    && (array[[3, 1]] == array[[2, 1]])
                    && (array[[2, 1]] == array[[1, 1]])
                    && (array[[1, 1]] == array[[0, 1]]))
                || ((array[[5, 2]] == cur_player)
                    && (array[[5, 2]] == array[[4, 2]])
                    && (array[[4, 2]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[2, 2]]))
                || ((array[[4, 2]] == cur_player)
                    && (array[[4, 2]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[1, 2]]))
                || ((array[[3, 2]] == cur_player)
                    && (array[[3, 2]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[1, 2]])
                    && (array[[1, 2]] == array[[0, 2]]))
                || ((array[[5, 3]] == cur_player)
                    && (array[[5, 3]] == array[[4, 3]])
                    && (array[[4, 3]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[2, 3]]))
                || ((array[[4, 3]] == cur_player)
                    && (array[[4, 3]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[1, 3]]))
                || ((array[[3, 3]] == cur_player)
                    && (array[[3, 3]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[1, 3]])
                    && (array[[1, 3]] == array[[0, 3]]))
                || ((array[[5, 4]] == cur_player)
                    && (array[[5, 4]] == array[[4, 4]])
                    && (array[[4, 4]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[2, 4]]))
                || ((array[[4, 4]] == cur_player)
                    && (array[[4, 4]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[1, 4]]))
                || ((array[[3, 4]] == cur_player)
                    && (array[[3, 4]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[1, 4]])
                    && (array[[1, 4]] == array[[0, 4]]))
                || ((array[[5, 5]] == cur_player)
                    && (array[[5, 5]] == array[[4, 5]])
                    && (array[[4, 5]] == array[[3, 5]])
                    && (array[[3, 5]] == array[[2, 5]]))
                || ((array[[4, 5]] == cur_player)
                    && (array[[4, 5]] == array[[3, 5]])
                    && (array[[3, 5]] == array[[2, 5]])
                    && (array[[2, 5]] == array[[1, 5]]))
                || ((array[[3, 5]] == cur_player)
                    && (array[[3, 5]] == array[[2, 5]])
                    && (array[[2, 5]] == array[[1, 5]])
                    && (array[[1, 5]] == array[[0, 5]]))
                || ((array[[5, 6]] == cur_player)
                    && (array[[5, 6]] == array[[4, 6]])
                    && (array[[4, 6]] == array[[3, 6]])
                    && (array[[3, 6]] == array[[2, 6]]))
                || ((array[[4, 6]] == cur_player)
                    && (array[[4, 6]] == array[[3, 6]])
                    && (array[[3, 6]] == array[[2, 6]])
                    && (array[[2, 6]] == array[[1, 6]]))
                || ((array[[3, 6]] == cur_player)
                    && (array[[3, 6]] == array[[2, 6]])
                    && (array[[2, 6]] == array[[1, 6]])
                    && (array[[1, 6]] == array[[0, 6]]))
            {
                sound.play(); //play winning sound
                println!(
                    "Player{} wins! 4 in a Vertical Row! \u{1F3C6} \u{1F31F} \u{1F973} \u{1F44F}",
                    cur_player
                ); //if we do then the cur_player wins!
                winner = true;
                assert!(winner);
                play_again()
            //finally check if we have 4 diagonal checkers that belong to cur_player
            } else if ((array[[2, 0]] == cur_player)
                && (array[[2, 0]] == array[[3, 1]])
                && (array[[3, 1]] == array[[4, 2]])
                && (array[[4, 2]] == array[[5, 3]]))
                || ((array[[1, 0]] == cur_player)
                    && (array[[1, 0]] == array[[2, 1]])
                    && (array[[2, 1]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[4, 3]]))
                || ((array[[2, 1]] == cur_player)
                    && (array[[2, 1]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[4, 3]])
                    && (array[[4, 3]] == array[[5, 4]]))
                || ((array[[0, 0]] == cur_player)
                    && (array[[0, 0]] == array[[1, 1]])
                    && (array[[1, 1]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[3, 3]]))
                || ((array[[1, 1]] == cur_player)
                    && (array[[1, 1]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[4, 4]]))
                || ((array[[2, 2]] == cur_player)
                    && (array[[2, 2]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[4, 4]])
                    && (array[[4, 4]] == array[[5, 5]]))
                || ((array[[0, 1]] == cur_player)
                    && (array[[0, 1]] == array[[1, 2]])
                    && (array[[1, 2]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[3, 4]]))
                || ((array[[1, 2]] == cur_player)
                    && (array[[1, 2]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[4, 5]]))
                || ((array[[2, 3]] == cur_player)
                    && (array[[2, 3]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[4, 5]])
                    && (array[[4, 5]] == array[[5, 6]]))
                || ((array[[0, 2]] == cur_player)
                    && (array[[0, 2]] == array[[1, 3]])
                    && (array[[1, 3]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[3, 5]]))
                || ((array[[1, 3]] == cur_player)
                    && (array[[1, 3]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[3, 5]])
                    && (array[[3, 5]] == array[[4, 6]]))
                || ((array[[0, 3]] == cur_player)
                    && (array[[0, 3]] == array[[1, 4]])
                    && (array[[1, 4]] == array[[2, 5]])
                    && (array[[2, 5]] == array[[3, 6]]))
                || ((array[[2, 6]] == cur_player)
                    && (array[[2, 6]] == array[[3, 5]])
                    && (array[[3, 5]] == array[[4, 4]])
                    && (array[[4, 4]] == array[[5, 3]]))
                || ((array[[1, 6]] == cur_player)
                    && (array[[1, 6]] == array[[2, 5]])
                    && (array[[2, 5]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[4, 3]]))
                || ((array[[2, 5]] == cur_player)
                    && (array[[2, 5]] == array[[3, 4]])
                    && (array[[3, 4]] == array[[4, 3]])
                    && (array[[4, 3]] == array[[5, 2]]))
                || ((array[[0, 6]] == cur_player)
                    && (array[[0, 6]] == array[[1, 5]])
                    && (array[[1, 5]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[3, 3]]))
                || ((array[[1, 5]] == cur_player)
                    && (array[[1, 5]] == array[[2, 4]])
                    && (array[[2, 4]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[4, 2]]))
                || ((array[[2, 4]] == cur_player)
                    && (array[[2, 4]] == array[[3, 3]])
                    && (array[[3, 3]] == array[[4, 2]])
                    && (array[[4, 2]] == array[[5, 1]]))
                || ((array[[0, 5]] == cur_player)
                    && (array[[0, 5]] == array[[1, 4]])
                    && (array[[1, 4]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[3, 2]]))
                || ((array[[1, 4]] == cur_player)
                    && (array[[1, 4]] == array[[2, 3]])
                    && (array[[2, 3]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[4, 1]]))
                || ((array[[2, 3]] == cur_player)
                    && (array[[2, 3]] == array[[3, 2]])
                    && (array[[3, 2]] == array[[4, 1]])
                    && (array[[4, 1]] == array[[5, 0]]))
                || ((array[[0, 4]] == cur_player)
                    && (array[[0, 4]] == array[[1, 3]])
                    && (array[[1, 3]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[3, 1]]))
                || ((array[[1, 3]] == cur_player)
                    && (array[[1, 3]] == array[[2, 2]])
                    && (array[[2, 2]] == array[[3, 1]])
                    && (array[[3, 1]] == array[[4, 0]]))
                || ((array[[0, 3]] == cur_player)
                    && (array[[0, 3]] == array[[1, 2]])
                    && (array[[1, 2]] == array[[2, 1]])
                    && (array[[2, 1]] == array[[3, 0]]))
            {
                sound.play(); //play winning sound
                println!(
                    "Player{} wins! 4 in a Diagonal Row! \u{1F3C6} \u{1F31F} \u{1F973} \u{1F44F}",
                    cur_player
                ); //if we do then the cur_player wins!
                winner = true;
                assert!(winner);
                play_again()
            } else if counter == 41 {
                //if both players fill the entire gameboard with 42 checkers
                println!("IT'S A TIE!!!! Everyone wins!! \u{1F44D}"); //then the game ends in a tie!
                winner = true;
                assert!(winner);
                play_again()
            } else {
                //if no winner then break
                break;
            }
        }

        if !winner {
            counter += 1; // increase counter for next turn
            if cur_player == player1 {
                //change cur_player for next turn
                cur_player = player2;
                println!("{}", "Ok, your turn Player2!".color("blue"));
            } else if cur_player == player2 {
                cur_player = player1;
                println!("{}", "Ok, your turn Player1!".color("red"));
            }
        }
    }
}

fn game_rules() {
    //displays the rules of Connect Four
    println!(
        "{}",
        "In Connect Four the rules are simple:\n
	You need to build a row of four checkers either vertically,\n
	horizontally or diagonally before your opponent does!!\n
	Each player checker is represented by a number on the gameboard: \n
	Player1 is represented by a 1 and Player 2 is represented by a 2. \n
	When it is your turn you choose the column you want to place your checker in \n
	but remember the checker will go all the way to the bottom of the gameboard! \n
	If you choose a column that is full you will be prompted to choose a new column \n
	and the game will continue. If no moves remain the game will end in a tie!!"
            .bright_cyan()
    )
}

fn play_again() {
    println!("{}", "Do you want to play again? Yes or No: ".yellow());
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let response: String = response.trim().parse().expect("Please enter yes or no");
    match response.as_ref() {
        "YES" | "Yes" | "yes" => main(),
        "NO" | "No" | "no" => println!("Game Over"),
        _ => println!("Error invalid input! \u{1F980}"),
    };
}

fn secret() {
    println!(
        "\u{1F52E} You found the secret! \u{1F192} \n
	Now time to test your \u{1F9E0}! Here's a riddle: \n
	I speak without a mouth \u{1F444} and hear without ears \u{1F442}. I have no body but \n
	I come alive with wind \u{1F32C}. What am I? \n
	1): Windmill
	2): Echo
	3): Ghost \n
	Please enter your input 1, 2, or 3: \n"
    );
    let mut correct = false;
    while !correct {
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read answer");
        let answer: u64 = answer.trim().parse().expect("Please enter 1, 2 or 3");
        match answer {
            2 => correct = true,
            _ => println!("Incorrect! Please try again!"),
        };
    }
    println!(
        "You are correct! Fantastic! \u{1F386} \u{1F387} \u{1F386} \n
		Here is your reward! \u{1F4B0} \n
		Now back to the game! \n"
    );
    main();
}
