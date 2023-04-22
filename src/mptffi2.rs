use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;

use openmpt::module::Logger;
use openmpt_sys::{openmpt_module, openmpt_module_create_from_memory, openmpt_log_func_silent};


type OpenMPTModule = *mut openmpt_module;
type OpenMPTModuleExt = *mut c_void;

#[repr(C)]
struct ModuleExt {
    openmpt_module_ext_metadata_get_key: extern "C" fn(
        *mut openmpt_module,
        *const c_char,
    ) -> *const c_char,
    // ...
}

#[link(name = "openmpt")]
extern "C" {
    fn openmpt_module_ext_create(module: OpenMPTModule, ext: *mut *mut c_void) -> i32;
    fn openmpt_module_ext_metadata_get_key(module: OpenMPTModule, key: *const c_char) -> *const c_char;
    // fn openmpt_module_ext_destruct(ext: *mut *mut c_void);
    fn openmpt_module_destroy(module: OpenMPTModule);
}

pub fn proses() {
  // Load module from memory
  let module_data = include_bytes!("..\\examples\\expr.it");
  let module_ptr = unsafe {
      openmpt_module_create_from_memory(
          module_data.as_ptr() as *const c_void,
          module_data.len(),
          log_func(Logger::StdErr),
          ptr::null_mut(),
          ptr::null_mut(),
      )
  };
  if module_ptr.is_null() {
      panic!("Failed to load module");
  }

  if !module_ptr.is_null() {
    println!("Module is loaded, whats next?");
  }

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