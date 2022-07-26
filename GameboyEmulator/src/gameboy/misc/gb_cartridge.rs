use super::super::GameBoy;

impl GameBoy {
    pub(in crate) fn insert_cartridge(&mut self, path: &str) -> bool {
        // Try to read the file
        self.rom = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Unable to load ROM file \"{path}\", error message: {e}");
                return false;
            }
        };

        // Init External RAM
        self.eram = vec![
            0xFF;
            match self.rom[0x149] {
                0 => 0,
                1 => 2 * 1024,
                2 => 8 * 1024,
                3 => 32 * 1024,
                4 => 128 * 1024,
                5 => 64 * 1024,
                _ => 128 * 1024, // fall back to 128
            }
        ];

        return true;
    }
}
