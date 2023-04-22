use libc::{c_char, c_int, c_void};
use openmpt_sys::{openmpt_module, openmpt_module_initial_ctl};
// include!("openmpt.h");

#[repr(C)]
pub struct liopenmpt_ext_interactive2 {}

#[link(name = "openmpt")]
extern "C" {
    fn openmpt_module_create_from_memory(
        data: *const c_void,
        size: c_int,
        logfunc: Option<extern "C" fn(msg: *const c_char, user: *mut c_void)>,
        user: *mut c_void,
        cfg: *const openmpt_module_initial_ctl,
    ) -> *mut openmpt_module;

    fn openmpt_module_destroy(module: *mut openmpt_module);

    fn openmpt_module_ext_interface_interactive(
        module: *mut openmpt_module,
        interactive2: *mut *mut liopenmpt_ext_interactive2,
    ) -> c_int;

    fn liopenmpt_ext_interactive2_get_channel_mute_status(
        interactive2: *mut liopenmpt_ext_interactive2,
        channel: c_int,
        is_muted: *mut bool,
    ) -> c_int;

    fn liopenmpt_ext_interactive2_destroy(interactive2: *mut liopenmpt_ext_interactive2);
}

// fn main() {
//     let data = "..." as *const str as *const std::ffi::c_void;
//     let size = 1234;
//     let module = unsafe { openmpt_module_create_from_memory(data, size, None, std::ptr::null_mut(), std::ptr::null()) };
//     if module.is_null() {
//         println!("Failed to load module.");
//         return;
//     }
// }

pub fn process() {
    let data = "..." as *const str as *const std::ffi::c_void;
    let size = 1234;
    let module = unsafe { openmpt_module_create_from_memory(data, size, None, std::ptr::null_mut(), std::ptr::null()) };
    if module.is_null() {
        println!("Failed to load module.");
        return;
    }

    let mut interactive2_ptr: *mut liopenmpt_ext_interactive2 = std::ptr::null_mut();
    let err = unsafe { openmpt_module_ext_interface_interactive(module, &mut interactive2_ptr) };
    // if err != OPENMPT_ERROR_OK {
    //     println!("Failed to obtain interactive2 interface.");
    //     unsafe { openmpt_module_destroy(module) };
    //     return;
    // }

    let interactive2 = unsafe { &mut *interactive2_ptr };
    let mut is_muted = false;
    let err = unsafe { liopenmpt_ext_interactive2_get_channel_mute_status(interactive2, 0, &mut is_muted) };
    // if err != LIOPENMPT_ERROR_OK {
    //     println!("Failed to get mute status.");
    //     unsafe { liopenmpt_ext_interactive2_destroy(interactive2_ptr) };
    //     unsafe { openmpt_module_destroy(module) };
    //     return;
    // }

    println!("Channel 0 mute status: {}", is_muted);

    unsafe { liopenmpt_ext_interactive2_destroy(interactive2_ptr) };
    unsafe { openmpt_module_destroy(module) };
}

fn test() {

}