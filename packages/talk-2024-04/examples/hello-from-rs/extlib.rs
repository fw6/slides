#[repr(C)]
pub struct User {
    pub name: String,
    age: u8,
}

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
