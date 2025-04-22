use core::mem;

use bitfields::bitfield;
use spin::Mutex;
use x86_64::instructions::port::Port;

use super::configuration_address::{ConfigurationAddress, ConfigurationAddressBuilder};

static CONFIG_DATA: Mutex<Port<u32>> = Mutex::new(Port::new(0xCFC));

#[derive(Debug)]
pub struct ConfigurationSpace {
    configuration_address: ConfigurationAddress,
}

impl ConfigurationSpace {
    /// Safety: `bus`, `device` and `function` must not overflow (see struct for layout)
    pub unsafe fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            configuration_address: ConfigurationAddressBuilder::new()
                .with_enable(true)
                .with_bus(bus)
                .with_device(device)
                .with_function(function)
                .with_offset(0)
                .build(),
        }
    }

    /// Safety: `offset` must be aligned to 4 bytes (i.e. the two least significant bits must be 0)
    unsafe fn read(&self, offset: u8) -> u32 {
        let mut configuration_address = self.configuration_address;
        configuration_address.set_offset(offset);
        configuration_address.write();

        unsafe { CONFIG_DATA.lock().read() }
    }

    /// Identifies the particular device. Where valid IDs are allocated by the vendors
    pub fn device_id(&self) -> u16 {
        (unsafe { self.read(0x0) } >> 16) as u16
    }

    /// Identifies the manufacturer of the device. Where valid IDs are allocated by PCI-SIG (the list is
    /// [here](https://pcisig.com/membership/member-companies)) to ensure uniqueness and `0xFFFF` is an invalid value
    /// that will be returned on read accesses to Configuration Space registers of non-existent devices
    pub fn vendor_id(&self) -> u16 {
        (unsafe { self.read(0x0) }) as u16
    }

    /// A register used to record status information for PCI bus related events
    pub fn status(&self) -> Status {
        Status::from_bits((unsafe { self.read(0x4) } >> 16) as u16)
    }

    /// Provides control over a device's ability to generate and respond to PCI cycles. Where the only functionality
    /// guaranteed to be supported by all devices is, when a 0 is written to this register, the device is disconnected
    /// from the PCI bus for all accesses except Configuration Space access
    pub fn command(&self) -> Command {
        Command::from_bits((unsafe { self.read(0x4) }) as u16)
    }

    /// A read-only register that specifies the type of function the device performs
    pub fn class_code(&self) -> u8 {
        (unsafe { self.read(0x8) >> 24 }) as u8
    }

    /// A read-only register that specifies the specific function the device performs
    pub fn subclass(&self) -> u8 {
        (unsafe { self.read(0x8) >> 16 }) as u8
    }

    /// A read-only register that specifies a register-level programming interface the device has, if it has any at all
    pub fn programming_interface_byte(&self) -> u8 {
        (unsafe { self.read(0x8) >> 8 }) as u8
    }

    /// Specifies a revision identifier for a particular device. Where valid IDs are allocated by the vendor
    pub fn revision_id(&self) -> u8 {
        (unsafe { self.read(0x8) }) as u8
    }

    /// Represents that status and allows control of a device's BIST (built-in self test)
    pub fn built_in_self_test(&self) -> BuiltInSelfTest {
        BuiltInSelfTest::from_bits((unsafe { self.read(0xC) >> 24 }) as u8)
    }

    /// Identifies the layout of the rest of the header beginning at byte `0x10` of the header. If bit 7 of this register
    /// is set, the device has multiple functions; otherwise, it is a single function device
    pub fn header_type(&self) -> HeaderType {
        HeaderType::from_bits((unsafe { self.read(0xC) >> 16 }) as u8)
    }

    /// Specifies the latency timer in units of PCI bus clocks
    pub fn latency_timer(&self) -> u8 {
        (unsafe { self.read(0xC) >> 8 }) as u8
    }

    /// Specifies the system cache line size in 32-bit units. A device can limit the number of cacheline sizes it can
    /// support, if a unsupported value is written to this field, the device will behave as if a value of 0 was written
    pub fn cache_line_size(&self) -> u8 {
        (unsafe { self.read(0xC) }) as u8
    }

    /// Rest of the header, which depends on the header type
    pub fn rest(&self) -> Rest {
        match self.header_type().value() {
            HeaderTypeValue::General => Rest::General(General::new(self.configuration_address)),
            HeaderTypeValue::PciToPci => Rest::PciToPci(PciToPci::new(self.configuration_address)),
            HeaderTypeValue::PciToCardBus => {
                Rest::PciToCardBus(PciToCardBus::new(self.configuration_address))
            }
        }
    }
}

