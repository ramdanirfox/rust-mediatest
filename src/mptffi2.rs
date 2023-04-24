use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void, c_float};
use std::{ptr, mem};

use openmpt::module::Logger;
use openmpt_sys::{
  openmpt_module,
  openmpt_module_create_from_memory, 
  openmpt_log_func_silent, 
  openmpt_free_string, 
  openmpt_module_initial_ctl, openmpt_log_func
};


type OpenMPTModule = *mut openmpt_module;
type OpenMPTModuleExt = *mut c_void;

impl Default for OpenMptModuleExtInterfaceInteractive {
  fn default() -> Self {
      unsafe { mem::zeroed() }
  }
}

#[repr(C)]
pub struct OpenMptModuleExtInterfaceInteractive {
  // pub set_current_speed: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub set_current_tempo: extern "C" fn(*mut openmpt_module_ext, i32) -> i32,
  // pub set_tempo_factor: extern "C" fn(*mut openmpt_module_ext, f64) -> i32,
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
  stop_note: Option<i32>,
}

#[repr(C)]
struct ModuleExt {
    openmpt_module_ext_metadata_get_key: extern "C" fn(
        *mut openmpt_module,
        *const c_char,
    ) -> *const c_char,
    // ...
}

#[repr(C)]
pub struct openmpt_module_ext {
    // fields omitted
}

// pub fn openmpt_module_create_from_memory(filedata:
//   *const ::std::os::raw::c_void,
// filesize: usize,
// logfunc: openmpt_log_func,
// user:
//   *mut ::std::os::raw::c_void,
// ctls:
//   *const openmpt_module_initial_ctl)
// -> *mut openmpt_module;

extern "C" fn openmpt_error_func_default(error: *const ::std::os::raw::c_int,
  user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int {
    println!("Ada error gaes!");
    error
  }

#[link(name = "openmpt")]
extern "C" {
  pub fn openmpt_module_ext_create_from_memory(
    filedata: *const ::std::os::raw::c_void,
    filesize: usize,
    logfunc: openmpt_sys::openmpt_log_func,
    loguser: *mut ::std::os::raw::c_void,
    errfunc: extern "C" fn(error: *const ::std::os::raw::c_int, user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int,
    erruser: *mut ::std::os::raw::c_void,
    error: *mut ::std::os::raw::c_int,
    error_message: *mut *const c_char,
    ctls: *const openmpt_module_initial_ctl,
) -> *mut openmpt_module;

  fn openmpt_module_create_from_memory2(
    filedata: *const ::std::os::raw::c_void,
    filesize: usize,
    logfunc: openmpt_log_func,
    loguser: *mut ::std::os::raw::c_void,
    errfunc: extern "C" fn(error: *const ::std::os::raw::c_int, user: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_int,
    erruser: *mut ::std::os::raw::c_void,
    error: *mut ::std::os::raw::c_int,
    error_message: *mut *const c_char,
    ctls: *const openmpt_module_initial_ctl,
  ) -> *mut openmpt_module;

  fn openmpt_module_ext_get_interface(
    module_ext: *mut openmpt_module,
    interface_id: *const ::std::os::raw::c_char,
    interface: *mut c_void,
    interface_size: usize,
  ) -> ::std::os::raw::c_int;
  
    fn openmpt_module_ext_metadata_get_key(module: OpenMPTModule, key: *const c_char) -> *const c_char;
    // fn openmpt_module_ext_destruct(ext: *mut *mut c_void);
    fn openmpt_module_destroy(module: OpenMPTModule);
    fn openmpt_get_supported_extensions() -> *const ::std::os::raw::c_char;
}

pub fn proses() {
  // Load module from memory
  let module_data = include_bytes!("..\\examples\\expr.it");
  // let module_ptr = unsafe {
  //     openmpt_module_create_from_memory(
  //         module_data.as_ptr() as *const c_void,
  //         module_data.len(),
  //         log_func(Logger::StdErr),
  //         ptr::null_mut(),
  //         ptr::null_mut(),
  //     )
  // };

  // let module_ptr: *mut openmpt_module = unsafe {
  let module_ptr: *mut openmpt_module = unsafe {
    // openmpt_module_create_from_memory2(
    openmpt_module_ext_create_from_memory(
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
  };

  if module_ptr.is_null() {
      panic!("Failed to load module");
  }

  if !module_ptr.is_null() {
    println!("Module is loaded, whats next?");
  }

  let bufferSize = 5000;

  let sample_rate = 48000;
  // let mut buffer = vec![0f32; bufferSize/1];
  let interleaved_stereo: &mut Vec<c_float> = &mut vec![0f32; bufferSize/1];
  // Try to read module
  let count = interleaved_stereo.capacity() >> 1; // Buffer needs to be of at least size count*2
		
  unsafe {
    openmpt_sys::openmpt_module_read_interleaved_float_stereo(module_ptr, sample_rate, count, interleaved_stereo.as_mut_ptr());
  }

  // println!("buffer is : {:#?}", interleaved_stereo);

  unsafe {
    let extensions = openmpt_get_supported_extensions();
    let mut i = 0;
    // while !extensions.is_null() {
        let extension = CStr::from_ptr(extensions).to_str().unwrap();
        println!("Supported extension: {}", extension);
        i += 1;
    openmpt_free_string(extensions);


    // }
  }

  // test read mute status
  // unsafe {
  //   let status = get_channel_mute_status(module_ptr, 0);
  //   println!("Mute status for 0 : {}", status);
  // }

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
      openmpt_module_ext_get_interface(
          module_ptr,
          interface_id,
          &mut interface as *mut OpenMptModuleExtInterfaceInteractive as *mut c_void,
          interface_size
      )
  };
  if result != 0 {
      panic!("Failed to get interface");
  }

  println!("Hasil getInterface (1=success, 0=interface not found) : {}", result);
  println!("hasil out : {:?}", interface.stop_note);
  println!("interface size : {:?}", interface_size);
  println!("interface id : {:?}", interface_id_bind);



//   Construct the extended interface structure
  let mut ext_interface_ptr: OpenMPTModuleExt = ptr::null_mut();
  let ext_interface_void_ptr: *mut *mut c_void = &mut ext_interface_ptr as *mut *mut c_void;
//   let result = unsafe { openmpt_module_ext_create(module_ptr, ext_interface_void_ptr) };
//   if result < 0 {
//       panic!("Failed to construct extended interface");
//   }
  

  // Use the extended interface
//   if !ext_interface_ptr.is_null() {
      // Call an extension function
    //   let metadata_get_key_ptr = unsafe {
    //       std::mem::transmute::<
    //           *mut c_void,
    //           unsafe extern "C" fn(OpenMPTModule, *const c_char) -> *const c_char,
    //       >((*ext_interface_ptr).openmpt_module_ext_metadata_get_key)
    //   };
    //   let key = CString::new("title").unwrap();
    //   let value_ptr = unsafe { metadata_get_key_ptr(module_ptr, key.as_ptr()) };
    //   if !value_ptr.is_null() {
    //       let value = unsafe { CStr::from_ptr(value_ptr) };
    //       println!("Title: {:?}", value);
    //   }
    // println!("HUH!");
//   }
  println!("ADUH!");

  // Clean up
  unsafe {
    //   openmpt_module_ext_destruct(ext_interface_void_ptr);
    //   openmpt_module_destroy(module_ptr);
  }

}


fn log_func(param: Logger) -> openmpt_sys::openmpt_log_func {
    match param {
        Logger::StdErr => Some(openmpt_sys::openmpt_log_func_default),
        Logger::None => Some(openmpt_sys::openmpt_log_func_silent)
    }
}