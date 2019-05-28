// Copyright © 2019 Jason Bockover
// CS 510 Course Project
// Connect Four 
// let's use a crate to create the gameboard 
// after each turn update the gameboard 

//#[macro_use(s)]

extern crate ndarray;
use std::io;


pub fn main() {
	let mut counter = 1;
	let player1 = 1;
	let player2 = 2;
	
	println!("Hello, and Welcome to Connect Four!!!");
	println!("Would you like to know the rules? Use 1 for yes and 0 for no: ");
	let mut response = String::new();
	io::stdin().read_line(&mut response).expect("Failed to read line");
	let response: u64 = response.trim().parse().expect("Please enter 1 or 0");
	match response {
		1 => game_rules(),
		0 => println!("Onto the game!"),
		_ => println!("Error invalid input! \u{1F980}") 
	};

	assert!(response == 1 || response == 0); //make sure input is valid 
	
	 println!("Now time to play!\n");
	 //game_board();
	 use ndarray::Array2;
	let mut array = Array2::<u64>::zeros((6,7));

	println!("{}", array);

		
	let mut cur_player;
	cur_player = 1;
	
		while counter < 41 {
			let mut winner = false;
			
	//now with connect four the checkers all go to the bottom unless a checker is under it
			println!("Turn {}", counter);
			println!("Please enter a column: ");
			let mut column = String::new();
			io::stdin().read_line(&mut column).expect("Failed to read column");
			let mut column: usize = column.trim().parse().expect("Please enter a column: 0 to 6");
			let mut row:usize = 5;
			let mut done = false;
			
			while !done { // check the rows to find an empty one 
				if array[[row,column]] != 0 { //if not empty 
				row -= 1 //go up to next row 
				} else if array[[row,column]] != 0 && row == 0 {
				println!("Error! Space is already taken! Please try again:\n");
				println!("Please enter a column: ");
				let mut column2 = String::new();
				io::stdin().read_line(&mut column2).expect("Failed to read column");
				let column2: usize = column2.trim().parse().expect("Please enter a column: 0 to 6");
				column = column2;	
				} else {
					done = true
			     }
			 }
			
			array[[row,column]] = cur_player;
			
					for row in 0..5 {
						let mut count = 0;
						for column in 0..6 {
						if array[[row,column]] == array[[row + 1, column]] {
							count += 1;
					    } else {
							count -= 1;
							if count < 0 {
								count = 0
							}
				        }
				         if count == 4 {
						     winner = true
					        } else {
						    break
					         }
				      }
				    }
			//array[[row,column]] = cur_player;
			counter += 1;
			println!("{}", array);
			if cur_player == player1 {
			cur_player = player2;
			println!("ok your turn player2!");
			} else if cur_player == player2 {
			cur_player = player1;
			println!("ok your turn player1!");
			}
		}
}



fn game_rules() {
println!("WOOOOOO!!!");
println!("In Connect Four the rules are simple:\n
	You need to build a row of four checkers either vertically,\n
	horizontally or diagonally before your opponent does!!\n") 
}

/*
fn check_board(a: &[{usize} ; 2])-> bool{
	if (a[[0,0]] == a[[0,1]] && a[[0,1]] == a[[0,2]] && a[[0,2]] == a[[0,3]] && a[[0,3]] == a[[0,4]] && a[[0,4]] == a[[0,5]] && a[[0,5]] == a[[0,6]]) ||
		(a[[1,0]] == a[[1,1]] && a[[1,1]] == a[[1,2]] && a[[1,2]] == a[[1,3]] && a[[1,3]] == a[[1,4]] && a[[1,4]] == a[[1,5]] && a[[1,5]] == a[[1,6]]) || 
	   (a[[2,0]] == a[[2,1]] && a[[2,1]] == a[[2,2]] && a[[2,2]] == a[[2,3]] && a[[2,3]] == a[[2,4]] && a[[2,4]] == a[[2,5]] && a[[2,5]] == a[[2,6]]) ||
		(a[[3,0]] == a[[3,1]] && a[[3,1]] == a[[3,2]] && a[[3,2]] == a[[3,3]] && a[[3,3]] == a[[3,4]] && a[[3,4]] == a[[3,5]] && a[[3,5]] == a[[3,6]]) ||
		(a[[4,0]] == a[[4,1]] && a[[4,1]] == a[[4,2]] && a[[4,2]] == a[[4,3]] && a[[4,3]] == a[[4,4]] && a[[4,4]] == a[[4,5]] && a[[4,5]] == a[[4,6]]) ||
		(a[[5,0]] == a[[5,1]] && a[[5,1]] == a[[5,2]] && a[[5,2]] == a[[5,3]] && a[[5,3]] == a[[5,4]] && a[[5,4]] == a[[5,5]] && a[[5,5]] == a[[5,6]]) {
	
	return true 
    } else {
	return false
    }
}
* 
* 
* while !winner {
			  if (array[[0,0]] == array[[0,1]] && array[[0,1]] == array[[0,2]] && array[[0,2]] == array[[0,3]] && array[[0,3]] == array[[0,4]] && array[[0,4]] == array[[0,5]] && array[[0,5]] == array[[0,6]]) ||
		         (array[[1,0]] == array[[1,1]] && array[[1,1]] == array[[1,2]] && array[[1,2]] == array[[1,3]] && array[[1,3]] == array[[1,4]] && array[[1,4]] == array[[1,5]] && array[[1,5]] == array[[1,6]]) || 
	             (array[[2,0]] == array[[2,1]] && array[[2,1]] == array[[2,2]] && array[[2,2]] == array[[2,3]] && array[[2,3]] == array[[2,4]] && array[[2,4]] == array[[2,5]] && array[[2,5]] == array[[2,6]]) ||
		         (array[[3,0]] == array[[3,1]] && array[[3,1]] == array[[3,2]] && array[[3,2]] == array[[3,3]] && array[[3,3]] == array[[3,4]] && array[[3,4]] == array[[3,5]] && array[[3,5]] == array[[3,6]]) ||
		         (array[[4,0]] == array[[4,1]] && array[[4,1]] == array[[4,2]] && array[[4,2]] == array[[4,3]] && array[[4,3]] == array[[4,4]] && array[[4,4]] == array[[4,5]] && array[[4,5]] == array[[4,6]]) ||
		         (array[[5,0]] == array[[5,1]] && array[[5,1]] == array[[5,2]] && array[[5,2]] == array[[5,3]] && array[[5,3]] == array[[5,4]] && array[[5,4]] == array[[5,5]] && array[[5,5]] == array[[5,6]]) {
	           winner = true
            } else {
     	     break
             }
	       }
* 
* 
* 
* 
*/
