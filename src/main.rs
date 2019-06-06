// Copyright © 2019 Jason Bockover
// CS 510 RUST Course Project
// Connect Four


extern crate ndarray;
extern crate rand;
use std::io;
use ndarray::Array2;
use rand::Rng;


fn main() {
    println!("Hello, and Welcome to Connect Four!!!");
    println!("Would you like to know the rules? Use 1 for yes and 0 for no: ");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let response: u64 = response.trim().parse().expect("Please enter 1 or 0");
    match response {              //check if player wants to know the rules or not 
        1 => game_rules(),
        0 => println!("Onto the game!"),
        _ => println!("Error invalid input! \u{1F980}"), 
    };

    assert!(response == 1 || response == 0); //make sure input is valid

    let mut array = Array2::<u64>::zeros((6, 7)); //create the 6 row 7 column gameboard 
    println!("{}", array); //print the gameboard



	//let's add a randomizer to randomize who goes first 
	

	let mut rng = rand::thread_rng();
	let start = rng.gen_range(1,3); //randomly generate number to decide who goes first 

    let player1 = 1; //define the players and the counter
    let player2 = 2;

    let mut counter = 0;

    let mut cur_player; //define the current player 
    
    if start == 1 {    
      cur_player = player1; //player1 goes first
      println!("Player {} goes first!", start);
	} else if start == 2 {
	  println!("Player {} goes first!", start);
      cur_player = player2; //player 2 goes first
    } else {
		cur_player = 0;
		assert!(cur_player == 1 || cur_player == 2); //make sure cur_player is a valid number
	}
		


    let mut winner = false; //set winner to false 

    while counter < 42 && !winner { 
        //now with connect four the checkers all go to the bottom of the gameboard unless a checker is under it
        println!("Turn {}", counter); //print the turn number 
        println!("Please enter a column: "); //ask user for column
        let mut column = String::new();
        io::stdin()
            .read_line(&mut column)
            .expect("Failed to read column");
        let mut col: usize = column
            .trim()
            .parse()
            .expect("Please enter a column: 0 to 6");
        let mut row: usize = 5; //set row value to the index at the bottom of the gameboard 
        let mut done = false; 

        while !done {
            // check the rows to find an empty one
            while array[[row, col]] != 0 {
                println!("{:?}", row);
                //if not empty
                if row == 0 {
                    //if we reach the topmost row in the column and its not empty
                    println!("Error! Space is already taken! Please try again:\n");
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
                } else if row > 0 { //if the row is not at the top of the column and is already taken 
                    row -= 1; //go up to next row
                }
            }
            array[[row, col]] = cur_player; //add checker to the gameboard 
            println!("{}", array); //print the array with the new checker added 
            done = true //empty slot is found for input
        }

        while !winner { //check if we have a winner 
			//first check if we have 4 horizontal checkers that belong to cur_player
            if ((array[[5,0]] == cur_player)  &&  (array[[5,0]] == array[[5,1]]) &&  (array[[5,1]] == array[[5,2]]) && (array[[5,2]] == array[[5,3]])) ||  
						 ((array[[5,1]] == cur_player)  &&  (array[[5,1]] == array[[5,2]]) &&  (array[[5,2]] == array[[5,3]]) && (array[[5,3]] == array[[5,4]])) ||	
						 ((array[[5,2]] == cur_player)  &&  (array[[5,2]] == array[[5,3]]) &&  (array[[5,3]] == array[[5,4]]) && (array[[5,4]] == array[[5,5]])) ||
						 ((array[[5,3]] == cur_player)  &&  (array[[5,3]] == array[[5,4]]) &&  (array[[5,4]] == array[[5,5]]) && (array[[5,5]] == array[[5,6]])) ||
						 ((array[[4,0]] == cur_player)  &&  (array[[4,0]] == array[[4,1]]) &&  (array[[4,1]] == array[[4,2]]) && (array[[4,2]] == array[[4,3]])) ||
						 ((array[[4,1]] == cur_player)  &&  (array[[4,1]] == array[[4,2]]) &&  (array[[4,2]] == array[[4,3]]) && (array[[4,3]] == array[[4,4]])) ||	
						 ((array[[4,2]] == cur_player)  &&  (array[[4,2]] == array[[4,3]]) &&  (array[[4,3]] == array[[4,4]]) && (array[[4,4]] == array[[4,5]])) ||
						 ((array[[4,3]] == cur_player)  &&  (array[[4,3]] == array[[4,4]]) &&  (array[[4,4]] == array[[4,5]]) && (array[[4,5]] == array[[4,6]])) ||
						 ((array[[3,0]] == cur_player)  &&  (array[[3,0]] == array[[3,1]]) &&  (array[[3,1]] == array[[3,2]]) && (array[[3,2]] == array[[3,3]])) ||
						 ((array[[3,1]] == cur_player)  &&  (array[[3,1]] == array[[3,2]]) &&  (array[[3,2]] == array[[3,3]]) && (array[[3,3]] == array[[3,4]])) ||	
						 ((array[[3,2]] == cur_player)  &&  (array[[3,2]] == array[[3,3]]) &&  (array[[3,3]] == array[[3,4]]) && (array[[3,4]] == array[[3,5]])) ||
						 ((array[[3,3]] == cur_player)  &&  (array[[3,3]] == array[[3,4]]) &&  (array[[3,4]] == array[[3,5]]) && (array[[3,5]] == array[[3,6]])) ||
						 ((array[[2,0]] == cur_player)  &&  (array[[2,0]] == array[[2,1]]) &&  (array[[2,1]] == array[[2,2]]) && (array[[2,2]] == array[[2,3]])) ||
						 ((array[[2,1]] == cur_player)  &&  (array[[2,1]] == array[[2,2]]) &&  (array[[2,2]] == array[[2,3]]) && (array[[2,3]] == array[[2,4]])) ||	
						 ((array[[2,2]] == cur_player)  &&  (array[[2,2]] == array[[2,3]]) &&  (array[[2,3]] == array[[2,4]]) && (array[[2,4]] == array[[2,5]])) ||
						 ((array[[2,3]] == cur_player)  &&  (array[[2,3]] == array[[2,4]]) &&  (array[[2,4]] == array[[2,5]]) && (array[[2,5]] == array[[2,6]])) ||
						 ((array[[1,0]] == cur_player)  &&  (array[[1,0]] == array[[1,1]]) &&  (array[[1,1]] == array[[1,2]]) && (array[[1,2]] == array[[1,3]])) ||
						 ((array[[1,1]] == cur_player)  &&  (array[[1,1]] == array[[1,2]]) &&  (array[[1,2]] == array[[1,3]]) && (array[[1,3]] == array[[1,4]])) ||	
						 ((array[[1,2]] == cur_player)  &&  (array[[1,2]] == array[[1,3]]) &&  (array[[1,3]] == array[[1,4]]) && (array[[1,4]] == array[[1,5]])) ||
						 ((array[[1,3]] == cur_player)  &&  (array[[1,3]] == array[[1,4]]) &&  (array[[1,4]] == array[[1,5]]) && (array[[1,5]] == array[[1,6]])) ||
						 ((array[[0,0]] == cur_player)  &&  (array[[0,0]] == array[[0,1]]) &&  (array[[0,1]] == array[[0,2]]) && (array[[0,2]] == array[[0,3]])) ||
						 ((array[[0,1]] == cur_player)  &&  (array[[0,1]] == array[[0,2]]) &&  (array[[0,2]] == array[[0,3]]) && (array[[0,3]] == array[[0,4]])) ||	
						 ((array[[0,2]] == cur_player)  &&  (array[[0,2]] == array[[0,3]]) &&  (array[[0,3]] == array[[0,4]]) && (array[[0,4]] == array[[0,5]])) ||
						 ((array[[0,3]] == cur_player)  &&  (array[[0,3]] == array[[0,4]]) &&  (array[[0,4]] == array[[0,5]]) && (array[[0,5]] == array[[0,6]]))
            {
                println!("Player{} wins! 4 in a Horizontal Row!", cur_player); //if we do then the cur_player wins!
                winner = true;
                assert!(winner)
                			//next check if we have 4 vertical checkers that belong to cur_player
            } else if ((array[[5,0]] == cur_player)  &&  (array[[5,0]] == array[[4,0]]) &&  (array[[4,0]] == array[[3,0]]) && (array[[3,0]] == array[[2,0]])) || 
									((array[[4,0]] == cur_player)  &&  (array[[4,0]] == array[[3,0]]) &&  (array[[3,0]] == array[[2,0]]) && (array[[2,0]] == array[[1,0]])) ||	
									((array[[3,0]] == cur_player)  &&  (array[[3,0]] == array[[2,0]]) &&  (array[[2,0]] == array[[1,0]]) && (array[[1,0]] == array[[0,0]])) ||
									((array[[5,1]] == cur_player)  &&  (array[[5,1]] == array[[4,1]]) &&  (array[[4,1]] == array[[3,1]]) && (array[[3,1]] == array[[2,1]])) ||
									((array[[4,1]] == cur_player)  &&  (array[[4,1]] == array[[3,1]]) &&  (array[[3,1]] == array[[2,1]]) && (array[[2,1]] == array[[1,1]])) ||	
									((array[[3,1]] == cur_player)  &&  (array[[3,1]] == array[[2,1]]) &&  (array[[2,1]] == array[[1,1]]) && (array[[1,1]] == array[[0,1]])) ||
									((array[[5,2]] == cur_player)  &&  (array[[5,2]] == array[[4,2]]) &&  (array[[4,2]] == array[[3,2]]) && (array[[3,2]] == array[[2,2]])) ||
									((array[[4,2]] == cur_player)  &&  (array[[4,2]] == array[[3,2]]) &&  (array[[3,2]] == array[[2,2]]) && (array[[2,2]] == array[[1,2]])) ||	
									((array[[3,2]] == cur_player)  &&  (array[[3,2]] == array[[2,2]]) &&  (array[[2,2]] == array[[1,2]]) && (array[[1,2]] == array[[0,2]])) ||
									((array[[5,3]] == cur_player)  &&  (array[[5,3]] == array[[4,3]]) &&  (array[[4,3]] == array[[3,3]]) && (array[[3,3]] == array[[2,3]])) ||
									((array[[4,3]] == cur_player)  &&  (array[[4,3]] == array[[3,3]]) &&  (array[[3,3]] == array[[2,3]]) && (array[[2,3]] == array[[1,3]])) ||	
									((array[[3,3]] == cur_player)  &&  (array[[3,3]] == array[[2,3]]) &&  (array[[2,3]] == array[[1,3]]) && (array[[1,3]] == array[[0,3]])) ||
									((array[[5,4]] == cur_player)  &&  (array[[5,4]] == array[[4,4]]) &&  (array[[4,4]] == array[[3,4]]) && (array[[3,4]] == array[[2,4]])) ||
									((array[[4,4]] == cur_player)  &&  (array[[4,4]] == array[[3,4]]) &&  (array[[3,4]] == array[[2,4]]) && (array[[2,4]] == array[[1,4]])) ||	
									((array[[3,4]] == cur_player)  &&  (array[[3,4]] == array[[2,4]]) &&  (array[[2,4]] == array[[1,4]]) && (array[[1,4]] == array[[0,4]])) ||
									((array[[5,5]] == cur_player)  &&  (array[[5,5]] == array[[4,5]]) &&  (array[[4,5]] == array[[3,5]]) && (array[[3,5]] == array[[2,5]])) ||
									((array[[4,5]] == cur_player)  &&  (array[[4,5]] == array[[3,5]]) &&  (array[[3,5]] == array[[2,5]]) && (array[[2,5]] == array[[1,5]])) ||	
									((array[[3,5]] == cur_player)  &&  (array[[3,5]] == array[[2,5]]) &&  (array[[2,5]] == array[[1,5]]) && (array[[1,5]] == array[[0,5]])) ||
									((array[[5,6]] == cur_player)  &&  (array[[5,6]] == array[[4,6]]) &&  (array[[4,6]] == array[[3,6]]) && (array[[3,6]] == array[[2,6]])) ||
									((array[[4,6]] == cur_player)  &&  (array[[4,6]] == array[[3,6]]) &&  (array[[3,6]] == array[[2,6]]) && (array[[2,6]] == array[[1,6]])) ||	
									((array[[3,6]] == cur_player)  &&  (array[[3,6]] == array[[2,6]]) &&  (array[[2,6]] == array[[1,6]]) && (array[[1,6]] == array[[0,6]]))
            {
                println!("Player{} wins! 4 in a Vertical Row!", cur_player); //if we do then the cur_player wins!
                winner = true;
                assert!(winner)
                			//finally check if we have 4 diagonal checkers that belong to cur_player
            } else if ((array[[2,0]] == cur_player)  &&  (array[[2,0]] == array[[3,1]]) &&  (array[[3,1]] == array[[4,2]]) && (array[[4,2]] == array[[5,3]])) || 
                                    ((array[[1,0]] == cur_player)  &&  (array[[1,0]] == array[[2,1]]) &&  (array[[2,1]] == array[[3,2]]) && (array[[3,2]] == array[[4,3]])) ||						 
						            ((array[[2,1]] == cur_player)  &&  (array[[2,1]] == array[[3,2]]) &&  (array[[3,2]] == array[[4,3]]) && (array[[4,3]] == array[[5,4]])) ||
						            ((array[[0,0]] == cur_player)  &&  (array[[0,0]] == array[[1,1]]) &&  (array[[1,1]] == array[[2,2]]) && (array[[2,2]] == array[[3,3]])) ||
						            ((array[[1,1]] == cur_player)  &&  (array[[1,1]] == array[[2,2]]) &&  (array[[2,2]] == array[[3,3]]) && (array[[3,3]] == array[[4,4]])) ||
						            ((array[[2,2]] == cur_player)  &&  (array[[2,2]] == array[[3,3]]) &&  (array[[3,3]] == array[[4,4]]) && (array[[4,4]] == array[[5,5]])) ||
						            ((array[[0,1]] == cur_player)  &&  (array[[0,1]] == array[[1,2]]) &&  (array[[1,2]] == array[[2,3]]) && (array[[2,3]] == array[[3,4]])) ||
						            ((array[[1,2]] == cur_player)  &&  (array[[1,2]] == array[[2,3]]) &&  (array[[2,3]] == array[[3,4]]) && (array[[3,4]] == array[[4,5]])) ||
						            ((array[[2,3]] == cur_player)  &&  (array[[2,3]] == array[[3,4]]) &&  (array[[3,4]] == array[[4,5]]) && (array[[4,5]] == array[[5,6]])) ||
						            ((array[[0,2]] == cur_player)  &&  (array[[0,2]] == array[[1,3]]) &&  (array[[1,3]] == array[[2,4]]) && (array[[2,4]] == array[[3,5]])) ||
						            ((array[[1,3]] == cur_player)  &&  (array[[1,3]] == array[[2,4]]) &&  (array[[2,4]] == array[[3,5]]) && (array[[3,5]] == array[[4,6]])) ||
						            ((array[[0,3]] == cur_player)  &&  (array[[0,3]] == array[[1,4]]) &&  (array[[1,4]] == array[[2,5]]) && (array[[2,5]] == array[[3,6]])) || //now to check from right to left 
						           
						            ((array[[2,6]] == cur_player)  &&  (array[[2,6]] == array[[3,5]]) &&  (array[[3,5]] == array[[4,4]]) && (array[[4,4]] == array[[5,3]])) ||
						            ((array[[1,6]] == cur_player)  &&  (array[[1,6]] == array[[2,5]]) &&  (array[[2,5]] == array[[3,4]]) && (array[[3,4]] == array[[4,3]])) ||
						            ((array[[2,5]] == cur_player)  &&  (array[[2,5]] == array[[3,4]]) &&  (array[[3,4]] == array[[4,3]]) && (array[[4,3]] == array[[5,2]])) ||
						            ((array[[0,6]] == cur_player)  &&  (array[[0,6]] == array[[1,5]]) &&  (array[[1,5]] == array[[2,4]]) && (array[[2,4]] == array[[3,3]])) ||
						            ((array[[1,5]] == cur_player)  &&  (array[[1,5]] == array[[2,4]]) &&  (array[[2,4]] == array[[3,3]]) && (array[[3,3]] == array[[4,2]])) ||
						            ((array[[2,4]] == cur_player)  &&  (array[[2,4]] == array[[3,3]]) &&  (array[[3,3]] == array[[4,2]]) && (array[[4,2]] == array[[5,1]])) || 
						            ((array[[0,5]] == cur_player)  &&  (array[[0,5]] == array[[1,4]]) &&  (array[[1,4]] == array[[2,3]]) && (array[[2,3]] == array[[3,2]])) ||
						            ((array[[1,4]] == cur_player)  &&  (array[[1,4]] == array[[2,3]]) &&  (array[[2,3]] == array[[3,2]]) && (array[[3,2]] == array[[4,1]])) ||
						            ((array[[2,3]] == cur_player)  &&  (array[[2,3]] == array[[3,2]]) &&  (array[[3,2]] == array[[4,1]]) && (array[[4,1]] == array[[5,0]])) ||
						            ((array[[0,4]] == cur_player)  &&  (array[[0,4]] == array[[1,3]]) &&  (array[[1,3]] == array[[2,2]]) && (array[[2,2]] == array[[3,1]])) ||
						            ((array[[1,3]] == cur_player)  &&  (array[[1,3]] == array[[2,2]]) &&  (array[[2,2]] == array[[3,1]]) && (array[[3,1]] == array[[4,0]])) ||
						            ((array[[0,3]] == cur_player)  &&  (array[[0,3]] == array[[1,2]]) &&  (array[[1,2]] == array[[2,1]]) && (array[[2,1]] == array[[3,0]]))
            {
                println!("Player{} wins! 4 in a Diagonal Row!", cur_player); //if we do then the cur_player wins!
                winner = true;
                assert!(winner)
            } else if counter == 41 { //if both players fill the entire gameboard with 42 checkers 
                println!("IT'S A TIE!!!! Everyone wins!!"); //then the game ends in a tie!
                winner = true;
                assert!(winner)
            } else { //if no winner then break 
                break;
            }
        }

        if !winner {
            counter += 1; // increase counter for next turn
            if cur_player == player1 { //change cur_player for next turn 
                cur_player = player2;
                println!("ok your turn Player2!");
            } else if cur_player == player2 {
                cur_player = player1;
                println!("ok your turn Player1!");
            }
        }
    }
}

fn game_rules() { //displays the rules of Connect Four
    println!(
        "In Connect Four the rules are simple:\n
	You need to build a row of four checkers either vertically,\n
	horizontally or diagonally before your opponent does!!\n
	Each player is represented by a number on the gameboard: \n
	Player1 is represented by a 1 and Player 2 is represented by a 2. \n
	When it is your turn you choose the column you want to place your checker in \n
	but remember the checker will go all the way to the bottom of the gameboard. \n
	If you choose a column that is full you will be prompted to choose a new column \n
	and the game will continue. If no moves remain the game will end in a tie!!" 
    )
}