#[bitfield(u16, order = msb)]
pub struct Status {
    /// This bit will be set to 1 whenever the device detects a parity error, even if parity error handling is disabled
    ///
    /// - RW1C
    #[bits(1)]
    detected_parity_error: bool,
    /// This bit will be set to 1 whenever the device asserts SERR#
    ///
    /// - RW1C
    #[bits(1)]
    signaled_system_error: bool,
    /// This bit will be set to 1, by a master device, whenever its transaction (except for Special Cycle transactions)
    /// is terminated with Master-Abort
    ///
    /// - RW1C
    #[bits(1)]
    received_master_abort: bool,
    /// This bit will be set to 1, by a master device, whenever its transaction is terminated with Target-Abort
    ///
    /// - RW1C
    #[bits(1)]
    received_target_abort: bool,
    /// This bit will be set to 1 whenever a target device terminates a transaction with Target-Abort
    ///
    /// - RW1C
    #[bits(1)]
    signalled_target_abort: bool,
    /// Read only bits that represent the slowest time that a device will assert DEVSEL# for any bus command except
    /// Configuration Space read and writes. Where a value of `0x0` represents fast timing, a value of `0x1`
    /// represents medium timing, and a value of `0x2` represents slow timing
    ///
    /// - RO
    #[bits(2, access = ro)]
    devsel_timing: u8,
    /// This bit is only set when the following conditions are met. The bus agent asserted PERR# on a read or observed
    /// an assertion of PERR# on a write, the agent setting the bit acted as the bus master for the operation in which
    /// the error occurred, and bit 6 of the Command register (Parity Error Response bit) is set to 1
    ///
    /// - RW1C
    #[bits(1)]
    master_data_parity_error: bool,
    /// If set to 1 the device can accept fast back-to-back transactions that are not from the same agent; otherwise,
    /// transactions can only be accepted from the same agent
    ///
    /// - RO
    #[bits(1, access = ro)]
    fast_back_to_back_capable: bool,
    /// As of revision 3.0 of the PCI Local Bus specification this bit is reserved. In revision 2.1 of the specification
    /// this bit was used to indicate whether or not a device supported User Definable Features
    ///
    /// - RO
    #[bits(1, default = 0)]
    _reserved_1: bool,
    /// If set to 1 the device is capable of running at 66 MHz; otherwise, the device runs at 33 MHz
    ///
    /// - RO
    #[bits(1, access = ro)]
    freq_66_mhz_capable: bool,
    /// If set to 1 the device implements the pointer for a New Capabilities Linked list at offset 0x34; otherwise,
    /// the linked list is not available
    ///
    /// - RO
    #[bits(1, access = ro)]
    capabilities_list: bool,
    /// Represents the state of the device's INTx# signal. If set to 1 and bit 10 of the Command register
    /// (Interrupt Disable bit) is set to 0 the signal will be asserted; otherwise, the signal will be ignored
    ///
    /// - RO
    #[bits(1, access = ro)]
    interrupt_status: bool,
    /// Reserved
    #[bits(3, default = 0)]
    _reserved_2: u8,
}

