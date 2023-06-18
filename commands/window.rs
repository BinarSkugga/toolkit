use std::ffi::CString;
use std::ptr::null_mut;
use glfw::ffi::{glfwCreateWindow, glfwDestroyWindow, glfwMakeContextCurrent, glfwPollEvents, glfwSetWindowSize, glfwSetWindowTitle, glfwShowWindow, glfwSwapBuffers, GLFWwindow, glfwWindowShouldClose};

#[allow(dead_code)]
pub struct Window {
    pub handle: *mut GLFWwindow,
    title: String,
    width: i32,
    height: i32
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Window {
        let c_title = CString::new(title).unwrap();
        let title_ptr = c_title.as_ptr();
        unsafe {
            let handle = glfwCreateWindow(width, height, title_ptr,
                                          null_mut(), null_mut());
            return Window {title: String::from(title), handle, width, height};
        };

    }

    #[allow(dead_code)]
    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = height;
        self.height = height;

        unsafe {
            glfwSetWindowSize(self.handle, width, height);
        }
    }

    #[allow(dead_code)]
    pub fn rename(&mut self, title: &str) {
        self.title = title.to_string();

        let c_title = CString::new(title).unwrap();
        let title_ptr = c_title.as_ptr();
        unsafe {
            glfwSetWindowTitle(self.handle, title_ptr);
        }
    }

    pub fn show(&self) {
        unsafe {
            glfwMakeContextCurrent(self.handle);
            glfwShowWindow(self.handle);
        }
    }

    pub fn run(&mut self) {
        unsafe {
            let mut i = 60;
            while glfwWindowShouldClose(self.handle) == 0 {
                i += 1;
                let mut new_title = String::from("MyTitle ");
                new_title.push_str(&i.to_string());
                self.rename(&new_title);

                glfwSwapBuffers(self.handle);
                glfwPollEvents();
            }
        }
    }

    pub fn destroy(&self) {
        unsafe {
            glfwDestroyWindow(self.handle);
        }
    }
}