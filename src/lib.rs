//! A lightweight way to defer execution of a block to the end of the scope, and
//! to run code during an unwind.
//!
//! This crate provides the [`defer!`] macro and [`Defer`] RAII guard.
//!
//! This crate is extremely lightweight: it has no external dependencies or build
//! scripts, and <150 LOC.
//!
//! # Notes
//!
//! ## Drop Order
//! In Rust, local variables are always dropped in **reverse** declaration order.  
//! ie. the following program prints `012`:
//! ```rust
//! # use scopeguard_lite::defer;
//! defer! { print!("2"); }
//! defer! { print!("1"); }
//! print!("0");
//! ```
//! That is to say, the *first* scopeguard you define in a scope is the one
//! that runs *last*.
//!
//! # Rust Version
//!
//! The current MSRV is `1.61`. While unlikely, it may increase in a future minor
//! release.

#![no_std]
#![allow(unknown_lints)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unnameable_types)]
#![warn(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unsafe_op_in_unsafe_fn)]
#![forbid(non_ascii_idents)]

use core::any::type_name;
use core::mem::ManuallyDrop;
use core::{fmt, mem};

/// Defers execution of the enclosed code until the end of scope. The code will
/// run when the current scope ends, regardless of how that happens (panic, return, etc).
///
/// For more information, see the [crate docs](crate).
///
/// # Expansion
///
/// This macro expands to a single statement:
/// ```rust
/// let _guard = ::scopeguard_lite::Defer::new(|| { /* your code here */ });
/// ```
///
/// Macro hygiene prevents you from naming the `_guard` variable. If you need
/// to conditionally defuse the guard, create a [`Defer`] guard manually with
/// [`Defer::new`], assign it to a variable, then call [`.defuse()`](Defer::defuse)
/// on the code paths you do *not* want it to execute on.
///
/// # Examples
///
/// Drop a value, then deallocate it (even if the dtor panics!):
/// ```
/// # use scopeguard_lite::defer;
/// # use std::{ptr, alloc::{dealloc, Layout}};
/// unsafe fn destroy<T: ?Sized>(ptr: *mut T) {
///     # // FIXME(feature layout_for_ptr #69835): `Layout::for_value(&*ptr)` -> `Layout::for_value_raw(ptr)`
///     // avoid memory leaks by ensuring the backing storage is
///     // always deallocated, even if the destructor panics
///     defer! { dealloc(ptr.cast(), Layout::for_value(&*ptr)) }
///     ptr::drop_in_place(ptr);
/// }
/// ```
#[macro_export]
macro_rules! defer {
    ($($tt:tt)*) => {
        let _guard = $crate::Defer::new(|| { $($tt)* });
    };
}

/// Execute a closure on drop.
///
/// # Examples
///
/// This program prints `01`:
/// ```
/// # use scopeguard_lite::Defer;
/// fn foo() {
///     let guard = Defer::new(|| print!("1"));
///     print!("0");
/// } // <- guard dropped here
/// ```
#[repr(transparent)]
pub struct Defer<F: FnOnce()> {
    f: ManuallyDrop<F>,
}

impl<F: FnOnce()> Defer<F> {
    /// Creates a new guard that will executed the provided closure when it is
    /// dropped.
    #[inline]
    #[must_use = "the closure will execute immediately if unused"]
    pub const fn new(f: F) -> Self {
        Defer {
            f: ManuallyDrop::new(f),
        }
    }

    /// "Defuses" this scope guard, preventing the closure from running.
    ///
    /// For more information, see the [crate docs](crate).
    ///
    /// # Notes
    ///
    /// This will drop the closure (and all of its captures).
    ///
    /// # Examples
    ///
    /// ```
    /// # use scopeguard_lite::Defer;
    /// let guard = Defer::new(|| unreachable!("never executed"));
    /// guard.defuse();
    /// ```
    #[inline]
    pub fn defuse(mut self) {
        unsafe { ManuallyDrop::drop(&mut self.f) };
        mem::forget(self);
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    #[inline]
    fn drop(&mut self) {
        let f = unsafe { ManuallyDrop::take(&mut self.f) };
        f();
    }
}

// perhaps debuggers or REPLs can take advantage of this
impl<F: FnOnce()> fmt::Debug for Defer<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(type_name::<Self>())
    }
}