#[bitfield(u16, order = msb)]
pub struct Command {
    /// Reserved
    #[bits(5, default = 0)]
    _reserved_1: u8,
    /// If set to 1 the assertion of the devices INTx# signal is disabled; otherwise, assertion of the signal is enabled
    ///
    /// - RW
    #[bits(1)]
    interrupt_disable: bool,
    /// If set to 1 indicates a device is allowed to generate fast back-to-back transactions; otherwise,
    /// fast back-to-back transactions are only allowed to the same agent
    ///
    /// - RO
    #[bits(1, access = ro)]
    fast_back_to_back_enable: bool,
    /// If set to 1 the SERR# driver is enabled; otherwise, the driver is disabled
    ///
    /// - RW
    #[bits(1)]
    serr_enable: bool,
    /// As of revision 3.0 of the PCI local bus specification this bit is hardwired to 0. In earlier versions of the
    /// specification this bit was used by devices and may have been hardwired to 0, 1, or implemented as a read/write bit
    ///
    /// - RO
    #[bits(1, default = 0)]
    _reserved_2: bool,
    /// If set to 1 the device will take its normal action when a parity error is detected; otherwise, when an error is
    /// detected, the device will set bit 15 of the Status register (Detected Parity Error Status Bit), but will not assert
    /// the PERR# (Parity Error) pin and will continue operation as normal
    ///
    /// - RW
    #[bits(1)]
    parity_error_response: bool,
    /// If set to 1 the device does not respond to palette register writes and will snoop the data; otherwise,
    /// the device will trate palette write accesses like all other accesses
    ///
    /// - RO
    #[bits(1, access = ro)]
    vga_platte_snoop: bool,
    /// If set to 1 the device can generate the Memory Write and Invalidate command; otherwise, the Memory Write
    /// command must be used
    ///
    /// - RO
    #[bits(1, access = ro)]
    memory_write_and_invalidate_enable: bool,
    /// If set to 1 the device can monitor Special Cycle operations; otherwise, the device will ignore them
    ///
    /// - RO
    #[bits(1, access = ro)]
    special_cycles: bool,
    /// If set to 1 the device can behave as a bus master; otherwise, the device can not generate PCI accesses
    ///
    /// - RW
    #[bits(1)]
    bus_master: bool,
    /// If set to 1 the device can respond to Memory Space accesses; otherwise, the device's response is disabled
    ///
    /// - RW
    #[bits(1)]
    memory_space: bool,
    /// If set to 1 the device can respond to I/O Space accesses; otherwise, the device's response is disabled
    ///
    /// - RW
    #[bits(1)]
    io_space: bool,
}

#[bitfield(u8, order = msb)]
struct BuiltInSelfTest {
    /// Will return 1 the device supports BIST
    #[bits(1)]
    capable: bool,
    /// When set to 1 the BIST is invoked. This bit is reset when BIST completes. If BIST does not complete after
    /// 2 seconds the device should be failed by system software
    #[bits(1)]
    start: bool,
    /// Reserved
    #[bits(2)]
    _reserved: u8,
    /// Will return 0, after BIST execution, if the test completed successfully
    #[bits(4)]
    completion_code: u8,
}

#[bitfield(u8, order = msb)]
pub struct HeaderType {
    /// If MF = 1 Then this device has multiple functions
    #[bits(1)]
    multi_function: bool,
    /// `0x0` Standard Header - `0x1` PCI-to-PCI Bridge - `0x2` CardBus Bridge
    #[bits(7)]
    value: HeaderTypeValue,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum HeaderTypeValue {
    General = 0,
    PciToPci = 1,
    PciToCardBus = 2,
}

impl HeaderTypeValue {
    pub const fn from_bits(bits: u8) -> Self {
        // Safety: The value of `bits` is guaranteed to be either `0x0`, `0x1` or `0x2`
        unsafe { mem::transmute(bits) }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug)]
pub enum Rest {
    General(General),
    PciToPci(PciToPci),
    PciToCardBus(PciToCardBus),
}

#[derive(Debug)]
struct General {
    configuration_address: ConfigurationAddress,
}

impl General {
    pub fn new(configuration_address: ConfigurationAddress) -> Self {
        Self {
            configuration_address,
        }
    }

