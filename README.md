# Rukata

Welcome to Rukata 🥋. This project contains puzzles to practice your Rust skills.

## Goal of Rukata

Just like any other language, there are features/libraries for Rust that you may not use every day.
Taking inspiration from traditional kata, Rukata is here to be a system of individual training exercises.
The original idea was to be able to practice parts of the language, standard library, or commonly used libraries.

The exercises are in the form of puzzles. Where you take an incomplete project and complete it.

This does mean that there is no explicit theme or order to these puzzles. The companion to Rukata is there to organize
the puzzles to some degree.

## Getting Started

To build Rukata, you will need to have Rust installed.
You can get it by visiting https://www.rust-lang.org/tools/install.

At the moment, we are not shipping Rukata to any package managers.

You will have to clone or download the zip of the repository.

Once setup, you should be able to run `cargo build --workspace` to build everything.

## Internal Crates

- [rukata](crates/rukata/README.md) - `Command line tool`
- [rukata-puzzle-data](crates/rukata-puzzle-data/README.md) - `Map containing all of the puzzle data`
- [rukata-settings](crates/rukata-settings/README.md) - `Global settings for rukata`
