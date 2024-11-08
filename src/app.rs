use std::{
    cell::RefCell,
    io::Error,
    rc::Rc, 
    sync::mpsc::{channel, Sender},
    thread
};
use std::sync::{Arc, Mutex};

use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW
};

use crate::window::Window;
use crate::thread_safe::ThreadSafeHwnd;

#[derive(Clone, Default)]
pub struct App {
    // Channel sender for thread communication
    tx: Option<Sender<()>>,  
    data: Arc<Mutex<u32>>,
    // (curr, max, msg)
    progress: Arc<Mutex<(u8, u8, String)>>,
}

impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(App {
                tx: None,
                data: Arc::new(Mutex::new(0)),
                ..Default::default()
            }))
    }

    pub fn display_words(&self) {
        // Send message to worker thread
        if let Some(tx) = &self.tx {
            let _ = tx.send(());
        }
    }

    pub fn run_progress_bar(&self, hwnd: ThreadSafeHwnd) {
        let pb_data = self.progress.clone();
        thread::spawn(move || {
            for i in 0..30 {
                thread::sleep(std::time::Duration::from_millis(400));
                let mut pb_data = pb_data.lock().unwrap();
                let msg = format!("進度{}/30", (i+1).to_string());
                *pb_data = (i+1, 30, msg);
                // Post message back to main window
                let hwnd = hwnd.clone().0;
                unsafe {
                    let _ = PostMessageW(
                        hwnd,
                        Window::WM_UPDATE_TEXT,
                        WPARAM(0),
                        LPARAM(0)
                    );
                }
            }
        });
    }

    pub fn get_progress(&self) -> (u8, u8, String) {
        (*self.progress.lock().unwrap()).clone()
    }

    pub fn get_data(&self) -> u32 {
        *self.data.lock().unwrap()
    }

    pub fn setup_worker_thread(&mut self, hwnd: ThreadSafeHwnd) {
        let (tx, rx) = channel();
        let data = self.data.clone();
        thread::spawn(move || {
            for _ in rx {
                // Simulate some work
                thread::sleep(std::time::Duration::from_secs(1));
                let mut data = data.lock().unwrap();
                *data += 1;
                // Post message back to main window
                let hwnd = hwnd.clone().0;
                unsafe {
                    let _ = PostMessageW(
                        hwnd,
                        Window::WM_UPDATE_TEXT,
                        WPARAM(0),
                        LPARAM(0)
                    );
                }
            }
        });

        self.tx = Some(tx);
    }

    pub fn run(app: Rc<RefCell<Self>>) -> Result<(), Error> {
        Window::new(
            "windows-app",
            800, 
            600,
            Rc::downgrade(&app)
        )?;

        Ok(())
    }
}
