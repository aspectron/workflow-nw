use wasm_bindgen::prelude::*;
use js_sys::Function;
use nw_sys::{prelude::*, result::Result};
use crate::app::{app, Callback, CallbackClosure};

pub struct ShortcutBuilder{
    pub options: nw_sys::shortcut::Options,
    pub active_listener: Option<Callback<CallbackClosure<JsValue>>>,
    pub failed_listener: Option<Callback<CallbackClosure<JsValue>>>
}

impl ShortcutBuilder{
    pub fn new()->Self{
        Self{
            options: nw_sys::shortcut::Options::new(),
            active_listener: None,
            failed_listener: None
        }
    }

    fn set(mut self, key:&str, value:JsValue)->Self{
        self.options = self.options.set(key, value);
        self
    }

    /// Set the `key` of a `Shortcut`.
    /// It is a string to specify the shortcut key, like "Ctrl+Alt+A".
    /// The key is consisted of zero or more modifiers and a key on your keyboard.
    /// Only one key code is supported. Key code is case insensitive.
    /// 
    /// ### List of supported modifiers:
    /// 
    /// - Ctrl
    /// - Alt
    /// - Shift
    /// - Command: Command modifier maps to Apple key (⌘) on Mac, 
    /// and maps to the Windows key on Windows and Linux.
    /// 
    /// ### List of supported keys:
    /// 
    /// - Alphabet: `A`-`Z`
    /// - Digits: `0`-`9`
    /// - Function Keys: `F1`-`F24`
    /// - Home / End / PageUp / PageDown / Insert / Delete
    /// - Up / Down / Left / Right
    /// - MediaNextTrack / MediaPlayPause / MediaPrevTrack / MediaStop
    /// - Comma or `,`
    /// - Period or `.`
    /// - Tab or `\t`
    /// - Backquote or `` ` ``
    /// - Enter or `\n`
    /// - Minus or `-`
    /// - Equal or `=`
    /// - Backslash or `\`
    /// - Semicolon or `;`
    /// - Quote or `'`
    /// - BracketLeft or `[`
    /// - BracketRight or `]`
    /// - Escape
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutkey)
    pub fn key(self, key:&str)->Self{
        self.set("key", JsValue::from(key))
    }

    /// Set the active callback of a Shortcut.
    /// It will be called when user presses the shortcut.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    pub fn active<F>(mut self, callback:F)->Self
    where
        F:FnMut(JsValue) -> std::result::Result<(), JsValue> + 'static
    {
        let listener = Callback::with_closure(callback);
        let cb:&Function = listener.into_js();
        self = self.set("active", JsValue::from(cb));
        self.active_listener = Some(listener);

        self        
    }

    /// Set the failed callback of a Shortcut.
    /// It will be called when application passes an invalid key,
    /// or failed to register the key.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutfailed)
    pub fn failed<F>(mut self, callback:F)->Self
    where
        F:FnMut(JsValue) -> std::result::Result<(), JsValue> + 'static
    {
        let listener = Callback::with_closure(callback);
        let cb:&Function = listener.into_js();
        self = self.set("failed", JsValue::from(cb));
        self.failed_listener = Some(listener);

        self        
    }

    pub fn build(self)->Result<nw_sys::Shortcut>{
        if let Some(listener) = self.active_listener{
            let app = match app(){
                Some(app)=>app,
                None=>return Err("app is not initialized".to_string().into())
            };
            app.push_callback(listener)?;
        }
        if let Some(listener) = self.failed_listener{
            let app = match app(){
                Some(app)=>app,
                None=>return Err("app is not initialized".to_string().into())
            };
            app.push_callback(listener)?;
        }

        let menu_item = nw_sys::Shortcut::new(&self.options);
        Ok(menu_item)
    }

    pub fn finalize(self)
    ->Result<(
        nw_sys::Shortcut,
        Option<Callback<CallbackClosure<JsValue>>>,
        Option<Callback<CallbackClosure<JsValue>>>
    )>{
        let menu_item = nw_sys::Shortcut::new(&self.options);
        Ok((menu_item, self.active_listener, self.failed_listener))
    }
}