use glfw::ffi::{glfwInit};

pub struct GLFW {}

impl GLFW {
    pub fn new() -> GLFW {
        unsafe {
            glfwInit();
            return GLFW {}
        }
    }
}
