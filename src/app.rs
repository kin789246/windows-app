use std::{
    io::Error,
    thread,
    mem,
};
use std::sync::{Arc, Mutex};

use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
    System::DataExchange::*,
};

use crate::window::Window;
use crate::thread_safe::ThreadSafeHwnd;

#[derive(Clone, Default)]
pub struct App {
    // Channel sender for thread communication
    window: Option<ThreadSafeHwnd>,
}

impl App {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn init_app(&mut self, hwnd: HWND) {
        self.window = Some(ThreadSafeHwnd(hwnd));
    }

    pub fn run_progress_bar(&self, app: Arc<Mutex<App>>) {
        thread::spawn(move || {
            let app = app.lock().unwrap();
            let mut progress: (usize, usize, String);
            app.post_message(Window::CTRL_EN_DIS, false);
            for i in 0..30 {
                thread::sleep(std::time::Duration::from_millis(300));
                let msg = format!("進度{}/30", (i+1).to_string());
                progress = (i+1, 30, msg);
                app.post_message(Window::APP_UPDATE_PROGRESS, progress);
                let msg = format!("append line {} to results", (i+1).to_string());
                app.post_message(Window::APP_UPDATE_RESULT, msg);
            }
            app.post_message(Window::CTRL_EN_DIS, true);
        });
    }

    pub fn run(app: Self) -> Result<(), Error> {
        Window::new(
            "windows-app",
            800, 
            600,
            app
        )?;

        Ok(())
    }

    fn post_message<T>(&self, msg: u32, data: T) {
        unsafe {
            let mut data = Box::new(data);
            let cds = COPYDATASTRUCT {
                dwData: 1, // Custom identifier
                cbData: mem::size_of::<T>() as u32,
                lpData: data.as_mut() as *mut _ as *mut core::ffi::c_void,
            };

            // Send with WM_USER
            if let Some(hwnd) = self.window.clone() {
                let hwnd = hwnd.0;
                // let _ = PostMessageW(hwnd, msg, WPARAM(0), LPARAM(0));
                let _ = SendMessageW(
                    hwnd,
                    msg,
                    WPARAM(&cds as *const _ as usize),
                    LPARAM(0)
                );
            }
        }
    }
}
