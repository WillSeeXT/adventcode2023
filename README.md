# Advent of code 2023
https://www.adventofcode.com

First time using rust.
I use this to learn the language and challenge me.


## Day 1
### Part 1
First time using rust. I had the logic in my head : iterate all line and check all char to if it's a number. First digit got the lower index in the string , second digit get the highest.
Now how to this in rust... a lot of search and a bit of help from copilot.

### Part 2
First time, same logic but check word and replace them in the string and redo part 1. Except that it doesn't work if parts of the word is also an other word in the string (e.g. "eightwo" 82). So Same as part 1, I had the logic in my head : search each string of number and and get is position in the string the lower for the first digit and the highest for the second digit. Then repeat but with character number. Had to search a bit more on string function in rust. Finally got it, probably not the best code but it works.

## Day 2
### Part 1 && 2
No problem thinking of the logic, main problem was still doing it in rust. I think it could have been done better using regex.

Need to start using more function and use Cargo !

## Day 3
Used cargo to create the project and learn to add dependencies.
Tried to create more function to make the code more readable.
Code surely could need a clean.
### Part 1
Started by getting each number on each line and saving there starting position in the line.
Then check around each number to look for none digit symbol. The real difficulty was still getting used to rust.
### Part 2
I had to rethink my design for part 2. I decided to look for '*' and save each position. Then check for each position on the line id there was number on each side then upper and lower line. When a number was found it was saved then multiplied and sum. Again, rust was my main problem and not the logic.

## Day 4
### Part 1
Puzzle was quite easy. Used a single regex to get all information : Card id, winning numbers and card numbers.
Add to chenge my regex to take into account double whitespace.
### Part 2
Thanks to all the nice information I had from pervious part, part 2 seemed easy. My idea was working but I had major problem implementing it because of my lack of knowledge of rust. I learned A LOT on mutability, ownership, Borrowing in loop. My idea work at the end but it took me a long time to make it work in rust.
Pretty sure there is a rustier way to do it.

## Day 5
### Part 1
No real trouble here, I parsed all maps to vectors. TO get the position, I checked the range then if in range, calculate the output then cascade the output to the next map.
### Part 2
Logic was quite easy, concern was in the computing time. Manage to do it in 21min. Probably could be cut down using threads.

## Day 6
### Part 1 && 2
No real trouble here.

## Day 7
Overall code could be leaner and cleaner but it works well !
### Part 1
Fiou! Got some usual difficulties with rust. Still not getting all the ownership and borrowing. So after getting the numeric values of all the cards in hand and the bet, I managed to sort them by type of hand (high card, pair, etc). After I could sort them depending on the cards position and values. I knew there were a mathematical way of doing it by giving a kind of weight to each hand. Last few days were quite busy and my brain was not there :P. Was trying to do it with loops... but finnaly got it... it was pretty simple: Give a weight to each card position and multiply it by the card values and add it. W = C1 * 10⁸ + C2 * 10⁶ + C3 * 10⁴ + C4 * 10² + C5 * 10⁰. Then sort the hands by weight. Learned hashmaps are nice!
## Part 2
With my approach for part1, part2 was quite easy. Simply sort like part1 all the hands with no joker. Make special sort for joker hands and give joker a value of 0 so they do not influence de weight. Done deal...

## Day 8
### Part 1 && 2
We don't talk about day 8.... carry on , carry on...
Trick for the seconde part was the lower common multiple.