// SPDX-License-Identifier: Apache-2.0 OR MIT
// SPDX-FileCopyrightText: 2017-2020 The array-init developers
//
// MaybeUninit leaks memory if an error occurs during initialization, but we learned from the
// following how to use Guard to prevent leaks if necessary.
//
// https://github.com/Manishearth/array-init/blob/master/src/lib.rs
use core::mem::{needs_drop, MaybeUninit};
use core::ptr;
use core::slice;

/// # Why need this?
/// Since initializing hundreds of fixed-length arrays first would be wasteful
/// and costly, they are written sequentially in an uninitialized state and treated
/// as initialized when all writes are completed.
pub struct LazyInitArray<T, const N: usize> {
    array: MaybeUninit<[T; N]>,
    base_ptr: Option<*mut T>,
    /// Point to the next write position after `write_next` executed in the loop.
    current_ptr: Option<*mut T>,
    initialized_count: usize,

    /// If only a stack is used, it goes into individual processing.
    ///
    /// The implementation differentiates two cases:
    ///   A) `T` does not need to be dropped. Even if the initializer panics
    ///      or returns `Err` we will not leak memory.
    ///   B) `T` needs to be dropped. We must keep track of which elements have
    ///      been initialized so far, and drop them if we encounter a panic or `Err` midway.
    needs_drop: bool,
}

impl<T, const N: usize> LazyInitArray<T, N> {
    /// Creates a new `LazyInitArray`
    pub const fn new() -> Self {
        // NOTE:
        // Cannot get array pointer here!
        // Even if we reference the array you created internally, it will not be a pointer to self.array.
        Self {
            base_ptr: None,
            current_ptr: None,
            array: MaybeUninit::uninit(),
            needs_drop: needs_drop::<T>(),
            initialized_count: 0,
        }
    }

    /// To access the stack's self-referential structure with a raw pointer, the address must be set externally after the stack array is created.
    ///
    /// This needs to be called only once immediately after the `new`. (i.e., it must point to an address in the same stack frame).
    #[inline]
    pub fn set_stack_ptr(&mut self) {
        self.base_ptr = Some(self.array.as_mut_ptr() as *mut T);
        self.current_ptr = Some(self.array.as_mut_ptr() as *mut T);
    }

    /// Initialize one element and advance next ptr.
    ///
    /// # Panics
    /// If Incorrect this API usage(then memory leaks).
    /// - N < initialized_count
    /// - forgot to call `self.set_stack_ptr` once
    pub fn write_next(&mut self, value: T) {
        debug_assert!(
            self.initialized_count < N,
            "Should be initialized_count < {N}",
        );
        if let Some(current_ptr) = self.current_ptr {
            unsafe {
                current_ptr.write(value); // 0..N
                self.current_ptr = Some(current_ptr.add(1));
            }
            self.initialized_count += 1;
        } else {
            panic!("It is necessary to call `set_stack_ptr` once before writing to find the point on the stack.")
        }
    }

    /// Try to initialize array.
    ///
    /// # Errors
    /// Returns a guard that drops initialized data along the way to avoid memory leaks.
    ///
    /// # Panics
    /// If Incorrect this API usage(then memory leaks).
    /// - N < initialized_count
    /// - forgot to call `self.set_stack_ptr` once
    pub fn try_init(self) -> Result<[T; N], UnsafeDropSliceGuard<T>> {
        if N == self.initialized_count {
            unsafe { Ok(self.array.assume_init()) }
        } else {
            debug_assert!(
                self.initialized_count < N,
                "Should be initialized_count < {N}"
            );

            if let Some(base_ptr) = self.base_ptr {
                // Since MaybeUninit does not originally have drop, it delegates the dropping of initialized data to
                // a structure that implements drop.
                Err(UnsafeDropSliceGuard {
                    base_ptr,
                    initialized_count: self.initialized_count,
                    needs_drop: self.needs_drop,
                })
            } else {
                panic!("It is necessary to call `set_stack_ptr` once before writing to find the point on the stack.")
            }
        }
    }
}

/// Used only when needs_drop is true (when using heap)
///
/// # Safety
///
///   - `base_ptr[.. initialized_count]` must be a slice of init elements...
///
///   - ... that must be sound to `ptr::drop_in_place` if/when
///     `UnsafeDropSliceGuard` is dropped: "symbolic ownership"
pub struct UnsafeDropSliceGuard<Item> {
    /// started address of the slice.
    base_ptr: *mut Item,
    /// Wrote value count
    pub initialized_count: usize,
    needs_drop: bool,
}

impl<Item> Drop for UnsafeDropSliceGuard<Item> {
    fn drop(self: &'_ mut Self) {
        if self.needs_drop {
            unsafe {
                // # Safety
                //
                //   - the contract of the struct guarantees that this is sound
                ptr::drop_in_place(slice::from_raw_parts_mut(
                    self.base_ptr,
                    self.initialized_count,
                ));
            }
        }
    }
}
