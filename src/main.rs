#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
};

pub struct Window {
    hwnd: HWND,
    app: Weak<RefCell<App>>,  // Changed from Option<Rc> to Weak
    textbox: HWND,
}

#[derive(Clone)]
pub struct App {
    window: Option<Rc<RefCell<Window>>>,  // Keep as Option<Rc> since App owns Window
}

impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(App { window: None }))
    }

    pub fn display_words(&self) {
        if let Some(window) = &self.window {
            let window = window.borrow();
            unsafe {
                let _= SetWindowTextW(
                    window.textbox,
                    w!("Button clicked! Hello from App!")
                );
            }
        }
    }

    pub fn run(app: Rc<RefCell<Self>>) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            
            let window_class = s!("window");
            let wc = WNDCLASSA {
                lpfnWndProc: Some(Self::wndproc),
                hInstance: instance.into(),
                lpszClassName: window_class,
                ..Default::default()
            };

            RegisterClassA(&wc);

            let window = Window::new(instance.into(), window_class, Rc::downgrade(&app))?;  // Use downgrade to create Weak reference
            app.borrow_mut().window = Some(Rc::new(RefCell::new(window)));

            let mut message = MSG::default();
            while GetMessageA(&mut message, None, 0, 0).into() {
                let _ = TranslateMessage(&message);
                DispatchMessageA(&message);
            }

            Ok(())
        }
    }

    extern "system" fn wndproc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_COMMAND => {
                    if wparam.0 == 1 {  // Button ID
                        let window_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Window;
                        (*window_ptr).hwnd = hwnd;
                        // Try to upgrade the Weak reference
                        if let Some(app) = (*window_ptr).app.upgrade() {
                            app.borrow().display_words();
                        }
                    }
                    LRESULT(0)
                },
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                },
                _ => DefWindowProcA(hwnd, message, wparam, lparam),
            }
        }
    }
}

impl Window {
    pub fn new(
        instance: HINSTANCE,
        window_class: PCSTR,
        app: Weak<RefCell<App>>,  // Changed parameter type to Weak
    ) -> Result<Self> {
        unsafe {
            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("Rust Windows App"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                300,
                200,
                None,
                None,
                instance,
                None,
            ).unwrap();

            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                s!("BUTTON"),
                s!("Click Me"),
                WINDOW_STYLE(
                    WS_TABSTOP.0 | 
                    WS_VISIBLE.0 | 
                    WS_CHILD.0 |
                    BS_DEFPUSHBUTTON as u32
                ),
                10,
                10,
                100,
                30,
                hwnd,
                HMENU(1 as _),  // Button ID = 1
                instance,
                None,
            ).unwrap();

            let textbox = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                s!("EDIT"),
                s!(""),
                WINDOW_STYLE(
                    WS_CHILD.0 |
                    WS_VISIBLE.0 |
                    WS_BORDER.0 | 
                    ES_LEFT as u32
                ),
                10,
                50,
                260,
                30,
                hwnd,
                HMENU(2 as _),  // TextBox ID = 2
                instance,
                None,
            ).unwrap();

            let window = Box::new(Window {
                hwnd,
                app: app.clone(),
                textbox,
            });

            SetWindowLongPtrA(
                hwnd,
                GWLP_USERDATA,
                Box::into_raw(window) as isize,
            );

            Ok(Window {
                hwnd,
                app,
                textbox,
            })
        }
    }
}

fn main() -> Result<()> {
    let app = App::new();
    App::run(app)
}