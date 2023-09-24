crate::number_types!(
    i8 => byte,
    i16 => short,
    i32 => int,
    i64 => long,

    u8 => u_byte,
    u16 => u_short,
    u32 => u_int,
    u64 => u_long,

    f32 => float,
    f64 => double
);

#[macro_export]
macro_rules! number_types {
    ($($t:ty=>$i:ident),*) => {
        $(
            #[allow(non_camel_case_types)]
            pub type $i = $t;
        )*
    };
}