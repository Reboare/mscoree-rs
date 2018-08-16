#[macro_export]
macro_rules! STDAPI {
    (fn $func:ident($($id:ident : $tpe:ty,)*) -> $return_type:ty) => {
        extern "system" { pub fn $func(
            $($id: $tpe,)*
        ) -> $return_type;}
    };
}