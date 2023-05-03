# Note taker program

This Solana program is a Note taking program that initializes a new note account to capture the notes.
## Description

This Solana program is a contract written in Rust, a programming language used for developing smart contracts on the Solana blockchain. This program was created using Anchor framework.

We use CPI to call notes program from note-taker program.

**take_note**

This function accepts note string and makes a CPI call to notes program's write_note function.


Sai Kranthi
[@iamsaikranthi](https://twitter.com/iamsaikranthi)


## License

This project is licensed under the MIT License - see the LICENSE.md file for details
