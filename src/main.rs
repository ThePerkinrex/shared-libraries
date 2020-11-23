use libloading::{Library, Symbol};
use std::ffi::CString;

type AddFunc = unsafe fn(Drinck);

#[repr(C)]
struct Drinck {
    name: *mut i8, // char *
    age: u32,
}

fn main() {
    
    let lib1 = Library::new("./dylibs/lib_c.so").unwrap();
    let lib2 = Library::new("./dylibs/lib_rust.so").unwrap();
    let lib3 = Library::new("./dylibs/lib_v.so").unwrap();

    unsafe {
        let func: Symbol<AddFunc> = lib1.get(b"add").unwrap();

        func(Drinck {
            name: CString::new("Timmy").unwrap().into_raw(),
            age: 3,
        });
        // println!("add(1) = {}", answer);
    }

    unsafe {
        let func: Symbol<AddFunc> = lib2.get(b"add").unwrap();

        func(Drinck {
            name: CString::new("Timmy").unwrap().into_raw(),
            age: 3,
        });
        // println!("add(1) = {}", answer);
    }

    unsafe {
        let func: Symbol<AddFunc> = lib3.get(b"add").unwrap();

        func(Drinck {
            name: CString::new("Timmy").unwrap().into_raw(),
            age: 3,
        });
        // println!("add(1) = {}", answer);
    }
}
