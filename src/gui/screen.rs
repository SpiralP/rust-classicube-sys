#![allow(non_snake_case)]

use crate::bindings::{
    cc_string, GuiPriority_GUI_PRIORITY_CHAT, GuiPriority_GUI_PRIORITY_DISCONNECT,
    GuiPriority_GUI_PRIORITY_HUD, GuiPriority_GUI_PRIORITY_INVENTORY,
    GuiPriority_GUI_PRIORITY_LOADING, GuiPriority_GUI_PRIORITY_MENU,
    GuiPriority_GUI_PRIORITY_OLDLOADING, GuiPriority_GUI_PRIORITY_TEXIDS,
    GuiPriority_GUI_PRIORITY_TEXPACK, GuiPriority_GUI_PRIORITY_TOUCH,
    GuiPriority_GUI_PRIORITY_TOUCHMORE, GuiPriority_GUI_PRIORITY_URLWARNING, Gui_Add, Gui_Remove,
    Screen as CCScreen, ScreenVTABLE,
};
use std::{
    mem,
    os::raw::{c_char, c_int, c_void},
    pin::Pin,
};

pub struct Screen {
    _vtable: Pin<Box<ScreenVTABLE>>,
    screen: Pin<Box<CCScreen>>,
}

impl Screen {
    pub fn new(callbacks: Callbacks) -> Self {
        let mut vtable = Box::pin(ScreenVTABLE {
            Init: Some(callbacks.init.unwrap_or(Self::Init)),
            Update: Some(callbacks.update.unwrap_or(Self::Update)),
            Free: Some(callbacks.free.unwrap_or(Self::Free)),
            Render: Some(callbacks.render.unwrap_or(Self::Render)),
            BuildMesh: Some(callbacks.build_mesh.unwrap_or(Self::BuildMesh)),
            HandlesInputDown: Some(
                callbacks
                    .handles_input_down
                    .unwrap_or(Self::HandlesInputDown),
            ),
            OnInputUp: Some(callbacks.on_input_up.unwrap_or(Self::OnInputUp)),
            HandlesKeyPress: Some(callbacks.handles_key_press.unwrap_or(Self::HandlesKeyPress)),
            HandlesTextChanged: Some(
                callbacks
                    .handles_text_changed
                    .unwrap_or(Self::HandlesTextChanged),
            ),
            HandlesPointerDown: Some(
                callbacks
                    .handles_pointer_down
                    .unwrap_or(Self::HandlesPointerDown),
            ),
            OnPointerUp: Some(callbacks.on_pointer_up.unwrap_or(Self::OnPointerUp)),
            HandlesPointerMove: Some(
                callbacks
                    .handles_pointer_move
                    .unwrap_or(Self::HandlesPointerMove),
            ),
            HandlesMouseScroll: Some(
                callbacks
                    .handles_mouse_scroll
                    .unwrap_or(Self::HandlesMouseScroll),
            ),
            Layout: Some(callbacks.layout.unwrap_or(Self::Layout)),
            ContextLost: Some(callbacks.context_lost.unwrap_or(Self::ContextLost)),
            ContextRecreated: Some(
                callbacks
                    .context_recreated
                    .unwrap_or(Self::ContextRecreated),
            ),
        });

        let screen = Box::pin(unsafe {
            let mut screen: CCScreen = mem::zeroed();
            screen.VTABLE = vtable.as_mut().get_unchecked_mut();
            screen
        });

        Self {
            _vtable: vtable,
            screen,
        }
    }

    pub fn add<T: Into<Priority>>(&mut self, priority: T) {
        unsafe {
            // priority is stored as a u8 even though api is c_int
            Gui_Add(
                self.screen.as_mut().get_unchecked_mut(),
                priority.into().to_u8() as _,
            );
        }
    }

    pub fn remove(&mut self) {
        unsafe {
            Gui_Remove(self.screen.as_mut().get_unchecked_mut());
        }
    }

    // default noop functions

