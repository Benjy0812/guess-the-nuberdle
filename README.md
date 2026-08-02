# Numberdle

Numberdle is Wordle, but with numbers instead of words. Instead of guessing a five-letter word, you guess a secret number, and each guess tells you which digits are correct, which are present but misplaced, and which are not in the number at all.

This project was built as a learning exercise in Rust. It is open source and free for anyone to read, run, fork, or use as a starting point for their own project.

## How it works

1. The game picks a secret number based on the chosen difficulty.
2. You guess a number of the same length.
3. For each digit, the game tells you:
   - Correct digit in the correct position
   - Correct digit in the wrong position
   - Digit not in the number
4. You keep guessing until you find the number or run out of attempts.

## Difficulty levels

- Easy: 1 to 5 digits
- Medium: 1 to 10 digits
- Hard: 1 to 15 digits

## Running the project

This project is built with Cargo, Rust's package manager and build tool.

```
cargo run
```

## Why this project exists

This is primarily a learning project, an excuse to practice core Rust concepts such as ownership, pattern matching, enums, and string/number parsing, while building something small and fun. It is shared publicly in case it is useful to anyone else learning Rust or looking for a simple project idea to build on.

## Contributing

This is a personal learning project, so there are no formal contribution guidelines. That said, issues, forks, and pull requests are welcome if you spot a bug or want to extend it.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

The MIT License was chosen because it is short, permissive, and puts as few restrictions as possible on anyone who wants to read, learn from, reuse, or build on this code.
