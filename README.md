# mayo_vm

Mayo VM is a (kinda) joke [virtual machine](https://en.wikipedia.org/wiki/Virtual_machine) written in [Rust](https://www.rust-lang.org/).
It's 16-bit and based off [this](https://www.youtube.com/playlist?list=PLP29wDx6QmW5DdwpdwHCRJsEubS5NrQ9b) amazing Youtube series.
If you want to contribute just make a pull request with your changes and I'll probably take a look sometime.

Mayo VM is built, run, and tested using [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager.
The project is split between `main.rs` and the `lib` folder. Basically everything important you will find in the `lib` folder.

This project uses [cargo-cmd](https://crates.io/crates/cargo-cmd) which adds the functionality of npm scripts to cargo.
It basically just adds the ability to alias any shell command.
