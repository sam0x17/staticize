use std::any::TypeId;

pub fn static_type_id<T: Staticize>() -> TypeId {
    TypeId::of::<T::Static>()
}

pub fn static_type_name<T: Staticize>() -> &'static str {
    &std::any::type_name::<T::Static>()
}

pub trait Staticize {
    type Static: 'static + ?Sized;
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
