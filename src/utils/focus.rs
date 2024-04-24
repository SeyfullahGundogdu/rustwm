use smithay::desktop::Window;
pub use smithay::{
    backend::input::KeyState,
    input::{
        keyboard::{KeyboardTarget, KeysymHandle, ModifiersState},
        pointer::{AxisFrame, ButtonEvent, MotionEvent, PointerTarget, RelativeMotionEvent},
        Seat,
    },
    reexports::wayland_server::{backend::ObjectId, protocol::wl_surface::WlSurface},
    utils::{IsAlive, Serial},
    wayland::seat::WaylandFocus,
};

use crate::state::{Backend, CompState};

#[derive(Debug, Clone, PartialEq)]
pub enum FocusTarget {
    Window(Window),
}

impl IsAlive for FocusTarget {
    fn alive(&self) -> bool {
        match self {
            FocusTarget::Window(w) => w.alive(),
        }
    }
}

impl From<FocusTarget> for WlSurface {
    fn from(target: FocusTarget) -> Self {
        target.wl_surface().unwrap()
    }
}

impl<BackendData: Backend> PointerTarget<CompState<BackendData>> for FocusTarget {
    fn enter(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        event: &MotionEvent,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::enter(w, seat, data, event),
        }
    }
    fn motion(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        event: &MotionEvent,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::motion(w, seat, data, event),
        }
    }
    fn relative_motion(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        event: &RelativeMotionEvent,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::relative_motion(w, seat, data, event),
        }
    }
    fn button(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        event: &ButtonEvent,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::button(w, seat, data, event),
        }
    }
    fn axis(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        frame: AxisFrame,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::axis(w, seat, data, frame),
        }
    }
    fn leave(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        serial: Serial,
        time: u32,
    ) {
        match self {
            FocusTarget::Window(w) => PointerTarget::leave(w, seat, data, serial, time),
        }
    }
}

impl<BackendData: Backend> KeyboardTarget<CompState<BackendData>> for FocusTarget {
    fn enter(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        keys: Vec<KeysymHandle<'_>>,
        serial: Serial,
    ) {
        match self {
            FocusTarget::Window(w) => KeyboardTarget::enter(w, seat, data, keys, serial),
        }
    }
    fn leave(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        serial: Serial,
    ) {
        match self {
            FocusTarget::Window(w) => KeyboardTarget::leave(w, seat, data, serial),
        }
    }
    fn key(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    ) {
        match self {
            FocusTarget::Window(w) => KeyboardTarget::key(w, seat, data, key, state, serial, time),
        }
    }
    fn modifiers(
        &self,
        seat: &Seat<CompState<BackendData>>,
        data: &mut CompState<BackendData>,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        match self {
            FocusTarget::Window(w) => KeyboardTarget::modifiers(w, seat, data, modifiers, serial),
        }
    }
}

impl WaylandFocus for FocusTarget {
    fn wl_surface(&self) -> Option<WlSurface> {
        match self {
            FocusTarget::Window(w) => w.wl_surface(),
        }
    }
    fn same_client_as(&self, object_id: &ObjectId) -> bool {
        match self {
            FocusTarget::Window(w) => w.same_client_as(object_id),
        }
    }
}

impl From<Window> for FocusTarget {
    fn from(w: Window) -> Self {
        FocusTarget::Window(w)
    }
}

