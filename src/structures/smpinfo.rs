use core::sync::atomic::{AtomicU64, Ordering};

#[repr(C)]
#[derive(Debug)]
/// CPU info structure
pub struct SMPInfo {
    /// The ID of the processor
    pub processor_id: u32,
    /// The local APIC ID of the processor
    pub lapic_id: u32,
    /// This is reserved
    reserved: u64,
    /// The address to jump to
    pub goto_address: AtomicU64,
    /// An extra argument, free for use
    pub extra_argument: u64,
}

#[cfg(test)]
impl SMPInfo {
    pub fn new_empty() -> Self {
        Self {
            processor_id: 0,
            lapic_id: 0,
            reserved: 0,
            goto_address: AtomicU64::new(0),
            extra_argument: 0,
        }
    }
}

impl PartialEq for SMPInfo {
    fn eq(&self, other: &Self) -> bool {
        self.processor_id == other.processor_id
            && self.lapic_id == other.lapic_id
            && self.reserved == other.reserved
            && self.goto_address.load(Ordering::Relaxed)
                == other.goto_address.load(Ordering::Relaxed)
            && self.extra_argument == other.extra_argument
    }
}

impl Eq for SMPInfo {}
