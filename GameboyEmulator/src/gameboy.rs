mod gb_cpu;
mod gb_memory_operations;
mod gb_misc;

pub struct GameBoy {
    pub(self) vram: [u8; 0x2000],
    pub(self) wram: [u8; 0x2000],
    pub(self) oam: [u8; 0xA0],
    pub(self) io: [u8; 0x80],
    pub(self) hram: [u8; 0x7F],
    pub(self) ie: u8,
    pub(self) pc: u16,
}