    unsafe extern "C" fn Init(_elem: *mut c_void) {}

    unsafe extern "C" fn Update(_elem: *mut c_void, _delta: f64) {}

    unsafe extern "C" fn Free(_elem: *mut c_void) {}

    unsafe extern "C" fn Render(_elem: *mut c_void, _delta: f64) {}

    unsafe extern "C" fn BuildMesh(_elem: *mut c_void) {}

    unsafe extern "C" fn HandlesInputDown(_elem: *mut c_void, _key: c_int) -> c_int {
        0
    }

    unsafe extern "C" fn OnInputUp(_elem: *mut c_void, _key: c_int) {}

    unsafe extern "C" fn HandlesKeyPress(_elem: *mut c_void, _keyChar: c_char) -> c_int {
        0
    }

    unsafe extern "C" fn HandlesTextChanged(_elem: *mut c_void, _str: *const cc_string) -> c_int {
        0
    }

    unsafe extern "C" fn HandlesPointerDown(
        _elem: *mut c_void,
        _id: c_int,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }

    unsafe extern "C" fn OnPointerUp(_elem: *mut c_void, _id: c_int, _x: c_int, _y: c_int) {}

    unsafe extern "C" fn HandlesPointerMove(
        _elem: *mut c_void,
        _id: c_int,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }

    unsafe extern "C" fn HandlesMouseScroll(_elem: *mut c_void, _delta: f32) -> c_int {
        0
    }

    unsafe extern "C" fn Layout(_elem: *mut c_void) {}

    unsafe extern "C" fn ContextLost(_elem: *mut c_void) {}

    unsafe extern "C" fn ContextRecreated(_elem: *mut c_void) {}
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.remove();
    }
}

#[derive(Debug, Clone)]
pub enum Priority {
    UnderDisconnect,
    Disconnect,
    OverDisconnect,
    UnderOldLoading,
    OldLoading,
    OverOldLoading,
    UnderMenu,
    Menu,
    OverMenu,
    UnderTouchMore,
    TouchMore,
    OverTouchMore,
    UnderUrlWarning,
    UrlWarning,
    OverUrlWarning,
    UnderTexPack,
    TexPack,
    OverTexPack,
    UnderTexIds,
    TexIds,
    OverTexIds,
    UnderTouch,
    Touch,
    OverTouch,
    UnderInventory,
    Inventory,
    OverInventory,
    UnderChat,
    Chat,
    OverChat,
    UnderHud,
    Hud,
    OverHud,
    UnderLoading,
    Loading,
    OverLoading,
    UnderEverything,
    OverEverything,
    Custom(u8),
}

impl From<u8> for Priority {
    fn from(n: u8) -> Self {
        Self::Custom(n)
    }
}

