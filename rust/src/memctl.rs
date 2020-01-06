
#[repr(C,packed)]
struct DevRequest(u8,u16);

pub macro_rules! add_dev{
    ($x: ident $devno: literal $props: literal) =>{
        extern"C"{
        #[no_mangle]
        #[link_section = ".note.snesdev.memctl.devs"]
         static __dev_$x : DevRequest = DevRequest($devno,$props)
        }
    }
}


