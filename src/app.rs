use wasm_bindgen::prelude::*;
//use workflow_log::log_trace;
use workflow_wasm::listener::Listener;
//use workflow_dom::utils::window;
use nw_sys::{prelude::*, utils, result::Result};
use web_sys::MouseEvent;
use std::sync::Arc;


//pub type Callback<T> = dyn FnMut(T) -> std::result::Result<(), JsValue>;

static mut APP:Option<Arc<App>> = None;

pub fn app()->Option<Arc<App>>{
    unsafe{APP.clone()}
}

#[derive(Clone)]
pub struct App{
    pub win_listeners: Arc<Mutex<Vec<Listener<nw::Window>>>>,
    pub menu_listeners: Arc<Mutex<Vec<Listener<JsValue>>>>,
    pub listeners: Arc<Mutex<Vec<Listener<MouseEvent>>>>
}

impl App{
    pub fn new()->Result<Arc<Self>>{
        let app = Arc::new(Self{
            win_listeners: Arc::new(Mutex::new(vec![])),
            menu_listeners: Arc::new(Mutex::new(vec![])),
            listeners: Arc::new(Mutex::new(vec![]))
        });

        unsafe{
            APP = Some(app.clone());
        };

        Ok(app)
    }

    pub fn push_window_listener(&self, listener:Listener<nw::Window>)->Result<()>{
        self.win_listeners.lock()?.push(listener);
        Ok(())
    }

    pub fn push_menu_listener(&self, listener:Listener<JsValue>)->Result<()>{
        self.menu_listeners.lock()?.push(listener);
        Ok(())
    }

    pub fn push_listener(&self, listener:Listener<MouseEvent>)->Result<()>{
        self.listeners.lock()?.push(listener);
        Ok(())
    }

    pub fn create_window_with_callback<F>(
        &self,
        url:&str,
        option:&nw::window::Options,
        callback:F
    )->Result<()>
    where
        F:FnMut(nw::Window) -> std::result::Result<(), JsValue> + 'static
    {
        let listener = Listener::new(callback);
    
        nw::Window::open_with_options_and_callback(
            url,
            option,
            listener.into_js()
        );

        self.push_window_listener(listener)?;
        Ok(())
    }

    pub fn create_window(url:&str, option:&nw::window::Options)->Result<()>{
        nw::Window::open_with_options(url, option);

        Ok(())
    }

    pub fn create_context_menu(&self, menus: Vec<nw::MenuItem>)->Result<()>{
        let popup_menu = nw::Menu::new();
        for menu_item in menus{
            popup_menu.append(&menu_item);
        }

        self.on_context_menu(move |ev:web_sys::MouseEvent|->std::result::Result<(), JsValue>{
            ev.prevent_default();
            popup_menu.popup(ev.x(), ev.y());
            Ok(())
        })?;

        Ok(())
    }

    pub fn on_context_menu<F>(&self, callback:F)->Result<()>
    where
        F:FnMut(MouseEvent) -> std::result::Result<(), JsValue> + 'static
    {
        let win = nw::Window::get();
        let dom_win = win.window();
        let body = utils::body(Some(dom_win));

        let listener = Listener::new(callback);
        body.add_event_listener_with_callback("contextmenu", listener.into_js())?;
        self.push_listener(listener)?;

        Ok(())
    }
    
} 




