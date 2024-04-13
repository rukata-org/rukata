# Rukata Puzzle Data

All the puzzle data for Rukata. The puzzle information is compiled and available for lookup in Rust.

## Puzzle layout

This is the general layout for a puzzle.

- `puzzle-config.json` - The information for the puzzle. Used to control what is included in the `build.rs` output.
- `starter` - The base layer used for `rukata generate`.
- `solution` - The secondary layer for `rukata solution`.
- `README.md` - General puzzle description and instructions.
- `data` - The extra files needed for the `README.md` file.

**Note** that the `README.md` file should not have a header. The header will be added with the `build.rs`. 
