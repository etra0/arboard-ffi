use arboard::Clipboard;

#[repr(u8)]
pub enum Status {
    Ok,
    Error
}

pub extern "C" fn clipboard_new(cb: *mut *mut Clipboard) -> Status {
    if let Ok(clipboard) = Clipboard::new().map(Box::new) {
        unsafe { *cb = Box::leak(clipboard) as _; }
        return Status::Ok;
    }

    return Status::Error;
}

// TODO: Search a way to avoid allocations.
pub extern "C" fn clipboard_set_text(cb: *mut Clipboard, text: *const char, length: usize) -> Status {
    let sl = unsafe { std::slice::from_raw_parts(text, length) };
    let inp = String::from_iter(sl.iter());
    unsafe { (*cb).set_text(inp).unwrap(); }
    return Status::Ok;
}

pub extern "C" fn clipboard_destroy(cb: *mut Clipboard) -> Status {
    unsafe { drop(Box::from_raw(cb)); }
    return Status::Ok;
}