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
derive_staticize!(bool);
derive_staticize!(usize);
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
derive_staticize!(char);
derive_staticize!(());
