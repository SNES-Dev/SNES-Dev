
#[repr(C,packed)]
pub struct DevRequest(u8,u16);

#[repr(C,packed)]
pub struct LinkMap<S>(u8,S);


#[macro_export]
macro_rules! devs{
    {$(dev $name:ident as $n:literal with $mode:literal;)*} =>{
        $(
            #[link_section=".note.snesdev.memctl.dev"]
            #[no_mangle]
            pub extern static mut $name: $crate::memctl::DevRequest = $crate::memctl::DevRequest($n,$mode);
        )*
    }
}


