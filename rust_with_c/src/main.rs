use libc;

#[repr(C)]
#[derive(Debug)]
struct CoolStruct {
    x: libc::uint16_t,
    y: libc::c_int
}

#[link(name = "cool")]
extern "C" {
    fn cool_add(v1: libc::c_float, v2: libc::c_float) -> libc::c_float;
    fn cool_function(i: libc::c_int, c: libc::c_char, cs: *mut CoolStruct);
}


fn main() {
    unsafe {
        let res = cool_add(10.9, 11.1);
        println!("Results {}", res);
        
        let mut cs = CoolStruct { x: 20, y: -10 };

        cool_function(33, 'c' as libc::c_char, &mut cs);
        println!("From rust: {:?}", cs);
    };
}