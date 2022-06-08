use ash::vk;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry = ash::Entry::new()?;
    let instance_create_info = vk::InstanceCreateInfo {
        ..Default::default()
    };
    dbg!(&instance_create_info);
    let instance = unsafe {
        entry.create_instance(&instance_create_info, None)?
    };
    unsafe {
        instance.destroy_instance(None)
    };
    Ok(())
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    // *const T is similar to &T but *const T directly translates to C
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    // this is a callback function that is called in our validation layer
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE // should we skip the call to the driver?
}