impl Priority {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::UnderDisconnect => GuiPriority_GUI_PRIORITY_DISCONNECT as u8 - 1,
            Self::Disconnect => GuiPriority_GUI_PRIORITY_DISCONNECT as u8,
            Self::OverDisconnect => GuiPriority_GUI_PRIORITY_DISCONNECT as u8 + 1,
            Self::UnderOldLoading => GuiPriority_GUI_PRIORITY_OLDLOADING as u8 - 1,
            Self::OldLoading => GuiPriority_GUI_PRIORITY_OLDLOADING as u8,
            Self::OverOldLoading => GuiPriority_GUI_PRIORITY_OLDLOADING as u8 + 1,
            Self::UnderMenu => GuiPriority_GUI_PRIORITY_MENU as u8 - 1,
            Self::Menu => GuiPriority_GUI_PRIORITY_MENU as u8,
            Self::OverMenu => GuiPriority_GUI_PRIORITY_MENU as u8 + 1,
            Self::UnderTouchMore => GuiPriority_GUI_PRIORITY_TOUCHMORE as u8 - 1,
            Self::TouchMore => GuiPriority_GUI_PRIORITY_TOUCHMORE as u8,
            Self::OverTouchMore => GuiPriority_GUI_PRIORITY_TOUCHMORE as u8 + 1,
            Self::UnderUrlWarning => GuiPriority_GUI_PRIORITY_URLWARNING as u8 - 1,
            Self::UrlWarning => GuiPriority_GUI_PRIORITY_URLWARNING as u8,
            Self::OverUrlWarning => GuiPriority_GUI_PRIORITY_URLWARNING as u8 + 1,
            Self::UnderTexPack => GuiPriority_GUI_PRIORITY_TEXPACK as u8 - 1,
            Self::TexPack => GuiPriority_GUI_PRIORITY_TEXPACK as u8,
            Self::OverTexPack => GuiPriority_GUI_PRIORITY_TEXPACK as u8 + 1,
            Self::UnderTexIds => GuiPriority_GUI_PRIORITY_TEXIDS as u8 - 1,
            Self::TexIds => GuiPriority_GUI_PRIORITY_TEXIDS as u8,
            Self::OverTexIds => GuiPriority_GUI_PRIORITY_TEXIDS as u8 + 1,
            Self::UnderTouch => GuiPriority_GUI_PRIORITY_TOUCH as u8 - 1,
            Self::Touch => GuiPriority_GUI_PRIORITY_TOUCH as u8,
            Self::OverTouch => GuiPriority_GUI_PRIORITY_TOUCH as u8 + 1,
            Self::UnderInventory => GuiPriority_GUI_PRIORITY_INVENTORY as u8 - 1,
            Self::Inventory => GuiPriority_GUI_PRIORITY_INVENTORY as u8,
            Self::OverInventory => GuiPriority_GUI_PRIORITY_INVENTORY as u8 + 1,
            Self::UnderChat => GuiPriority_GUI_PRIORITY_CHAT as u8 - 1,
            Self::Chat => GuiPriority_GUI_PRIORITY_CHAT as u8,
            Self::OverChat => GuiPriority_GUI_PRIORITY_CHAT as u8 + 1,
            Self::UnderHud => GuiPriority_GUI_PRIORITY_HUD as u8 - 1,
            Self::Hud => GuiPriority_GUI_PRIORITY_HUD as u8,
            Self::OverHud => GuiPriority_GUI_PRIORITY_HUD as u8 + 1,
            Self::UnderLoading => GuiPriority_GUI_PRIORITY_LOADING as u8 - 1,
            Self::Loading => GuiPriority_GUI_PRIORITY_LOADING as u8,
            Self::OverLoading => GuiPriority_GUI_PRIORITY_LOADING as u8 + 1,
            Self::UnderEverything => 0,
            Self::OverEverything => 255,
            Self::Custom(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Callbacks {
    /// Initialises persistent state.
    pub init: Option<unsafe extern "C" fn(elem: *mut c_void)>,
    /// Updates this screen, called every frame just before Render().
    pub update: Option<unsafe extern "C" fn(elem: *mut c_void, delta: f64)>,
    /// Frees/releases persistent state.
    pub free: Option<unsafe extern "C" fn(elem: *mut c_void)>,
    /// Draws this screen and its widgets on screen.
    pub render: Option<unsafe extern "C" fn(elem: *mut c_void, delta: f64)>,
    /// Builds the vertex mesh for all the widgets in the screen.
    pub build_mesh: Option<unsafe extern "C" fn(elem: *mut c_void)>,
    /// Returns non-zero if an input press is handled.
    pub handles_input_down: Option<unsafe extern "C" fn(elem: *mut c_void, key: c_int) -> c_int>,
    /// Returns non-zero if an input release is handled.
    pub on_input_up: Option<unsafe extern "C" fn(elem: *mut c_void, key: c_int)>,
    /// Returns non-zero if a key character press is handled.
    pub handles_key_press:
        Option<unsafe extern "C" fn(elem: *mut c_void, keyChar: c_char) -> c_int>,
    /// Returns non-zero if a key character press is handled.
    /// Currently only raised by on-screen keyboard in web client.
    pub handles_text_changed:
        Option<unsafe extern "C" fn(elem: *mut c_void, str: *const cc_string) -> c_int>,
    /// Returns non-zero if a pointer press is handled.
    pub handles_pointer_down:
        Option<unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int) -> c_int>,
    /// Returns non-zero if a pointer release is handled.
    pub on_pointer_up:
        Option<unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int)>,
    /// Returns non-zero if a pointer movement is handled.
    pub handles_pointer_move:
        Option<unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int) -> c_int>,
    /// Returns non-zero if a mouse wheel scroll is handled.
    pub handles_mouse_scroll: Option<unsafe extern "C" fn(elem: *mut c_void, delta: f32) -> c_int>,
    /// Positions widgets on screen. Typically called on window resize.
    pub layout: Option<unsafe extern "C" fn(elem: *mut c_void)>,
    /// Destroys graphics resources. (textures, vertex buffers, etc)
    pub context_lost: Option<unsafe extern "C" fn(elem: *mut c_void)>,
    /// Allocates graphics resources. (textures, vertex buffers, etc)
    pub context_recreated: Option<unsafe extern "C" fn(elem: *mut c_void)>,
}

