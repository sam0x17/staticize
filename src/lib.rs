//! Staticize provides a [`Staticize`] trait which provides a handy associated type `Static`
//! which provides a `'static` version of `T` for all `T` that implement [`Staticize`].
//! [`Staticize`] is implemented on all primitives, as well as references, tuples up to size
//! 16, arrays, and slices of any `T` that implements [`Staticize`]. Implementations are also
//! provided for a variety of built-in types including but not limited to [`Option`],
//! [`Result`] and atomics.
//!
//! [![Crates.io](https://img.shields.io/crates/v/staticize)](https://crates.io/crates/staticize)
//! [![docs.rs](https://img.shields.io/docsrs/staticize?label=docs)](https://docs.rs/staticize/latest/staticize/)
//! [![MIT License](https://img.shields.io/github/license/sam0x17/staticize)](https://github.com/sam0x17/staticize/blob/main/LICENSE)
//!
//! ## Use Cases
//!
//! Staticize is useful for situations where you have a `T` but for whatever reason need a
//! `'static` version of it, such as when you are working with type-erased heap allocations, as
//! is the case with my [interned](https://crates.io/crates/interned) crate. Another common
//! use-case is situations where a static version of a type is needed for some generic method,
//! such as [`core::any::TypeId`].
//!
//! For example, a method that takes a value of type `T`, stores it in a static on the heap, and
//! returns a `'static` reference to the static version of the data on the heap, might use the
//! following signature:
//!
//! ```ignore
//! pub fn heap_allocate<T: Staticize>(val: T) -> T::Static {
//!   // ...
//! }
//! ```
//!
//! ## Features
//!
//! Two convenience methods, [`static_type_id`](`Staticize::static_type_id`) and
//! [`static_type_name`](`Staticize::static_type_id`) are also provided on [`Staticize`]. These
//! use the facilities in [`core::any`] to return the underlying
//! [`TypeId`](`core::any::TypeId`) and name (as a `&'static str`) of the _static_ version of
//! `T`.
//!
//! Staticize is completely `no_std`, so it can be used in exotic scenarios where the standard
//! library is not available, such as embedded devices or in WASM.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::{
    any::{type_name, TypeId},
    ops::{Bound, ControlFlow},
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
        AtomicU64, AtomicU8, AtomicUsize, Ordering,
    },
};

/// Provides a handy `Static` associated type which should resolve to a `'static` version of
/// `T` for all `T` that implement [`Staticize`].
pub trait Staticize {
    /// A `'static` version of `T`.
    type Static: 'static + ?Sized;

    /// Returns the [`TypeId`] of the `'static` version of `T`
    fn static_type_id() -> TypeId {
        TypeId::of::<Self::Static>()
    }

    /// Returns the type name of the `'static` version of `T`
    fn static_type_name() -> &'static str {
        &type_name::<Self::Static>()
    }
}

impl<'a, T: ?Sized> Staticize for &'a T
where
    T: Staticize,
{
    type Static = &'static T::Static;
}

impl<'a, T: Staticize> Staticize for &'a [T]
where
    <T as Staticize>::Static: Sized,
{
    type Static = &'static [T::Static];
}

impl<T: Staticize> Staticize for Option<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = Option<T::Static>;
}

impl<T: Staticize, E: Staticize> Staticize for Result<T, E>
where
    <T as Staticize>::Static: Sized,
    <E as Staticize>::Static: Sized,
{
    type Static = Result<T::Static, E::Static>;
}

impl<B: Staticize, C: Staticize> Staticize for ControlFlow<B, C>
where
    <B as Staticize>::Static: Sized,
    <C as Staticize>::Static: Sized,
{
    type Static = ControlFlow<B::Static, C::Static>;
}

impl<T: Staticize> Staticize for Bound<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = Bound<T::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<T: Staticize> Staticize for alloc::vec::Vec<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = alloc::vec::Vec<T::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<K: Staticize, V: Staticize> Staticize for alloc::collections::BTreeMap<K, V>
where
    <K as Staticize>::Static: Sized,
    <V as Staticize>::Static: Sized,
{
    type Static = alloc::collections::BTreeMap<K::Static, V::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<T: Staticize> Staticize for alloc::collections::BTreeSet<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = alloc::collections::BTreeSet<T::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<T: Staticize> Staticize for alloc::collections::BinaryHeap<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = alloc::collections::BinaryHeap<T::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<T: Staticize> Staticize for alloc::collections::LinkedList<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = alloc::collections::LinkedList<T::Static>;
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<T: Staticize> Staticize for alloc::collections::VecDeque<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = alloc::collections::VecDeque<T::Static>;
}

