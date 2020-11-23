use std::ffi::CString;

#[repr(C)]
struct Drinck {
    name: *mut i8, // char *
    age: u32,
}

#[no_mangle]
extern "C" fn add(a: Drinck) {
	println!("{}, rusty alcoholic (Age {})", unsafe {CString::from_raw(a.name)}.into_string().unwrap(), a.age)
}