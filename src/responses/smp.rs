use crate::structures::smpinfo::SMPInfo;

#[repr(C)]
#[derive(Debug)]
/// Response to [SMPRequest]
pub struct SMPResponse {
    /// The response revision number
    pub revision: u64,
    /// Flags for the bootloader
    /// `Bit 0` - X2APIC has been enabled
    pub flags: u32,
    /// The boot processor's local APIC ID
    pub bsp_lapic_id: u32,
    /// The amount of CPUs available
    pub cpu_count: u64,
    /// A pointer to an array of [SMPInfo]
    pub cpus: *const *const SMPInfo,
}

impl SMPResponse {
    /// Get the CPU info slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [SMPInfo]s
    pub unsafe fn get_cpu_info(&self) -> Option<&[&SMPInfo]> {
        if self.cpus.is_null() {
            return None;
        }

        Some(core::slice::from_raw_parts(
            self.cpus as *const &SMPInfo,
            self.cpu_count as usize,
        ))
    }
}
