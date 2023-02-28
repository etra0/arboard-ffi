use arboard::Clipboard;

#[repr(u8)]
pub enum Status {
    Ok,
    Error
}

#[no_mangle]
pub extern "C" fn clipboard_new(cb: *mut *mut Clipboard) -> Status {
    if let Ok(clipboard) = Clipboard::new().map(Box::new) {
        unsafe { *cb = Box::leak(clipboard) as _; }
        return Status::Ok;
    }

    return Status::Error;
}

// TODO: Search a way to avoid allocations.
#[no_mangle]
pub extern "C" fn clipboard_set_text(cb: *mut Clipboard, text: *const char, length: usize) -> Status {
    let sl = unsafe { std::slice::from_raw_parts(text as *const u8, length) };
    let inp = String::from_utf8_lossy(sl);
    println!("Will set it to {inp}");
    unsafe { (*cb).set_text(inp).unwrap(); }
    return Status::Ok;
}

#[no_mangle]
pub extern "C" fn clipboard_destroy(cb: *mut Clipboard) -> Status {
    unsafe { drop(Box::from_raw(cb)); }
    return Status::Ok;
}
