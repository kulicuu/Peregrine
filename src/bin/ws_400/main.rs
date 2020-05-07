





fn main () {
    println!("ws-400");

    unsafe {
        let mut x = 5;
        let raw = &mut x as *mut i32;
        println!("raw: {:?}", *raw);
        *raw = 10;
        println!("now raw {:?}", *raw);
    }

}
