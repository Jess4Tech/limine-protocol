use core::ffi::CStr;

#[allow(unused_imports)]
use crate::{
    requests::*,
    structures::{
        file::File,
        framebuffer::Framebuffer,
        memory_map_entry::MemoryMapEntry,
        smpinfo::SMPInfo,
        terminal::{Terminal, TerminalWriteFn},
    },
};

#[repr(C)]
#[derive(Debug)]
/// Response to [InfoRequest]
pub struct InfoResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// A null-terminated string of the bootloader's name
    pub name: &'a CStr,
    /// A null-terminated string of the bootloader's version
    pub version: &'a CStr,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [StackSizeRequest]
pub struct StackSizeResponse {
    /// The response revision number
    pub revision: u64,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [HHDMRequest]
pub struct HHDMResponse {
    /// The response revision number
    pub revision: u64,
    /// The offset of the HHDM
    pub offset: u64,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [TerminalRequest]
pub struct TerminalResponse {
    /// The response revision number
    pub revision: u64,
    /// The number of [Terminal]s in `terminals`
    pub terminal_count: u64,
    /// A pointer to an array of [Terminal] pointers
    pub terminals: *const *const Terminal,
    /// The terminal write function
    pub write: TerminalWriteFn,
}

impl TerminalResponse {
    /// Get the terminal slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [Terminal]s
    pub unsafe fn get_terminals(&self) -> Option<&[&Terminal]> {
        if self.terminals.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.terminals as *const &Terminal,
            self.terminal_count as usize,
        ))
    }
}

#[repr(C)]
#[derive(Debug)]
/// Response to [FramebufferRequest]
pub struct FramebufferResponse {
    /// The response revision number
    pub revision: u64,
    /// The number of [Framebuffer]s in `framebuffers`
    pub framebuffer_count: u64,
    /// A pointer to an array of [Framebuffer] pointers
    pub framebuffers: *mut *mut Framebuffer,
}

impl FramebufferResponse {
    /// Get the framebuffer slice
    /// # Safety
    /// The pointer must point to a valid array of [Framebuffer]s
    pub unsafe fn get_framebuffers(&self) -> Option<&[&Framebuffer]> {
        if self.framebuffers.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.framebuffers as *const &Framebuffer,
            self.framebuffer_count as usize,
        ))
    }
}

#[repr(C)]
#[derive(Debug)]
/// Response to [Level5PagingRequest]
pub struct Level5PagingResponse {
    /// The response revision number
    pub revision: u64,
}

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

#[repr(C)]
#[derive(Debug)]
/// Response to [MemoryMapRequest]
pub struct MemoryMapResponse {
    /// The response revision number
    pub revision: u64,
    /// The amount of entries in the memory map
    pub entry_count: u64,
    /// An array of [MemoryMapEntry] pointers
    pub entries: *const *const MemoryMapEntry,
}

impl MemoryMapResponse {
    /// Get the memory map entry slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [MemoryMapEntry]
    pub unsafe fn get_memory_map(&self) -> Option<&[&MemoryMapEntry]> {
        if self.entries.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.entries as *const &MemoryMapEntry,
            self.entry_count as usize,
        ))
    }
}

#[repr(C)]
#[derive(Debug)]
/// Response to [EntryPointRequest]
pub struct EntryPointResponse {
    /// The response revision number
    pub revision: u64,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [KernelFileRequest]
pub struct KernelFileResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// Pointer to the kernel file structure
    pub kernel_file: *const File<'a>,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [ModuleRequest]
pub struct ModuleResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// The number of modules
    pub module_count: u64,
    /// An array of [File] pointers
    pub modules: *const *const File<'a>,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [RSDPRequest]
pub struct RSDPResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the RSDP table
    pub address: Option<*const u8>,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [SMBIOSRequest]
pub struct SMBIOSResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the 32-bit SMBIOS entry point, null if not present
    pub entry_32: Option<*const u8>,
    /// Address of the 64-bit SMBIOS entry point, null if not present
    pub entry_64: Option<*const u8>,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [EfiSystemTableRequest]
pub struct EfiSystemTableResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the EFI System Table
    pub address: Option<*const u8>,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [BootTimeRequest]
pub struct BootTimeResponse {
    /// The response revision number
    pub revision: u64,
    /// The UNIX time on boot
    pub boot_time: i64,
}

#[repr(C)]
#[derive(Debug)]
/// Response to [KernelAddressRequest]
pub struct KernelAddressResponse {
    /// The response revision number
    pub revision: u64,
    /// The physical base of the kernel
    pub physical_base: u64,
    /// The virtual base of the kernel
    pub virtual_base: u64,
}
