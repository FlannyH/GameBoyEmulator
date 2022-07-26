use super::super::GameBoy;

impl GameBoy {
    pub(in crate) fn insert_cartridge(&mut self, path: &str) -> bool {
        // Try to read the ROM file
        self.rom = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Unable to load ROM file \"{path}\", error message: {e}");
                return false;
            }
        };

        // Get ERAM size
        let eram_size = match self.rom[0x149] {
            0 => 0,
            1 => 2 * 1024,
            2 => 8 * 1024,
            3 => 32 * 1024,
            4 => 128 * 1024,
            5 => 64 * 1024,
            _ => 128 * 1024, // fall back to 128
        };

        // If the cart has ERAM
        if eram_size > 0 {
            // Try to read the RAM file
            self.save_path = path.replace(".gbc", ".gb").replace(".gb", ".sav");

            // If it read succesfully, load that into ERAM, otherwise, initialize ERAM
            self.eram = match std::fs::read(self.save_path.as_str()) {
                Ok(bytes) => bytes,
                Err(_e) => {
                    vec![0xFF; eram_size]
                }
            };
        }

        return true;
    }
}
