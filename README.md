# Flan's Game Boy Emulator
 Game Boy Emulator written in Rust, as a way to get hands-on with the Rust programming language, and creating a proper project using it, while learning some new stuff about the Game Boy as well.
 
## Controls
(Sadly not remappable yet)
- **D-pad**: Arrow keys
- **B button**: Z key
- **A button**: X key
- **Select**: Right Shift key
- **Start**: Enter key

 
## How to run
### From release
1. Download the latest release from the `Releases` tab on this GitHub page
2. Drag a Game Boy ROM onto `flan_gb_emulator.exe`
3. Have Fun!

### From source
1. Make sure you have the Rust compiler installed
2. Clone this repository
3. Open a command prompt and navigate to the `GameBoyEmulator/GameboyEmulator/` folder
4. Run the command `cargo build --release`
5. Navigate to the `GameBoyEmulator/GameboyEmulator/target/release/` folder using your file explorer of choice
6. Drag a Game Boy ROM onto `flan_gb_emulator.exe`
7. Have fun!

## Boot ROM
If you want to have a real Nintendo boot ROM, follow these steps:
1. Navigate to the folder containing the `flan_gb_emulator.exe` executable
2. Create a folder called `bios/`
3. Put the boot rom file in that newly created `bios/` folder
4. Rename it to `dmg_boot.bin`

## Future plans
- RTC support (so games like Pokemon Gold/Siver work on this emulator)
- An actual proper UI
- Remappable controls
- Super Game Boy borders
- Super Game Boy color functionality
- Extra palettes?
