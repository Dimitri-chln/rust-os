use bitfields::bitfield;
use spin::Mutex;
use x86_64::instructions::port::PortWriteOnly;

static CONFIG_ADDRESS: Mutex<PortWriteOnly<u32>> = Mutex::new(PortWriteOnly::new(0xCF8));

#[bitfield(u32, order = msb)]
#[derive(Clone, Copy)]
pub struct ConfigurationAddress {
    /// Enable flag for determining when accesses to CONFIG_DATA should be translated to configuration cycles
    #[bits(1)]
    enable: bool,
    /// Reserved
    #[bits(7, default = 0)]
    _reserved: u8,
    /// Choose a specific PCI bus in the system
    #[bits(8)]
    bus: u8,
    /// Select the specific device on the PCI Bus
    #[bits(5)]
    device: u8,
    /// Choose a specific function in a device (if the device supports multiple functions)
    #[bits(3)]
    function: u8,
    /// The least significant byte selects the offset into the 256-byte configuration space available through this method.
    /// Since all reads and writes must be both 32-bits and aligned to work on all implementations, the two lowest bits of
    /// `offset` must always be zero, with the remaining six bits allowing to choose each of the 64 32-bit words.
    #[bits(8)]
    offset: u8,
}

impl ConfigurationAddress {
    /// Write configuration address to the [`CONFIG_ADDRESS`] I/O port
    pub fn write(&self) {
        unsafe { CONFIG_ADDRESS.lock().write(self.into_bits()) };
    }
}