    /// Safety: `offset` must be aligned to 4 bytes (i.e. the two least significant bits must be 0)
    unsafe fn read(&self, offset: u8) -> u32 {
        let mut configuration_address = self.configuration_address;
        configuration_address.set_offset(offset);
        configuration_address.write();

        unsafe { CONFIG_DATA.lock().read().to_le() }
    }

    /// Base address #0 (BAR0)
    pub fn base_address_0(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x10) })
    }

    /// Base address #1 (BAR1)
    pub fn base_address_1(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x14) })
    }

    /// Base address #2 (BAR2)
    pub fn base_address_2(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x18) })
    }

    /// Base address #3 (BAR3)
    pub fn base_address_3(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x1C) })
    }

    /// Base address #4 (BAR4)
    pub fn base_address_4(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x20) })
    }

    /// Base address #5 (BAR5)
    pub fn base_address_5(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x24) })
    }

    /// Points to the Card Information Structure and is used by devices that share silicon between CardBus and PCI
    pub fn cardbus_card_information_structure_pointer(&self) -> u32 {
        unsafe { self.read(0x28) }
    }

    pub fn subsystem_id(&self) -> u16 {
        (unsafe { self.read(0x2C) >> 16 }) as u16
    }

    pub fn subsystem_vendor_id(&self) -> u16 {
        (unsafe { self.read(0x2C) }) as u16
    }

    pub fn expansion_rom_base_address(&self) -> u32 {
        unsafe { self.read(0x30) }
    }

    /// Points (i.e. an offset into this function's configuration space) to a linked list of new capabilities implemented
    /// by the device. Used if bit 4 of the status register (Capabilities List bit) is set to 1. The bottom two bits are
    /// reserved and should be masked before the Pointer is used to access the Configuration Space
    pub fn capabilities_pointer(&self) -> u8 {
        (unsafe { self.read(0x34) }) as u8
    }

    /// A read-only register that specifies how often the device needs access to the PCI bus (in 1/4 microsecond units)
    pub fn max_latency(&self) -> u8 {
        (unsafe { self.read(0x3C) >> 24 }) as u8
    }

    /// A read-only register that specifies the burst period length, in 1/4 microsecond units, that the device needs
    /// (assuming a 33 MHz clock rate)
    pub fn min_grant(&self) -> u8 {
        (unsafe { self.read(0x3C) >> 16 }) as u8
    }

    /// Specifies which interrupt pin the device uses. Where a value of `0x1` is INTA#, `0x2` is INTB#, `0x3` is INTC#,
    /// `0x4` is INTD#, and 0x0 means the device does not use an interrupt pin
    pub fn interrupt_pin(&self) -> u8 {
        (unsafe { self.read(0x3C) >> 8 }) as u8
    }

    /// Specifies which input of the system interrupt controllers the device's interrupt pin is connected to and is
    /// implemented by any device that makes use of an interrupt pin. For the x86 architecture this register corresponds
    /// to the PIC IRQ numbers 0-15 (and not I/O APIC IRQ numbers) and a value of `0xFF` defines no connection
    pub fn interrupt_line(&self) -> u8 {
        (unsafe { self.read(0x3C) }) as u8
    }
}

#[derive(Debug)]
struct PciToPci {
    configuration_address: ConfigurationAddress,
}

impl PciToPci {
    pub fn new(configuration_address: ConfigurationAddress) -> Self {
        Self {
            configuration_address,
        }
    }

    /// Safety: `offset` must be aligned to 4 bytes (i.e. the two least significant bits must be 0)
    unsafe fn read(&self, offset: u8) -> u32 {
        let mut configuration_address = self.configuration_address;
        configuration_address.set_offset(offset);
        configuration_address.write();

        unsafe { CONFIG_DATA.lock().read() }
    }

