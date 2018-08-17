#[macro_export]
macro_rules! STDAPI {
    (fn $func:ident($($id:ident : $tpe:ty,)*) -> $return_type:ty) => {
        extern "system" { pub fn $func(
            $($id: $tpe,)*
        ) -> $return_type;}
    };
}
//pub type FLockClrVersionCallback = extern fn() -> HRESULT;
#[macro_export]
macro_rules! FUNC_PTR {
    ($name:ident($($p_name:ident : $p_ty:ty),*) -> $res:ty ) => {
        pub type $name = extern fn($($p_ty),*) -> $res;
    };
}