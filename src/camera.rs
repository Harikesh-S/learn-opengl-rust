/// Camera class that implements a Free cam

// Notes: 
// camera's matrices are only calculated if necessary, and the shader is updated only if the matrices are
// Not sure if this makes any impact to performance, but technically it should result in less updates to the gpu

// update_cam_matrix does calculcate the projection matrix every frame, but this can be changed by screen or user
// input. Not sure if its worth it to spearate it and only update if necessary (aspect, fov change)

use glfw;
use nalgebra_glm as glm;
use std::ffi::CStr;
use crate::shader::Shader;

pub trait Camera {
    // Create and init a new camera
    fn new(position : glm::Vec3, roll : f32, pitch : f32, yaw : f32,  width: u32, height : u32) -> Self;
    // Handle window events
    fn handle_window_event(&mut self, event : &glfw::WindowEvent, time_delta : &f64);
    // Used to update the shader's camMatrix
    fn set_cam_matrix(&mut self, shader : &Shader);
    // Main update function
    fn update(&mut self, window : &mut glfw::PWindow, time_delta : f64);
    // Update camera's cam_matrix
    fn update_cam_matrix(&mut self, calc_dir : bool);
    // Update camera's direction
    fn update_cam_direction(&mut self);
    // Force update to shader once
    fn force_set_cam_matrix(&mut self, shader : &Shader);
}

/// Free Camera
/// updates are handled internally using camera.update
/// WASD to move, QE to rotate, Mouse to pitch/yaw while holding right click
/// Shift to speed up, scroll to zoom in/out
/// Limitation: Cannot pitch/yaw when roll != 0. since the mouse input is added to pitch/yaw and does not account for the current roll
pub struct FreeCamera {
    // Current Screen Info
    pub width : u32,
    pub height : u32,

    // Current Position and Orientation
    pub position : glm::Vec3, 
    roll : f32,
    pitch : f32,
    yaw : f32,

    // Matrices used to send data to the shader
    pub direction : glm::Vec3,
    pub up : glm::Vec3,
    cam_matrix : glm::Mat4, // projection * view

    // Movement speed
    pub speed : f32,
    shift_multiplier : f32,
    roll_speed : f32,
    sensitivity : f32,
    zoom_sensitivity : f32,

    // Perspective
    fov : f32,
    pub near_plane : f32,
    pub far_plane : f32,

    // Internal flags
    first_click : bool,             // is this the first click?
    calculate_cam_matrix : bool,    // should matrix be recalculated?
    pub is_matrix_updated : bool        // should matrix be updated in the shader?
    
    // all of these are private since its updated using camera.update()
    // but these could be made public/getset to update them in the main loop
    // or with a sepratae event handler
}

impl Camera for FreeCamera {

    // Returns a fly camera with all fields initialized
    fn new(position : glm::Vec3, roll : f32, pitch : f32, yaw : f32, width : u32, height : u32) -> FreeCamera {
        let mut cam = FreeCamera{
            position : position,
            roll : roll,
            pitch : pitch,
            yaw : yaw,
            width : width,
            height : height,
            ..Default::default()
        };
        cam.update_cam_matrix(true);
        cam.calculate_cam_matrix = false;
        cam
    }
    
