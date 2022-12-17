use wasm_bindgen::prelude::*;
use js_sys::Function;
use nw_sys::{prelude::*, result::Result};
use workflow_wasm::listener::Listener;
use nw::menu_item::Type as MenuItemType;
use nw::{Menu, MenuItem};
use crate::app::{app, Callback};

pub fn menu_separator()->nw::MenuItem{
    nw::MenuItem::new(&nw::menu_item::Type::Separator.into())
}


pub struct MenubarBuilder{
    pub mac_options: nw::menu::MacOptions,
    pub app_name: String,
    pub menubar: nw::Menu,
    pub menu_items: Vec<nw::MenuItem>
}

impl MenubarBuilder{
    pub fn new(app_name:&str)->Self{
        Self{
            mac_options: nw::menu::MacOptions::new(),
            app_name: app_name.to_string(),
            menubar: nw::Menu::new_with_options(&nw::menu::Type::Menubar.into()),
            menu_items: vec![]
        }
    }
    /// (Mac) do not populate the Edit menu
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn mac_hide_edit(mut self, hide:bool)->Self{
        self.mac_options = self.mac_options.hide_edit(hide);
        self
    }
    /// (Mac) do not populate the Window menu
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn mac_hide_window(mut self, hide:bool)->Self{
        self.mac_options = self.mac_options.hide_window(hide);
        self
    }

    /// Append new child menu item
    pub fn append(mut self, menu_item:nw::MenuItem)->Self{
        self.menu_items.push(menu_item);
        self
    }

    /// Build menubar
    /// 
    /// optionally attach menubar to app/window
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/For%20Users/Advanced/Customize%20Menubar/#create-and-set-menubar)
    pub fn build(self, attach:bool)->Result<nw::Menu>{
        self.menubar.create_mac_builtin_with_options(&self.app_name, &self.mac_options);
        for item in self.menu_items{
            self.menubar.append(&item);
        }
        if attach{
            nw::Window::get().set_menu(&self.menubar);
        }
        Ok(self.menubar)
    }

}

pub struct MenuItemBuilder{
    pub options: nw::menu_item::Options,
    pub listener: Option<Listener<Callback<JsValue>>>
}

impl MenuItemBuilder{
    pub fn new()->Self{
        Self{
            options: nw::menu_item::Options::new(),
            listener: None
        }
    }

    fn set(mut self, key:&str, value:JsValue)->Self{
        self.options = self.options.set(key, value);
        self
    }

    /// Type of MenuItem
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn set_type(self, t:MenuItemType)->Self{
        self.set("type", t.into())
    }

    /// Label for normal item or checkbox
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn label(self, label:&str)->Self{
        self.set("label", JsValue::from(label))
    }

    /// Icon for normal item or checkbox
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn icon(self, icon:&str)->Self{
        self.set("icon", JsValue::from(icon))
    }

    /// Tooltip for normal item or checkbox
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn tooltip(self, tooltip:&str)->Self{
        self.set("tooltip", JsValue::from(tooltip))
    }

    /// The callback function when item is triggered by mouse click or keyboard shortcut
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn callback<F>(mut self, callback:F)->Self
    where
        F:FnMut(JsValue) -> std::result::Result<(), JsValue> + 'static
    {
        let listener = Listener::with_callback(callback);
        let cb:&Function = listener.into_js();
        self = self.set("click", JsValue::from(cb));
        self.listener = Some(listener);

        self        
    }

    /// Whether the item is enabled or disabled. It’s set to true by default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn enabled(self, enabled:bool)->Self{
        self.set("enabled", JsValue::from(enabled))
    }

    /// Whether the checkbox is checked or not. It’s set to false by default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn checked(self, checked:bool)->Self{
        self.set("checked", JsValue::from(checked))
    }

    /// A submenu
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn submenu(self, submenu:&Menu)->Self{
        self.set("submenu", JsValue::from(submenu))
    }

    /// Create submenu from menu items
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn submenus(self, items:Vec<MenuItem>)->Self{
        let submenu = nw::Menu::new();
        for menu_item in items{
            submenu.append(&menu_item);
        }
        self.set("submenu", JsValue::from(submenu))
    }

    /// The key of the shortcut
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn key(self, key:&str)->Self{
        self.set("key", JsValue::from(key))
    }

    /// The modifiers of the shortcut
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn modifiers(self, modifiers:&str)->Self{
        self.set("modifiers", JsValue::from(modifiers))
    }

    pub fn build(self)->Result<nw::MenuItem>{
        if let Some(listener) = self.listener{
            let app = match app(){
                Some(app)=>app,
                None=>return Err("app is not initialized".to_string().into())
            };
            app.push_listener(listener)?;
        }

        let menu_item = nw::MenuItem::new(&self.options);
        Ok(menu_item)
    }

    pub fn finalize(self)->Result<(nw::MenuItem, Option<Listener<Callback<JsValue>>>)>{
        let menu_item = nw::MenuItem::new(&self.options);
        Ok((menu_item, self.listener))
    }
}
