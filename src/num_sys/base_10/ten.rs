pub trait Ten {
    const TEN: Self;
}

macro_rules! impl_trait {
    ($trait_name:ident for @PRIM_INTS) => {
        impl_trait!($trait_name for [u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize]);
    };
    ($trait_name:ident for [$($t:ty),+]) => {
        $(
            impl $trait_name for $t {
                const TEN: Self = 10;
            }
        )+
    }
}

impl_trait!(Ten for @PRIM_INTS);