    /// Function to handle all window events
    /// Resize window, scroll etc. This can be used for direction as well
    /// Note : framebuffer and scroll polling has to be enabled
    fn handle_window_event(&mut self, event : &glfw::WindowEvent, time_delta : &f64) {
        match event {
            // Update width/height when frame buffer changes
            glfw::WindowEvent::FramebufferSize(width, height) => {
                self.width = *width as u32;
                self.height = *height as u32;
                self.calculate_cam_matrix = true;
            },

            // Zoom in/out when user scrolls
            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                self.fov -= *y_offset as f32 * *time_delta as f32 * self.zoom_sensitivity;
                if self.fov <= 1.0 { self.fov = 1.0; }
                if self.fov >= 90.0 { self.fov = 90.0; }
                self.calculate_cam_matrix = true;
            },
            
            // Could implement direction here
            _ => {}
        }
    }

    /// Function to set the camera's cam_matrix to the shader's camMatrix
    /// ! This activates the shader
    fn set_cam_matrix(&mut self, shader : &Shader) {

            // Calculate matrices if required
            if self.calculate_cam_matrix {
                self.update_cam_matrix(true);
            }

            // Set shader uniform if required
            if self.is_matrix_updated {
                unsafe {
                    shader.use_program();
                    shader.set_mat4(c_str!("camMatrix"), self.cam_matrix);
                }
        }
    }
    
    /// Function to handle all updates to the camera
    fn update(&mut self, window : &mut glfw::PWindow, time_delta : f64) {
        let mut update_speed = self.speed * time_delta as f32;
        let mut update_roll_speed = self.roll_speed * time_delta as f32;
        let update_sensitivty = self.sensitivity * time_delta as f32;

        // Resetting flag here to allow callling set_cam_matrix for multiple shaders
        self.is_matrix_updated = false;
        self.calculate_cam_matrix = false;

        // Shift - speed multiplier for position/roll
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            update_speed *= self.shift_multiplier;
            update_roll_speed *= self.shift_multiplier;
        }

        // Debug - Print current position
        if window.get_key(glfw::Key::P) == glfw::Action::Press && window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            println!("Position: {:?}, Pitch: {}, Roll: {}, Yaw: {}", self.position, self.pitch, self.roll, self.yaw);
        }

        // Direction - pitch and yaw from mouse

        // This can be done using setMouseCallback instead as it is in learnopengl
        // but this done with global variables in the example and im not sure how to do that
        // in rust, since the callback may outlive the main function

        // This can be done using events, but i'm leaving it here to refer lated
        // it would look exactly like the scroll part of the handler
        if window.get_mouse_button(glfw::MouseButtonRight) == glfw::Action::Press {
            // Reset roll, due to limitation - would have to account for current roll when changing pitch/yaw
            self.roll = 0.;

            // Do nothing on the first click
            if self.first_click {
                // Hide cursor
                window.set_cursor_mode(glfw::CursorMode::Hidden);
                // Reset cursor position
                window.set_cursor_pos(self.width as f64/2., self.height as f64/2.);
                self.first_click = false;
                return;
            }

            // Get mouse cursor position
            let (mouse_x, mouse_y) = window.get_cursor_pos();

            // Rotate pitch (up and dow) using cursor's y pos
            self.pitch -= update_sensitivty * (mouse_y as f32 - (self.height as f32 / 2.))/self.height as f32;

            // Rotate yaw (left and right) using cursor's x pos
            self.yaw += update_sensitivty * (mouse_x as f32 - (self.width as f32 / 2.))/self.width as f32;

            // Restricting pitch, since going beyond 90 inverts the mouse
            if self.pitch > 85. { self.pitch = 85. };
            if self.pitch < -85. { self.pitch = -85. };

            // Recalculate view matrix and update camera's direction vectors
            self.calculate_cam_matrix = true;

            // Reset cursor back to center of screen
            window.set_cursor_pos(self.width as f64/2., self.height as f64/2.);
        }
        if window.get_mouse_button(glfw::MouseButtonRight) == glfw::Action::Release {
            // Show cursor
            window.set_cursor_mode(glfw::CursorMode::Normal);
            self.first_click = true;
        }

        // Direction - roll from keyboard inputs
        if window.get_key(glfw::Key::Q) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.roll -= update_roll_speed;
        }
        if window.get_key(glfw::Key::E) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.roll += update_roll_speed;
        }

        // Direction - calculate - only affected by mouse input, used by movement input
        if self.calculate_cam_matrix {
            self.update_cam_direction();
        }

        // Position - from keyboard inputs
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.position += update_speed * self.direction;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.position -= update_speed * self.direction;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.position -= update_speed * glm::normalize(&glm::cross(&self.direction, &self.up));
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.calculate_cam_matrix = true;
            self.position += update_speed * glm::normalize(&glm::cross(&self.direction, &self.up));
        }

        // Calculate view and projection matrices
        if self.calculate_cam_matrix {
            self.update_cam_matrix(false); // not calculating dir again
        }
    }

    /// Function to update the camera's direction
    fn update_cam_direction(&mut self) {
        // Calculating direction from euler angles
        self.direction = glm::normalize(
            &glm::vec3(
                f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch)),
                f32::sin(f32::to_radians(self.pitch)),
                f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch))));

        // Calculating camera's up direction
        // using y vector for up, since its not updated (in this case)
        // self.up = glm::mat4_to_mat3(&glm::rotate(&glm::Mat4::identity(), f32::to_radians(self.roll), &self.direction))
        //      * glm::Vec3::y(); 
        self.up = glm::rotate_vec3(&glm::Vec3::y(), f32::to_radians(self.roll), &self.direction);
    }
    
    /// Function to update the camera's cam_matrix using the current state
    fn update_cam_matrix(&mut self, calc_dir : bool) {
        if calc_dir {
            self.update_cam_direction();
        }

        // Calculating view and projection matrices
        let view = glm::look_at_rh(&self.position, &(self.position + self.direction), &self.up); 
        let projection = glm::perspective(self.width as f32/self.height as f32, f32::to_radians(self.fov), self.near_plane, self.far_plane);

        self.cam_matrix = projection * view;
        self.is_matrix_updated = true;
        self.calculate_cam_matrix = false;
    }

    fn force_set_cam_matrix(&mut self, shader : &Shader) {
        self.calculate_cam_matrix = true;
        self.set_cam_matrix(shader);
        self.calculate_cam_matrix = false;
    }
}

impl Default for FreeCamera {
    fn default() -> FreeCamera {
        FreeCamera {
            position : glm::vec3(0., 0., 0.),
            roll : 0.,
            pitch : 0.,
            yaw : -90.,

            speed : 1.,
            shift_multiplier : 5.,
            roll_speed : 25.,
            sensitivity : 2000.,
            zoom_sensitivity : 100.,

            width : 100,
            height : 100,
            near_plane : 0.1,
            far_plane : 100.,
            fov : 45.,
            
            first_click: true,
            calculate_cam_matrix : true,
            is_matrix_updated : true,
            
            cam_matrix : glm::Mat4::identity(),
            direction : glm::vec3(0.,0.,0.),
            up : glm::vec3(0., 0., 0.),
        }
    }
}