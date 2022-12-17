use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
//use workflow_log::log_trace;
pub use workflow_wasm::listener::{Listener, Callback, CallbackWithouResult};
use nw_sys::{prelude::*, utils, result::Result};
use web_sys::{MouseEvent, MediaStream, MediaStreamTrack};
use std::sync::Arc;
use crate::media::MediaStreamTrackKind;

static mut APP:Option<Arc<App>> = None;

pub fn app()->Option<Arc<App>>{
    unsafe{APP.clone()}
}

#[derive(Clone)]
pub struct App{
    pub win_listeners: Arc<Mutex<Vec<Listener<Callback<nw::Window>>>>>,
    pub js_value_listeners: Arc<Mutex<Vec<Listener<Callback<JsValue>>>>>,
    pub js_value_listeners_without_result: Arc<Mutex<Vec<Listener<CallbackWithouResult<JsValue>>>>>,
    pub mouse_listeners: Arc<Mutex<Vec<Listener<Callback<MouseEvent>>>>>,
    pub media_stream: Arc<Mutex<Option<MediaStream>>>
}

impl App{
    pub fn new()->Result<Arc<Self>>{
        let app = Arc::new(Self{
            win_listeners: Arc::new(Mutex::new(vec![])),
            js_value_listeners: Arc::new(Mutex::new(vec![])),
            js_value_listeners_without_result: Arc::new(Mutex::new(vec![])),
            mouse_listeners: Arc::new(Mutex::new(vec![])),
            media_stream: Arc::new(Mutex::new(None))
        });

        unsafe{
            APP = Some(app.clone());
        };

        Ok(app)
    }

    pub fn set_media_stream(&self, media_stream:Option<MediaStream>)->Result<()>{
        *self.media_stream.lock()? = media_stream;
        Ok(())
    }

    pub fn get_media_stream(&self)->Result<Option<MediaStream>>{
        let media_stream = self.media_stream.lock()?.clone();
        Ok(media_stream)
    }

    pub fn stop_media_stream(
        &self,
        track_kind:Option<MediaStreamTrackKind>,
        mut stream: Option<MediaStream>
    )->Result<()>{
        if stream.is_none(){
            stream = self.get_media_stream()?;
        }
        if let Some(media_stream) = stream{
            let tracks = media_stream.get_tracks();
            let kind = track_kind.unwrap_or(MediaStreamTrackKind::All);
            let mut all = false;
            let mut video = false;
            let mut audio = false;
            match kind {
                MediaStreamTrackKind::All=>{
                    all = true;
                }
                MediaStreamTrackKind::Video=>{
                    video = true;
                }
                MediaStreamTrackKind::Audio=>{
                    audio = true;
                }
            }

            for index in 0..tracks.length(){
                if let Ok(track) = tracks.get(index).dyn_into::<MediaStreamTrack>(){
                    let k = track.kind();
                    if all || (k.eq("video") && video) || (k.eq("audio") && audio){
                        track.stop();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn push_window_listener(&self, listener:Listener<Callback<nw::Window>>)->Result<()>{
        let mut locked = match self.win_listeners.lock(){
            Ok(a)=>a,
            Err(e)=>{
                return Err(e.to_string().into());
            }
        };
        locked.push(listener);
        Ok(())
    }

    pub fn push_js_value_listener(&self, listener:Listener<Callback<JsValue>>)->Result<()>{
        let mut locked = match self.js_value_listeners.lock(){
            Ok(a)=>a,
            Err(e)=>{
                return Err(e.to_string().into());
            }
        };
        locked.push(listener);
        Ok(())
    }

    pub fn push_js_value_listener_without_result(&self, listener:Listener<CallbackWithouResult<JsValue>>)->Result<()>{
        let mut locked = match self.js_value_listeners_without_result.lock(){
            Ok(a)=>a,
            Err(e)=>{
                return Err(e.to_string().into());
            }
        };
        locked.push(listener);
        Ok(())
    }

    pub fn push_listener(&self, listener:Listener<Callback<MouseEvent>>)->Result<()>
    {
        let mut locked = match self.mouse_listeners.lock(){
            Ok(a)=>a,
            Err(e)=>{
                return Err(e.to_string().into());
            }
        };
        locked.push(listener);
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
        let listener = Listener::<Callback<nw::Window>>::with_callback(callback);
    
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

        self.on_context_menu(move |ev:MouseEvent|->std::result::Result<(), JsValue>{
            ev.prevent_default();
            popup_menu.popup(ev.x(), ev.y());
            Ok(())
        })?;

        Ok(())
    }

    pub fn on_context_menu<F>(&self, callback:F)->Result<()>
    where
        F: FnMut(MouseEvent) -> std::result::Result<(), JsValue> + 'static
    {
        let win = nw::Window::get();
        let dom_win = win.window();
        let body = utils::body(Some(dom_win));

        let listener = Listener::with_callback(callback);
        body.add_event_listener_with_callback("contextmenu", listener.into_js())?;
        self.push_listener(listener)?;

        Ok(())
    }
    
} 




