use wasm_bindgen::{prelude::*, JsCast};
use js_sys::Object;
use nw_sys::options::OptionsExt;
use nw_sys::result::Result;
use workflow_wasm::listener::Listener;
use workflow_log::log_debug;
use workflow_dom::utils::window;
use std::sync::Arc;
use web_sys::MediaStream;
use crate::app::{app, CallbackWithouResult};

pub enum MediaStreamTrackKind {
    Video,
    Audio,
    All
}

impl ToString for MediaStreamTrackKind{
    fn to_string(&self) -> String {
        match self{
            Self::Video=>"Video".to_string(),
            Self::Audio=>"Audio".to_string(),
            Self::All=>"All".to_string()
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type VideoConstraints;
}

impl OptionsExt for VideoConstraints{}

/*

/// Auto gain control
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn auto_gain_control(self, auto_gain_control:bool)->Self{
        self.set("autoGainControl", JsValue::from(auto_gain_control))
    }
*/

impl VideoConstraints {


    /// Source Id
    /// 
    /// 
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn source_id(self, source_id:&str)->Self{
        self.set("mandatory.chromeMediaSource", JsValue::from("desktop"))
            .set("mandatory.chromeMediaSourceId", JsValue::from(source_id))
    }

    /// Max Width
    /// 
    /// 
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn max_width(self, max_width:u32)->Self{
        self.set("mandatory.maxWidth", JsValue::from(max_width))
    }

    /// Max Height
    /// 
    /// 
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn max_height(self, max_height:u32)->Self{
        self.set("mandatory.maxHeight", JsValue::from(max_height))
    }


    /// Device Id
    /// 
    /// a device ID or an array of device IDs which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn device_id(self, device_id:&str)->Self{
        self.set("deviceId", JsValue::from(device_id))
    }

    /// Group Id
    /// 
    /// a group ID or an array of group IDs which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn group_id(self, group_id:&str)->Self{
        self.set("groupId", JsValue::from(group_id))
    }

    /// Aspect ratio of video
    /// 
    /// specifying the video aspect ratio or range of aspect ratios 
    /// which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn aspect_ratio(self, aspect_ratio:f32)->Self{
        self.set("aspectRatio", JsValue::from(aspect_ratio))
    }

    /// Facing mode
    /// 
    /// Object specifying a facing or an array of facings which are acceptable 
    /// and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn facing_mode(self, facing_mode:&str)->Self{
        self.set("facingMode", JsValue::from(facing_mode))
    }

    /// Frame rate
    /// 
    /// frame rate or range of frame rates which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn frame_rate(self, frame_rate:f32)->Self{
        self.set("frameRate", JsValue::from(frame_rate))
    }

    /// Width of video
    /// 
    /// video width or range of widths which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn width(self, width:u16)->Self{
        self.set("width", JsValue::from(width))
    }

    ///Height of video
    /// 
    /// video height or range of heights which are acceptable and/or required.
    /// 
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints)
    pub fn height(self, height:u16)->Self{
        self.set("height", JsValue::from(height))
    }

    
}


pub fn get_user_media(
    video_constraints: VideoConstraints,
    audio_constraints: Option<JsValue>,
    callback: Arc<dyn Fn(Option<MediaStream>)>
)->Result<()>{
    let app = match app(){
        Some(app)=>app,
        None=>return Err("app is not initialized".to_string().into())
    };

    let navigator = window().navigator();
    let media_devices = navigator.media_devices()?;

    log_debug!("navigator: {:?}", navigator);
    log_debug!("media_devices: {:?}", media_devices);
    log_debug!("video_constraints: {:?}", video_constraints);

    let audio_constraints = audio_constraints.unwrap_or(JsValue::from(false));

    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints
        .audio(&audio_constraints)
        .video(&JsValue::from(&video_constraints));

    log_debug!("constraints: {:?}", constraints);

    let promise = media_devices.get_user_media_with_constraints(&constraints)?;

    let mut listener = Listener::<CallbackWithouResult<JsValue>>::new();

    listener.callback(move |value:JsValue|{
        if let Ok(media_stream) = value.dyn_into::<MediaStream>(){
            callback(Some(media_stream));
        }else{
            callback(None);
        }
    });

    //log_info!("listener: {:?}", listener);
    let binding = match listener.closure(){
        Ok(b)=>b,
        Err(err)=>{
            return Err(format!("media::get_user_media(), listener.closure_without_result failed, error: {:?}", err).into());
        }
    };
    let cb = binding.as_ref();
    let _ = promise.then(cb);

    app.push_listener(listener)?;
    Ok(())
}
