# Advent of Code 2022 in Rust

See https://adventofcode.com/2022

## How to run
```shell
cargo run --bin dayXX
```
where `dayXX` is the day you want to run. For example, to run day 1:
```shell
cargo run --bin day01
```

## Tests
For each day, I add a test for the sample input given in the problem description. This can be run with
```shell
cargo test
```
or 
```shell
cargo test --bin dayXX
```

## Adding a new solution
1. Make a new branch from master. 
2. Copy `template.rs` into `src/bin`. Rename it `dayXX.rs` as described above. 
3. Replace all occurences of `dayXX` in the file with the actual day. 
4. Solve it
5. Create a pull request, make sure all tests pass. 