use libc::{self, c_char};

#[derive(Debug)]
#[repr(C)]
struct CoolStruct {
    x: libc::uint16_t,
    y: libc::c_int
}

#[no_mangle]
extern "C" fn cool_add(v1: f32, v2: f32) -> f32 {
    println!("Adding in rust...");
    v1 + v2
}

#[no_mangle]
extern "C" fn cool_function(i: libc::c_int, c: c_char, cs: &mut CoolStruct, cs2: CoolStruct) {
    let c = c as u8;
    let c = c as char;
    println!("Hello from rust i: {}, c: {}, cs.x = {}, cs.y = {}", i, c, cs.x, cs.y);
    println!("cs2: {:?}", cs2);
    (*cs).x -= 1;
    (*cs).y -= 1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
