use windows::Win32::Foundation::HWND;

// Thread-safe window handle wrapper
#[derive(Clone)]
pub struct ThreadSafeHwnd(pub HWND);
unsafe impl Send for ThreadSafeHwnd {}