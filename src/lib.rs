//! Crate containing Rust representations for the Limine Boot Protocol
//!
//! # Example
//! If one wanted to have 64KiB of stack space, they would make a request like such
//! ```rust
//! // Note that `#[used]` may be required in case Rust decides the object is unreachable or unused
//! #[used]
//! static STACK_REQUEST: Request<StackSizeRequest> = StackSizeRequest {
//!    stack_size: 1024 * 64,
//!    ..StackSizeRequest::default()
//! }
//! .into();
//! ```
//!
//! One could also decide to link the request into the `limine_reqs` section, which would be done like such
//! ```rust
//! pub struct NotSync<T>(pub T);
//!
//! impl<T> core::ops::Deref for NotSync<T> {
//!     type Target = T;
//!  
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! unsafe impl<T> Sync for NotSync<T> {}
//!
//! // Note that `#[used]` may be required in case Rust decides the object is unreachable or unused
//! #[used]
//! static ARR: [NotSync<*mut ()>; 1] = [NotSync(&STACK_REQUEST as *const _ as *mut ())];
//! ```
//! Note that this is very unsafe, as is expected by such a thing as a Bootloader. Rust has no clue we're doing this at all.
#![no_std]
#![deny(missing_docs)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(clippy::module_name_repetitions)]

use core::{cell::UnsafeCell, fmt::Debug, ops::Deref};

#[repr(transparent)]
/// A Request type, which wraps the internal request in an unsafe cell,
/// due to the possibility it may be mutated by things outside rust.
/// However, it automatically derefences to the internal type for ergonomics.
pub struct Request<T>(UnsafeCell<T>);

impl<T> Request<T> {
    /// Create a new request.
    /// This should not be used outside of the `limine-protocol` crate.
    pub(crate) const fn new(req: T) -> Self {
        Self(UnsafeCell::new(req))
    }
}

impl<T> Deref for Request<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.get() }
    }
}

impl<T: Debug> Debug for Request<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as Debug>::fmt(unsafe { &*self.0.get() }, f)
    }
}

unsafe impl<T> Sync for Request<T> {}

/// Responses
mod responses;
pub use responses::*;

/// Requests
mod requests;
pub use requests::*;

mod default_const;
pub use default_const::ConstDefault;

/// Structures returned by the responses
pub mod structures;

#[cfg(test)]
mod tests {
    use crate::{
        responses::{FramebufferResponse, SMPResponse, TerminalResponse},
        structures::{framebuffer::Framebuffer, smpinfo::SMPInfo, terminal::Terminal},
    };

    extern "C" fn fake_write(_: *mut Terminal, _: *const u8, _: u64) {}

    #[test]
    fn terminal_get_slice() {
        let mut framebuffer = Framebuffer::new_empty();
        let tarray: [&Terminal; 1] = [&Terminal {
            columns: 120,
            rows: 80,
            framebuffer: &mut framebuffer,
        }];
        let terminal = TerminalResponse {
            revision: 1,
            terminal_count: 1,
            terminals: tarray.as_ptr() as *const *const Terminal,
            write: fake_write,
        };
        let tarray_from_terminal = unsafe { terminal.get_terminals() }.unwrap();
        assert_eq!(tarray.as_slice(), tarray_from_terminal);
    }

    #[test]
    fn terminal_none_when_null() {
        let terminal = TerminalResponse {
            revision: 1,
            terminal_count: 0,
            terminals: core::ptr::null(),
            write: fake_write,
        };
        assert!(unsafe { terminal.get_terminals() }.is_none());
    }

    #[test]
    fn framebuffer_get_slice() {
        let mut framebuffer_a = [&Framebuffer::new_empty(), &Framebuffer::new_empty()];
        let framebuffer = FramebufferResponse {
            revision: 1,
            framebuffer_count: 2,
            framebuffers: framebuffer_a.as_mut_ptr() as *mut *mut Framebuffer,
        };
        let fa_from_resp = unsafe { framebuffer.get_framebuffers() }.unwrap();
        assert_eq!(framebuffer_a, fa_from_resp);
    }

    #[test]
    fn framebuffer_none_when_null() {
        let framebuffer = FramebufferResponse {
            revision: 1,
            framebuffer_count: 2,
            framebuffers: core::ptr::null_mut(),
        };
        assert!(unsafe { framebuffer.get_framebuffers() }.is_none());
    }

    #[test]
    fn smpinfo_get_slice() {
        let c0 = SMPInfo::new_empty();
        let mut c1 = SMPInfo::new_empty();
        c1.processor_id = 1;
        c1.lapic_id = 1;
        let smps = [&c0, &c1];
        let resp = SMPResponse {
            revision: 1,
            flags: 0,
            bsp_lapic_id: 0,
            cpu_count: 2,
            cpus: smps.as_ptr() as *const *const SMPInfo,
        };
        let from_resp = unsafe { resp.get_cpu_info() }.unwrap();
        assert_eq!(smps, from_resp);
    }

    #[test]
    fn smpinfo_none_when_null() {
        let resp = SMPResponse {
            revision: 1,
            flags: 0,
            bsp_lapic_id: 0,
            cpu_count: 2,
            cpus: core::ptr::null(),
        };
        assert!(unsafe { resp.get_cpu_info() }.is_none())
    }
}
