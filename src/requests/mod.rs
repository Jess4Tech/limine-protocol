use super::responses::*;
use crate::LimineRequest;
use crate::{impl_request, LimineRequestMarker};
use core::{cell::UnsafeCell, ptr::NonNull};

const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];

#[macro_export]
/// Implement `new` for structs with the proper ID values
macro_rules! impl_request {
    ($id1:expr, $id2:expr, $item:ident) => {
        impl $item {
            /// The ID of the request
            pub const ID: [u64; 4] = [COMMON_MAGIC[0], COMMON_MAGIC[1], $id1, $id2];

            /// Return a new instance of the item with the correct ID values
            pub fn new() -> $item {
                Self {
                    id: Self::ID,
                    ..Self::default()
                }
            }

            /// Convert the request into a [LimineRequest]
            pub const fn into_request(self) -> LimineRequest<Self> {
                LimineRequest(UnsafeCell::new(self))
            }
        }

        impl LimineRequestMarker for $item {}
    };
}

#[macro_export]
/// Implement `new` for structs with lifetimes with the proper ID values
macro_rules! impl_request_with_lifetime {
    ($id1:expr, $id2:expr, $item:ident) => {
        impl<'a> $item<'a> {
            /// The ID of the request
            pub const ID: [u64; 4] = [COMMON_MAGIC[0], COMMON_MAGIC[1], $id1, $id2];

            /// Return a new instance of the item with the correct ID values
            pub fn new() -> $item<'a> {
                Self {
                    id: [COMMON_MAGIC[0], COMMON_MAGIC[1], $id1, $id2],
                    ..Self::default()
                }
            }

            /// Convert the request into a [LimineRequest]
            pub const fn into_request(self) -> LimineRequest<Self> {
                LimineRequest(UnsafeCell::new(self))
            }
        }

        impl<'a> LimineRequestMarker for $item<'a> {}
    };
}

#[repr(C)]
#[derive(Debug, Default)]
/// Get information regarding the bootloader
pub struct InfoRequest<'a> {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<InfoResponse<'a>>>,
}

impl_request_with_lifetime!(0xf55038d8e2a1202f, 0x279426fcf5f59740, InfoRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Specify how much stack you desire the bootloader to give you
pub struct StackSizeRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<StackSizeResponse>>,
    /// The amount of stack to request
    pub stack_size: u64,
}

impl_request!(0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d, StackSizeRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request Higher Half Direct Mapping be enabled
pub struct HHDMRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<HHDMResponse>>,
}

impl_request!(0x48dcf1cb8ad2b852, 0x63984e959a98244b, HHDMRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request a terminal
pub struct TerminalRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<TerminalResponse>>,
}

impl_request!(0x0785a0aea5d0750f, 0x1c1936fee0d6cf6e, TerminalRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request a framebuffer
pub struct FramebufferRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<FramebufferResponse>>,
}

impl_request!(0xcbfe81d7dd2d1977, 0x063150319ebc9b71, FramebufferRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request Level 5 paging be enabled
pub struct Level5PagingRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<Level5PagingResponse>>,
}

impl_request!(0x94469551da9b3192, 0xebe5e86db7382888, Level5PagingRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the bootloader bootstrap the secondary processors
pub struct SMPRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<SMPResponse>>,
    /// Flags for the bootloader
    /// `Bit 0` - Enable X2APIC if available
    pub flags: u64,
}

impl_request!(0x95a67b819a1b857e, 0xa0b61b723b6a73e0, SMPRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request for receiving the Memory Map from the bootloader
pub struct MemoryMapRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<MemoryMapResponse>>,
}

impl_request!(0x67cf3d9d378a806f, 0xe304acdfc50c3c62, MemoryMapRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the bootloader use the specified entry point instead of the default one
pub struct EntryPointRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<EntryPointResponse>>,
    /// Function to the entry point
    pub entry: Option<*mut ()>,
}

impl_request!(0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a, EntryPointRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the file the Kernel was loaded from
pub struct KernelFileRequest<'a> {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<KernelFileResponse<'a>>>,
}

impl_request_with_lifetime!(0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69, KernelFileRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the loaded modules
pub struct ModuleRequest<'a> {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<ModuleResponse<'a>>>,
}

impl_request_with_lifetime!(0x3e7e279702be32af, 0xca1c4f3bd1280cee, ModuleRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the address of the RSDP
pub struct RSDPRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<RSDPResponse>>,
}

impl_request!(0xc5e77b6b397e7b43, 0x27637845accdcf3c, RSDPRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the SMBIOS entry point
pub struct SMBIOSRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<SMBIOSResponse>>,
}

impl_request!(0x9e9046f11e095391, 0xaa4a520fefbde5ee, SMBIOSRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the address of the EFI System Table
pub struct EfiSystemTableRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<EfiSystemTableResponse>>,
}

impl_request!(
    0x5ceba5163eaaf6d6,
    0x0a6981610cf65fcc,
    EfiSystemTableRequest
);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the time on boot
pub struct BootTimeRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<BootTimeResponse>>,
}

impl_request!(0x502746e184c088aa, 0xfbc5ec83e6327893, BootTimeRequest);

#[repr(C)]
#[derive(Debug, Default)]
/// Request the physical and virtual address of the Kernel
pub struct KernelAddressRequest {
    /// ID Array
    pub id: [u64; 4],
    /// Revision numbers
    pub revision: u64,
    /// Response pointer
    pub response: Option<NonNull<KernelAddressResponse>>,
}

impl_request!(0x71ba76863cc55f63, 0xb2644a48c516a487, KernelAddressRequest);
