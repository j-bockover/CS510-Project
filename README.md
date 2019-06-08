# CS510-Project
Copyright (c) 2019 Jason Bockover

## What is this project?
This project is a Connect Four game made in Rust for CS 510. In Connect Four two players go head to head to build a row of four checkers either vertically, horizontally or diagonally on the gameboard. First player to get four checkers in a row wins. 

## What does work?
Game is currently working without errors. The game will start by asking if the players want to know the rules and if not then the game begins. 
The game selects which player begins by random and asks the current player for an input. 
If the input is incorrect they are asked to try again. The input is checked to make sure that it is placed in an open space on the game board.
Then the gameboard is checked to see if there is a winner and if not then the turn is incremented and then it switches to the next player.


## Lessons learned
-practice, practice, practice! Only through routine practice will my coding skills get better especially when learning a new language like rust.
-rustfmt and clippy are fantastic tools! Clippy helped me in many ways to reduce redundant code (for example I had used "assert!(winner == true)" to check if either player had won and clippy helped me reduce that code to just assert!(winner)). 
And rustfmt helped me clean up my code and make it look great especially with the code that checks the gameboard to see whether a player has won or not.
-continue programming with a modular mindset instead of jumping headfirst into a program and write up a bunch of code that ends up not working. 


## License 
This program is licensed under the "MIT License." Please see the file LICENSE in the source distribution of this software for license terms.

### Instructions

1. First, download Rust by using this command in your terminal for Linux (or go to https://rust-lang.org/tools/install for other platforms):
```
curl https://sh.rustup.rs -sSf | sh
```
2. Second, please clone the repo in your terminal using this command:
```
git clone https://github.com/j-bockover/CS510-Project.git
```
3. Third, please install OpenAl and libsndfile on your system (to get the audio file to play):
    -For Linux(Debian and Ubuntu):
    ```
    sudo apt install libopenal-dev libsndfile1-dev
    ```
    -Linux(Fedora):
    ```
    sudo dnf install openal-soft-devel libsndfile-devel
    ```
    -Mac:
    ```
    brew install openal-soft libsndfile
    ```
    -Windows:
    For windows install MSYS2 by going to https://www.msys2.org and use the default installation folder.
    After installation, please run this command in the MSYS2 shell:
    ```
    pacman -S mingw-w64-x86_64-libsndfile mingw-w64-x86_64-openal
    ```
4. Navigate to the directory of the project in your terminal then build the game:
  ```
  cargo build
  ```
5. Launch the game:
  ```
  cargo run
  ```
6. HAVE FUN!
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
