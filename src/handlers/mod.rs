use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor, delegate_data_device, delegate_output,
    delegate_primary_selection, delegate_seat, delegate_shm,
    input::{SeatHandler, SeatState},
    reexports::wayland_server::{
        protocol::wl_surface::WlSurface,
        Resource,
    },
    wayland::{
        buffer::BufferHandler,
        compositor::{get_parent, is_sync_subsurface, CompositorHandler, CompositorState},
        data_device::{
            set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler, ServerDndGrabHandler,
        },
        primary_selection::{set_primary_focus, PrimarySelectionHandler},
        seat::WaylandFocus,
        shm::{ShmHandler, ShmState},
    },
};

use crate::{
    state::{Backend, CompState},
    utils::focus::FocusTarget,
};

pub mod input;
pub mod xdg_shell;

impl<BackendData: Backend> CompositorHandler for CompState<BackendData> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler(surface);
        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self
                .workspaces
                .all_windows()
                .find(|w| w.toplevel().wl_surface() == &root) {
                window.on_commit();
            }
        };
        self.popup_manager.commit(surface);
        xdg_shell::handle_commit(&self.workspaces, surface, &self.popup_manager);
    }

}


impl<BackendData: Backend> BufferHandler for CompState<BackendData> {
    fn buffer_destroyed(
        &mut self,
        _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
    }
}

impl<BackendData: Backend> ShmHandler for CompState<BackendData> {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}


impl<BackendData: Backend> SeatHandler for CompState<BackendData> {
    type KeyboardFocus = FocusTarget;
    type PointerFocus = FocusTarget;
    
    fn seat_state(&mut self) -> &mut SeatState<CompState<BackendData>> {
        &mut self.seat_state
    }
    
    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {}
    fn focus_changed(&mut self, seat: &smithay::input::Seat<Self>, focused: Option<&FocusTarget>) {
        let dh = &self.dh;
        
        let focus = focused
        .and_then(WaylandFocus::wl_surface)
        .and_then(|s| dh.get_client(s.id()).ok());
    set_data_device_focus(dh, seat, focus.clone());
    set_primary_focus(dh, seat, focus);
    
    if let Some(focus_target) = focused {
        match focus_target {
            FocusTarget::Window(w) => {
                for window in self.workspaces.all_windows() {
                    if window.eq(w) {
                        window.set_activated(true);
                    } else {
                        window.set_activated(false);
                    }
                    window.toplevel().send_configure();
                }
            }
        };
    }
}
}


//
// Wl Data Device
//

impl<BackendData: Backend> DataDeviceHandler for CompState<BackendData> {
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }
}

impl<BackendData: Backend> ClientDndGrabHandler for CompState<BackendData> {}
impl<BackendData: Backend> ServerDndGrabHandler for CompState<BackendData> {}


impl<BackendData: Backend,> PrimarySelectionHandler for CompState<BackendData> {
    fn primary_selection_state(
        &self,
    ) -> &smithay::wayland::primary_selection::PrimarySelectionState {
        &self.primary_selection_state
    }
}

delegate_compositor!(@<BackendData: Backend + 'static> CompState<BackendData>);
delegate_shm!(@<BackendData: Backend + 'static> CompState<BackendData>);
delegate_seat!(@<BackendData: Backend + 'static> CompState<BackendData>);
delegate_data_device!(@<BackendData: Backend + 'static> CompState<BackendData>);
delegate_primary_selection!(@<BackendData: Backend + 'static> CompState<BackendData>);
delegate_output!(@<BackendData: Backend + 'static> CompState<BackendData>);