    /// Base address #0 (BAR0)
    pub fn base_address_0(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x10) })
    }

    /// Base address #1 (BAR1)
    pub fn base_address_1(&self) -> BaseAddressRegister {
        BaseAddressRegister(unsafe { self.read(0x14) })
    }

    pub fn secondary_latency_timer(&self) -> u8 {
        (unsafe { self.read(0x18) >> 24 }) as u8
    }

    pub fn subordinate_bus_timer(&self) -> u8 {
        (unsafe { self.read(0x18) >> 16 }) as u8
    }

    pub fn secondary_bus_number(&self) -> u8 {
        (unsafe { self.read(0x18) >> 8 }) as u8
    }

    pub fn primary_bus_nubmer(&self) -> u8 {
        (unsafe { self.read(0x18) }) as u8
    }

    pub fn secondary_status(&self) -> Status {
        Status::from_bits((unsafe { self.read(0x1C) >> 16 }) as u16)
    }

    pub fn io_limit(&self) -> u8 {
        (unsafe { self.read(0x1C) >> 8 }) as u8
    }

    pub fn io_base(&self) -> u8 {
        (unsafe { self.read(0x1C) }) as u8
    }

    pub fn memory_limit(&self) -> u16 {
        (unsafe { self.read(0x20) >> 16 }) as u16
    }

    pub fn memory_base(&self) -> u16 {
        (unsafe { self.read(0x20) }) as u16
    }

    pub fn prefetchable_memory_limit(&self) -> u16 {
        (unsafe { self.read(0x24) >> 16 }) as u16
    }

    pub fn prefetchable_memory_base(&self) -> u16 {
        (unsafe { self.read(0x24) }) as u16
    }

    pub fn prefetchable_base_upper_32_bits(&self) -> u32 {
        unsafe { self.read(0x28) }
    }

    pub fn prefetchable_limit_upper_32_bits(&self) -> u32 {
        unsafe { self.read(0x2C) }
    }

    pub fn io_limit_upper_16_bits(&self) -> u16 {
        (unsafe { self.read(0x30) >> 16 }) as u16
    }

    pub fn io_base_upper_16_bits(&self) -> u16 {
        (unsafe { self.read(0x30) }) as u16
    }

    pub fn capability_pointer(&self) -> u8 {
        (unsafe { self.read(0x34) }) as u8
    }

    pub fn expansion_rom_base_address(&self) -> u32 {
        unsafe { self.read(0x38) }
    }

    pub fn bridge_control(&self) -> u16 {
        (unsafe { self.read(0x3C) >> 16 }) as u16
    }

    /// Specifies which interrupt pin the device uses. Where a value of `0x1` is INTA#, `0x2` is INTB#, `0x3` is INTC#,
    /// `0x4` is INTD#, and 0x0 means the device does not use an interrupt pin
    pub fn interrupt_pin(&self) -> u8 {
        (unsafe { self.read(0x3C) >> 8 }) as u8
    }

    /// Specifies which input of the system interrupt controllers the device's interrupt pin is connected to and is
    /// implemented by any device that makes use of an interrupt pin. For the x86 architecture this register corresponds
    /// to the PIC IRQ numbers 0-15 (and not I/O APIC IRQ numbers) and a value of `0xFF` defines no connection
    pub fn interrupt_line(&self) -> u8 {
        (unsafe { self.read(0x3C) }) as u8
    }
}

#[derive(Debug)]
struct PciToCardBus {
    configuration_address: ConfigurationAddress,
}

impl PciToCardBus {
    pub fn new(configuration_address: ConfigurationAddress) -> Self {
        Self {
            configuration_address,
        }
    }

    /// Safety: `offset` must be aligned to 4 bytes (i.e. the two least significant bits must be 0)
    unsafe fn read(&self, offset: u8) -> u32 {
        let mut configuration_address = self.configuration_address;
        configuration_address.set_offset(offset);
        configuration_address.write();

        unsafe { CONFIG_DATA.lock().read() }
    }

    // TODO
}

#[derive(Debug)]
struct BaseAddressRegister(u32);