// /* Functions for a Screen instance. */
// struct ScreenVTABLE {
// 	/* Initialises persistent state. */
// 	void (*Init)(void* elem);
// 	/* Updates this screen, called every frame just before Render(). */
// 	void (*Update)(void* elem, double delta);
// 	/* Frees/releases persistent state. */
// 	void (*Free)(void* elem);
// 	/* Draws this screen and its widgets on screen. */
// 	void (*Render)(void* elem, double delta);
// 	/* Builds the vertex mesh for all the widgets in the screen. */
// 	void (*BuildMesh)(void* elem);
// 	/* Returns non-zero if an input press is handled. */
// 	int  (*HandlesInputDown)(void* elem, int key);
// 	/* Called when an input key or button is released */
// 	void (*OnInputUp)(void* elem, int key);
// 	/* Returns non-zero if a key character press is handled. */
// 	int  (*HandlesKeyPress)(void* elem, char keyChar);
// 	/* Returns non-zero if on-screen keyboard text changed is handled. */
// 	int  (*HandlesTextChanged)(void* elem, const cc_string* str);
// 	/* Returns non-zero if a pointer press is handled. */
// 	int  (*HandlesPointerDown)(void* elem, int id, int x, int y);
// 	/* Called when a pointer is released. */
// 	void (*OnPointerUp)(void* elem,   int id, int x, int y);
// 	/* Returns non-zero if a pointer movement is handled. */
// 	int  (*HandlesPointerMove)(void* elem, int id, int x, int y);
// 	/* Returns non-zero if a mouse wheel scroll is handled. */
// 	int  (*HandlesMouseScroll)(void* elem, float delta);
// 	/* Positions widgets on screen. Typically called on window resize. */
// 	void (*Layout)(void* elem);
// 	/* Destroys graphics resources. (textures, vertex buffers, etc) */
// 	void (*ContextLost)(void* elem);
// 	/* Allocates graphics resources. (textures, vertex buffers, etc) */
// 	void (*ContextRecreated)(void* elem);
// };

// /* Represents a container of widgets and other 2D elements. May cover entire window. */
// struct Screen {
//     const struct ScreenVTABLE* VTABLE;
//     cc_bool grabsInput;  /* Whether this screen grabs input. Causes the cursor to become visible. */
//     cc_bool blocksWorld; /* Whether this screen completely and opaquely covers the game world behind it. */
//     cc_bool closable;    /* Whether this screen is automatically closed when pressing Escape */
//     cc_bool dirty;       /* Whether this screens needs to have its mesh rebuilt. */
//     int maxVertices;
//     GfxResourceID vb;
//     struct Widget** widgets;
//     int numWidgets;
// };
