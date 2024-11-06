use crate::window::Window;

#[derive(Default)]
pub struct App {
    main: Box<Window>,
}

impl App {
    pub fn new() -> Self {
        Self {
            main: Window::new("win", 800, 600, Box::<App>::new_uninit()).unwrap()
        }
    }
}
