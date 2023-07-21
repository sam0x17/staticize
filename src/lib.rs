//! Staticize provides a [`Staticize`] trait which provides a handy associated type `Static`
//! which provides a `'static` version of `T` for all `T` that implement [`Staticize`].
//! [`Staticize`] is implemented on all primitives, as well as references, tuples up to size
//! 16, arrays, and slices of any `T` that implements [`Staticize`].
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

#![no_std]

use core::any::{type_name, TypeId};

pub trait Staticize {
    type Static: 'static + ?Sized;

    fn static_type_id() -> TypeId {
        TypeId::of::<Self::Static>()
    }

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
