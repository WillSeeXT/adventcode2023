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