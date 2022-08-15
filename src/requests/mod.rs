/// Common Magic for Limine requests
pub const COMMON_MAGIC: [u64; 2] = [0xc7b1_dd30_df4c_8b88, 0x0a82_e883_a194_f07b];

/// This creates a limine request with const defaults by black magic.
/// It also automatically creates the `id` and `revision` fields.
/// # Example
/// ```rust
/// use limine_protocol::limine_request;
/// use core::ptr::NonNull;
///
/// pub struct TestResponse {
///     /// The response revision number
///     pub revision: u64,
///     /// A test number from the bootloader (not actually)
///     pub number: u64,
/// }
///
/// limine_request! {
///     /// A test limine request, being public isn't required    
///     pub struct TestRequest: [0xdead, 0xbeef] {
///         /// The response
///         pub response: Option<NonNull<TestResponse>>,
///     }
/// }
/// ```
/// Will be expanded to
/// ```rust
/// use core::ptr::NonNull;
/// pub struct TestResponse {
///     /// The response revision number
///     pub revision: u64,
///     /// A test number from the bootloader (not actually)
///     pub number: u64,
/// }
///
/// pub struct TestRequest {
///     /// The request id array
///     pub id: [u64; 4],
///     /// The request revision
///     pub revision: u64,
///     /// The response
///     pub response: Option<NonNull<TestResponse>>,
/// }
///
/// impl TestRequest {
///     /// Create a new instance of this request
///     pub const fn new() -> Self {
///         use limine_protocol::ConstDefault;
///         Self {
///             id: [
///                 limine_protocol::COMMON_MAGIC[0],
///                 limine_protocol::COMMON_MAGIC[1],
///                 0xdead,
///                 0xbeef,
///             ],
///             revision: 0,
///             response: None,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! limine_request {
    (
        $(#[$outer_meta:meta])*
        $vis:vis struct $req:ident: [$val1:expr, $val2:expr] {
            $(
                $(#[$inner_meta:meta])*
                $vis_f:vis $ident_f:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$outer_meta])*
        $vis struct $req {
            /// The request id array
            pub id: [u64; 4],
            /// The request revision
            pub revision: u64,
            $(
                $(#[$inner_meta])*
                $vis_f $ident_f: $ty,
            )*
        }

        impl $req {
            /// Create a new instance of this request
            #[must_use]
            pub const fn new() -> Self {
                use $crate::ConstDefault;
                Self {
                    id: [
                        $crate::COMMON_MAGIC[0],
                        $crate::COMMON_MAGIC[1],
                        $val1,
                        $val2
                    ],
                    revision: 0,
                    $(
                        $ident_f: <$ty>::DEFAULT,
                    )*
                }
            }

            /// Alias to `Self::new`
            #[must_use]
            pub const fn default() -> Self {
                Self::new()
            }

            /// Wrap the item in [Request]
            #[must_use]
            pub const fn into(self) -> $crate::Request<Self> {
                $crate::Request::new(self)
            }
        }
    };
}

mod boot_time;
pub use boot_time::*;

mod efi_sys_table;
pub use efi_sys_table::*;

mod entry_point;
pub use entry_point::*;

mod framebuffer;
pub use framebuffer::*;

mod hhdm;
pub use hhdm::*;

mod bootloader_info;
pub use bootloader_info::*;

mod kernel_address;
pub use kernel_address::*;

mod kernel_file;
pub use kernel_file::*;

mod level_5_paging;
pub use level_5_paging::*;

mod memory_map;
pub use memory_map::*;

mod module;
pub use module::*;

mod rsdp;
pub use rsdp::*;

mod smbios;
pub use smbios::*;

mod smp;
pub use smp::*;

mod stack_size;
pub use stack_size::*;

mod terminal;
pub use terminal::*;
