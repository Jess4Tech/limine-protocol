#[repr(u64)]
#[derive(Debug, PartialEq, Eq)]
/// Possible types of Memory Map Entries
pub enum EntryType {
    /// Usable
    Usable = 0,
    /// Reserved
    Reserved = 1,
    /// Reclaimable after parsing ACPI Tables
    AcpiReclaimable = 2,
    /// Non-volatile storage for ACPI
    AcpiNonVolatile = 3,
    /// Bad memory
    BadMemory = 4,
    /// Reclaimable after the bootloader structures are no longer required
    BootloaderReclaimable = 5,
    /// Used by the Kernel and Modules
    KernelAndModules = 6,
    /// Allocated for a Framebuffer
    Framebuffer = 7,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
/// Entry in the Memory Map
pub struct MemoryMapEntry {
    /// The base address of the entry
    pub base: u64,
    /// The length of the entry
    pub length: u64,
    /// The type of the entry
    pub kind: EntryType,
}

impl MemoryMapEntry {
    /// The ending address of the memory map entry
    pub fn end(&self) -> u64 {
        self.base + self.length
    }
}
