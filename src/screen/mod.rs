mod priority;

use core::{mem, pin::Pin};

pub use self::priority::Priority;
use crate::{
    bindings::{cc_string, Gui_Add, Gui_Remove, Screen, ScreenVTABLE},
    std_types::{c_char, c_int, c_void, Box},
};

pub struct OwnedScreen {
    pub screen: Pin<Box<Screen>>,
    vtable: Pin<Box<ScreenVTABLE>>,
    added: bool,
}

impl OwnedScreen {
    pub fn new() -> Self {
        let mut vtable = Box::pin(ScreenVTABLE {
            Init: Some(Init),
            Update: Some(Update),
            Free: Some(Free),
            Render: Some(Render),
            BuildMesh: Some(BuildMesh),
            HandlesInputDown: Some(HandlesInputDown),
            OnInputUp: Some(OnInputUp),
            HandlesKeyPress: Some(HandlesKeyPress),
            HandlesTextChanged: Some(HandlesTextChanged),
            HandlesPointerDown: Some(HandlesPointerDown),
            OnPointerUp: Some(OnPointerUp),
            HandlesPointerMove: Some(HandlesPointerMove),
            HandlesMouseScroll: Some(HandlesMouseScroll),
            Layout: Some(Layout),
            ContextLost: Some(ContextLost),
            ContextRecreated: Some(ContextRecreated),
        });

        let screen = Box::pin(unsafe {
            let mut screen: Screen = mem::zeroed();
            screen.VTABLE = vtable.as_mut().get_unchecked_mut();
            screen
        });

        Self {
            screen,
            vtable,
            added: false,
        }
    }

    pub fn add<T: Into<Priority>>(&mut self, priority: T) {
        if self.added {
            return;
        }
        unsafe {
            // priority is stored as a u8 even though api is c_int
            Gui_Add(
                self.screen.as_mut().get_unchecked_mut(),
                priority.into().to_u8() as _,
            );
        }
        self.added = true;
    }

    pub fn remove(&mut self) {
        if self.added {
            unsafe {
                Gui_Remove(self.screen.as_mut().get_unchecked_mut());
            }
            self.added = false;
        }
    }

    /// Initialises persistent state.
    pub fn on_init(&mut self, f: unsafe extern "C" fn(elem: *mut c_void)) -> &mut Self {
        self.vtable.as_mut().Init = Some(f);
        self
    }

    /// Updates this screen, called every frame just before Render().
    pub fn on_update(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, delta: f64),
    ) -> &mut Self {
        self.vtable.as_mut().Update = Some(f);
        self
    }

    /// Frees/releases persistent state.
    pub fn on_free(&mut self, f: unsafe extern "C" fn(elem: *mut c_void)) -> &mut Self {
        self.vtable.as_mut().Free = Some(f);
        self
    }

    /// Draws this screen and its widgets on screen.
    pub fn on_render(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, delta: f64),
    ) -> &mut Self {
        self.vtable.as_mut().Render = Some(f);
        self
    }

    /// Builds the vertex mesh for all the widgets in the screen.
    pub fn on_build_mesh(&mut self, f: unsafe extern "C" fn(elem: *mut c_void)) -> &mut Self {
        self.vtable.as_mut().BuildMesh = Some(f);
        self
    }

    /// Returns non-zero if an input press is handled.
    pub fn on_handles_input_down(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, key: c_int) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesInputDown = Some(f);
        self
    }

    /// Returns non-zero if an input release is handled.
    pub fn on_on_input_up(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, key: c_int),
    ) -> &mut Self {
        self.vtable.as_mut().OnInputUp = Some(f);
        self
    }

    /// Returns non-zero if a key character press is handled.
    pub fn on_handles_key_press(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, keyChar: c_char) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesKeyPress = Some(f);
        self
    }

    /// Returns non-zero if a key character press is handled.
    /// Currently only raised by on-screen keyboard in web client.
    pub fn on_handles_text_changed(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, str: *const cc_string) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesTextChanged = Some(f);
        self
    }

    /// Returns non-zero if a pointer press is handled.
    pub fn on_handles_pointer_down(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesPointerDown = Some(f);
        self
    }

    /// Returns non-zero if a pointer release is handled.
    pub fn on_on_pointer_up(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int),
    ) -> &mut Self {
        self.vtable.as_mut().OnPointerUp = Some(f);
        self
    }

    /// Returns non-zero if a pointer movement is handled.
    pub fn on_handles_pointer_move(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, id: c_int, x: c_int, y: c_int) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesPointerMove = Some(f);
        self
    }

    /// Returns non-zero if a mouse wheel scroll is handled.
    pub fn on_handles_mouse_scroll(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void, delta: f32) -> c_int,
    ) -> &mut Self {
        self.vtable.as_mut().HandlesMouseScroll = Some(f);
        self
    }

    /// Positions widgets on screen. Typically called on window resize.
    pub fn on_layout(&mut self, f: unsafe extern "C" fn(elem: *mut c_void)) -> &mut Self {
        self.vtable.as_mut().Layout = Some(f);
        self
    }

    /// Destroys graphics resources. (textures, vertex buffers, etc)
    pub fn on_context_lost(&mut self, f: unsafe extern "C" fn(elem: *mut c_void)) -> &mut Self {
        self.vtable.as_mut().ContextLost = Some(f);
        self
    }

    /// Allocates graphics resources. (textures, vertex buffers, etc)
    pub fn on_context_recreated(
        &mut self,
        f: unsafe extern "C" fn(elem: *mut c_void),
    ) -> &mut Self {
        self.vtable.as_mut().ContextRecreated = Some(f);
        self
    }
}

impl Default for OwnedScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for OwnedScreen {
    fn drop(&mut self) {
        #[cfg(not(test))]
        self.remove();
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

#[test]
fn test_screen() {
    extern "C" fn init(_elem: *mut c_void) {
        //
    }

    let _screen = OwnedScreen::new().on_init(init);
    let mut screen = OwnedScreen::new();
    screen.on_init(init);
}