#[cfg(feature = "std")]
impl<T: Staticize> Staticize for std::vec::Vec<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = std::vec::Vec<T::Static>;
}

#[cfg(feature = "std")]
impl<K: Staticize, V: Staticize> Staticize for std::collections::BTreeMap<K, V>
where
    <K as Staticize>::Static: Sized,
    <V as Staticize>::Static: Sized,
{
    type Static = std::collections::BTreeMap<K::Static, V::Static>;
}

#[cfg(feature = "std")]
impl<T: Staticize> Staticize for std::collections::BTreeSet<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = std::collections::BTreeSet<T::Static>;
}

#[cfg(feature = "std")]
impl<T: Staticize> Staticize for std::collections::BinaryHeap<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = std::collections::BinaryHeap<T::Static>;
}

#[cfg(feature = "std")]
impl<T: Staticize> Staticize for std::collections::LinkedList<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = std::collections::LinkedList<T::Static>;
}

#[cfg(feature = "std")]
impl<T: Staticize> Staticize for std::collections::VecDeque<T>
where
    <T as Staticize>::Static: Sized,
{
    type Static = std::collections::VecDeque<T::Static>;
}

/// Used to implement [`Staticize`] for n-sized tuples.
///
/// For example, to add support for tuples of size 17, you would write:
///
/// ```ignore
/// derive_staticize_tuples!(A, B, C);
/// ```
///
/// Note though that this would compile-error because [`Staticize`] is already implemented for
/// tuple sizes up to 16. You can use the macro to implement it for tuples of larger sizes, if
/// needed. Just make sure to provide `N` unique letters separated by commas as input to the
/// macro.
#[macro_export]
macro_rules! derive_staticize_tuples {
    ($($ident:ident),*) => {
        impl<$($ident: Staticize),*> Staticize for ($($ident),*,)
        where $(<$ident as Staticize>::Static: Sized),*
        {
            type Static = ($($ident::Static),*,);
        }
    };
}

derive_staticize_tuples!(A);
derive_staticize_tuples!(A, B);
derive_staticize_tuples!(A, B, C);
derive_staticize_tuples!(A, B, C, D);
derive_staticize_tuples!(A, B, C, D, E);
derive_staticize_tuples!(A, B, C, D, E, F);
derive_staticize_tuples!(A, B, C, D, E, F, G);
derive_staticize_tuples!(A, B, C, D, E, F, G, H);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
derive_staticize_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

/// Implements [`Staticize`] for the specified type.
///
/// This will only work for `T: 'static`, and this macro only exists because we cannot do a
/// blanket impl for `T: 'static` because it would conflict with many of the other impls we
/// provide.
#[macro_export]
macro_rules! derive_staticize {
    ($typ:ty) => {
        impl $crate::Staticize for $typ {
            type Static = $typ;
        }
    };
}

derive_staticize!(str);
derive_staticize!(char);
derive_staticize!(bool);
derive_staticize!(usize);
derive_staticize!(isize);
derive_staticize!(u8);
derive_staticize!(u16);
derive_staticize!(u32);
derive_staticize!(u64);
derive_staticize!(u128);
derive_staticize!(i8);
derive_staticize!(i16);
derive_staticize!(i32);
derive_staticize!(i64);
derive_staticize!(i128);
derive_staticize!(f32);
derive_staticize!(f64);
derive_staticize!(());

derive_staticize!(Ordering);
derive_staticize!(AtomicBool);
derive_staticize!(AtomicU8);
derive_staticize!(AtomicU16);
derive_staticize!(AtomicU32);
derive_staticize!(AtomicU64);
derive_staticize!(AtomicI8);
derive_staticize!(AtomicI16);
derive_staticize!(AtomicI32);
derive_staticize!(AtomicI64);
derive_staticize!(AtomicIsize);
derive_staticize!(AtomicUsize);

#[cfg(all(feature = "alloc", not(feature = "std")))]
derive_staticize!(alloc::string::String);

#[cfg(feature = "std")]
derive_staticize!(String);
