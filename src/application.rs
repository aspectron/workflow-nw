//! 
//! Node Webkit application helper provided by the [`App`] struct.
//! 
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub use workflow_wasm::callback::{CallbackMap, CallbackId, Callback, AsCallback, CallbackClosure, CallbackClosureWithoutResult};
use nw_sys::{prelude::*, utils, result::Result};
use web_sys::{MouseEvent, MediaStream, MediaStreamTrack};
use crate::media::MediaStreamTrackKind;

static mut APP:Option<Arc<Application>> = None;

pub fn app()->Option<Arc<Application>>{
    unsafe{APP.clone()}
}

/// Application helper. This struct contains a map of callbacks that
/// can be used to retain different application callbacks as well as
/// media stream helper functions for controlling media playback.
#[derive(Clone)]
pub struct Application{
    pub media_stream: Arc<Mutex<Option<MediaStream>>>,
    pub callbacks: CallbackMap, //Arc<Mutex<HashMap<CallbackId, Arc<dyn AsCallback>>>>,
}

impl Application{
    pub fn new()->Result<Arc<Self>>{
        let app = Arc::new(Self{
            callbacks: CallbackMap::new(), //Arc::new(Mutex::new(HashMap::new())),
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
        track_kind: Option<MediaStreamTrackKind>,
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

    // pub fn push_callback<L>(&self, callback:L)->Result<()>
    // where
    //     L: Sized + AsCallback + 'static
    // {
    //     let mut locked = match self.callbacks.lock(){
    //         Ok(a)=>a,
    //         Err(e)=>{
    //             return Err(e.to_string().into());
    //         }
    //     };
    //     let id = callback.get_id();
    //     locked.insert(id, Arc::new(callback));
    //     Ok(())
    // }

    // pub fn remove_callback(&self, id:&CallbackId)->Result<()>{
    //     let mut locked = match self.callbacks.lock(){
    //         Ok(a)=>a,
    //         Err(e)=>{
    //             return Err(e.to_string().into());
    //         }
    //     };

    //     locked.remove(id);
    //     Ok(())
    // }

    pub fn create_window_with_callback<F>(
        &self,
        url:&str,
        option:&nw_sys::window::Options,
        callback:F
    )->Result<()>
    where
        F:FnMut(nw_sys::Window) -> std::result::Result<(), JsValue> + 'static
    {
        let listener = Callback::<CallbackClosure<nw_sys::Window>>::new(callback);
    
        nw_sys::window::open_with_options_and_callback(
            url,
            option,
            listener.as_ref()
            // listener.into_js()
        );

        self.callbacks.insert(listener)?;
        Ok(())
    }

    pub fn create_window(url:&str, option:&nw_sys::window::Options)->Result<()>{
        nw_sys::window::open_with_options(url, option);

        Ok(())
    }

    pub fn create_context_menu(&self, menus: Vec<nw_sys::MenuItem>)->Result<()>{
        let popup_menu = nw_sys::Menu::new();
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
        F: Sized+FnMut(MouseEvent) -> std::result::Result<(), JsValue> + 'static
    {
        let win = nw_sys::window::get();
        let dom_win = win.window();
        let body = utils::body(Some(dom_win));

        let listener = Callback::<CallbackClosure<MouseEvent>>::new(callback);
        // body.add_event_listener_with_callback("contextmenu", listener)?;
        body.add_event_listener_with_callback("contextmenu", listener.into_js())?;

        self.callbacks.insert(listener)?;

        Ok(())
    }
    
} 



