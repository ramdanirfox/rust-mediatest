use std::{os::raw::{c_char, c_void, c_float}, ptr, ffi::{CString, c_schar}};
use openmpt::module::Logger;
use openmpt_sys::{openmpt_module_initial_ctl, openmpt_module};


const LIBPATH: &str = "lib/dll/libopenmpt.dll";


#[repr(C)]
pub struct OpenMptModuleExtInterfaceInteractive {
  // pub set_current_speed: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub set_current_tempo: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub set_tempo_factor: extern "C"  fn(*mut openmpt_module_ext, f64) -> i32,
  // pub get_tempo_factor: extern "C" fn(*mut openmpt_module_ext) -> f64,
  // pub set_pitch_factor: extern "C" fn(*mut openmpt_module_ext, f64) -> i32,
  // pub get_pitch_factor: extern "C" fn(*mut openmpt_module_ext) -> f64,
  // pub set_global_volume: extern "C" fn(*mut openmpt_module_ext, f64) -> i32,
  // pub get_global_volume: extern "C" fn(*mut openmpt_module_ext) -> f64,
  // pub set_channel_volume: extern "C" fn(*mut openmpt_module_ext, i32, f64) -> i32,
  // pub get_channel_volume: extern "C" fn(*mut openmpt_module_ext, i32) -> f64,
  // pub set_channel_mute_status: extern "C" fn(*mut openmpt_module_ext, i32, i32) -> i32,
  // pub get_channel_mute_status: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub set_instrument_mute_status: extern "C" fn(*mut openmpt_module_ext, i32, i32) -> i32,
  // pub get_instrument_mute_status: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub play_note: extern "C" fn(*mut openmpt_module_ext, i32, i32, f64, f64) -> i32,
  // pub stop_note: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  set_current_speed: Option<i32>,
  set_current_tempo: Option<i32>,
  set_tempo_factor: Option<i32>,
  get_tempo_factor: Option<i32>,
  set_pitch_factor: Option<i32>,
  get_pitch_factor: Option<i32>,
  set_global_volume: Option<i32>,
  get_global_volume: Option<i32>,
  set_channel_volume: Option<i32>,
  get_channel_volume: Option<i32>,
  set_channel_mute_status: Option<i32>,
  get_channel_mute_status: Option<i32>,
  set_instrument_mute_status: Option<i32>,
  get_instrument_mute_status: Option<i32>,
  play_note: Option<i32>,
  stop_note: Option<i32>
}


extern "C" fn openmpt_error_func_default(error: *const ::std::os::raw::c_int,
    user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int {
      println!("Ada error gaes!");
      error
    }

fn log_func(param: Logger) -> openmpt_sys::openmpt_log_func {
    match param {
        Logger::StdErr => Some(openmpt_sys::openmpt_log_func_default),
        Logger::None => Some(openmpt_sys::openmpt_log_func_silent)
    }
}


pub fn libopenmpt_module_ext_create_from_memory(
    filedata: *const ::std::os::raw::c_void,
    filesize: usize,
    logfunc: openmpt_sys::openmpt_log_func,
    loguser: *mut ::std::os::raw::c_void,
    errfunc: extern "C" fn(error: *const ::std::os::raw::c_int, user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int,
    erruser: *mut ::std::os::raw::c_void,
    error: *mut ::std::os::raw::c_int,
    error_message: *mut *const c_char,
    ctls: *const openmpt_module_initial_ctl
) -> Result<*mut openmpt_module, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(LIBPATH)?;
        let func: libloading::Symbol<unsafe extern fn(
            filedata: *const ::std::os::raw::c_void,
            filesize: usize,
            logfunc: openmpt_sys::openmpt_log_func,
            loguser: *mut ::std::os::raw::c_void,
            errfunc: extern "C" fn(error: *const ::std::os::raw::c_int, user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int,
            erruser: *mut ::std::os::raw::c_void,
            error: *mut ::std::os::raw::c_int,
            error_message: *mut *const c_char,
            ctls: *const openmpt_module_initial_ctl
        ) -> *mut openmpt_module> = lib.get(b"openmpt_module_ext_create_from_memory")?;
        Ok(func(
            filedata,
            filesize,
            logfunc,
            loguser,
            errfunc,
            erruser,
            error,
            error_message,
            ctls
        ))
    }
}



fn libopenmpt_module_ext_get_interface(
    module_ext: *mut openmpt_module,
    interface_id: *const ::std::os::raw::c_char,
    interface: *mut c_void,
    interface_size: usize
) -> Result<::std::os::raw::c_int, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(LIBPATH)?;
        let func: libloading::Symbol<unsafe extern fn(
            module_ext: *mut openmpt_module,
            interface_id: *const ::std::os::raw::c_char,
            interface: *mut c_void,
            interface_size: usize,
          ) -> ::std::os::raw::c_int> = lib.get(b"openmpt_module_ext_get_interface")?;
        Ok(func(
            module_ext,
            interface_id,
            interface,
            interface_size
        ))
    }
}

pub fn initialize() {
    let module_data = include_bytes!("..\\examples\\expr.it");
    let module_ptr= unsafe {
        // openmpt_module_create_from_memory2(
        libopenmpt_module_ext_create_from_memory(
          module_data.as_ptr() as *const c_void,
          module_data.len(),
          log_func(Logger::StdErr),
          ptr::null_mut(),
          openmpt_error_func_default,
          ptr::null_mut(),
          ptr::null_mut(),
          ptr::null_mut(),
          ptr::null_mut()
        )
      }.unwrap();

      if !module_ptr.is_null() {
        println!("Module is loaded, whats next?");
      }

      // obtain a valid openmpt_module_ext pointer and interface_id 
      let interface_id_bind = CString::new("interactive").unwrap();
        let interface_id = interface_id_bind.as_ptr();

        // define the interface struct and its size
        let mut interface: OpenMptModuleExtInterfaceInteractive = OpenMptModuleExtInterfaceInteractive {
            set_current_speed: None,
            set_current_tempo: None,
            set_tempo_factor: None,
            get_tempo_factor: None,
            set_pitch_factor: None,
            get_pitch_factor: None,
            set_global_volume: None,
            get_global_volume: None,
            set_channel_volume: None,
            get_channel_volume: None,
            set_channel_mute_status: None,
            get_channel_mute_status: None,
            set_instrument_mute_status: None,
            get_instrument_mute_status: None,
            play_note: None,
            stop_note: None,
        };
        let interface_size = std::mem::size_of::<OpenMptModuleExtInterfaceInteractive>();

        // call openmpt_module_ext_get_interface
        let result = unsafe {
        libopenmpt_module_ext_get_interface(
                module_ptr,
                interface_id,
                &mut interface as *mut OpenMptModuleExtInterfaceInteractive as *mut c_void,
                interface_size
            )
        }.unwrap();
        // if result != 0 {
        //     panic!("Failed to get interface");
        // }

        println!("interface_size : {}", interface_size);
        println!("interface_id : {:?}", interface_id_bind);

        println!("Hasil getInterface (1=success, 0=interface not found) : {}", result);

}