# Flan's Game Boy Emulator
 Game Boy Emulator written in Rust, as a way to get hands-on with the Rust programming language, and creating a proper project using it, while learning some new stuff about the Game Boy as well.
 
## How to compile and run
 - Make sure you have the Rust compiler installed
 - Download or rip the DMG (Original Fat Game Boy) boot rom
 - Create a new folder inside `GameBoyEmulator/GameboyEmulator/` called `bios/` (this step will change in the next push)
 - Put the boot rom file into that new folder with the file name `dmg_boot.bin`
 - Open a command prompt and navigate to the `GameBoyEmulator/GameboyEmulator/` folder
 - Run the command `cargo run` to run it, or `cargo build` to build it

 Your folder structure should look something like this:
```
GameBoyEmulator/
   GameboyEmulator/
      bios/
         dmg_boot.bin
      src/
      test_roms/
      Cargo.lock
      Cargo.toml
   ```
           
 Then open a command prompt in `GameBoyEmulator/GameboyEmulator/` and type `cargo run`.
