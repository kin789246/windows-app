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

#[derive(Clone)]
pub struct App {
    // Channel sender for thread communication
    tx: Option<Sender<()>>,  
    data: Arc<Mutex<u32>>,
}

impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(App {
                tx: None,
                data: Arc::new(Mutex::new(0))
            }))
    }

    pub fn display_words(&self) {
        // Send message to worker thread
        if let Some(tx) = &self.tx {
            let _ = tx.send(());
        }
    }

    pub fn get_data(&self) -> u32 {
        *self.data.lock().unwrap()
    }

    pub fn set_tx(&mut self, tx: Option<Sender<()>>) {
        self.tx = tx;
    }

    pub fn setup_worker_thread(&self, hwnd: ThreadSafeHwnd) -> Sender<()> {
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

        tx
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
