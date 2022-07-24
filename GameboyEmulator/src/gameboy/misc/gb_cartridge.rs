
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
        return true;
    }
}
