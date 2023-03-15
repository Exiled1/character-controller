#[macro_use]
extern crate cimvr_engine_interface;

use cimvr_derive_macros::ComponentDerive;
use cimvr_engine_interface::FrameTime;
pub use cimvr_engine_interface::{prelude::*, println as prnt};
use cimvr_common::{render::{MeshHandle, Mesh, Vertex, UploadMesh}, Transform, desktop::InputEvents};
// use cimvr_engine_interface::{prelude::*};

// All state associated with client-side behaviour
struct ClientState;

// impl UserState for ClientState {
//     // Implement a constructor
//     fn new(_io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {
//         prnt!("Hello, client!");
        

//         // NOTE: We are using the println defined by cimvr_engine_interface here, NOT the standard library!
//         prnt!("This prints");
//         std::println!("But this doesn't");
        
//         Self
//     }
// }

// All state associated with server-side behaviour
struct ServerState;

impl UserState for ServerState {
    // Implement a constructor
    fn new(_io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {
        prnt!("Hello, server!");
        Self
    }
}

// Defines entry points for the engine to hook into.
make_app_state!(ClientState, ServerState);

// Stuff for the engine to use ^^^

// Stuff for me to use vvv


// Calls new() for the appropriate state.
// Step 1: Upload mesh data
const CUBE_HANDLE: MeshHandle = MeshHandle::new(pkg_namespace!("Cube")); 


#[derive(ComponentDerive, Serialize, Deserialize, Clone, Copy, Default)]
#[size(4)]
struct CharacterTransform3d{
    transform: Transform,
    // scale: f32, // Maybe later: https://en.wikipedia.org/wiki/Scale_(geometry)
}

impl UserState for ClientState {
    fn new(io: &mut EngineIo, sched: &mut EngineSchedule<Self>) -> Self {

        
        let cube_mesh = cube(); // Create an upload mesh
        io.send(&cube_mesh); // Upload it to the engine

        // Subscribe to Input events (keyboard, mouse, etc) and frame time
        // This makes it so our ClientState update function is called with the ability to receive these events.
        // SystemDescriptor seems to be a System Manager of some sort.
        let system_desc = SystemDescriptor::new(Stage::Update)
            .subscribe::<InputEvents>() // Subscribe to input events
            .subscribe::<FrameTime>(); // Subscribe to frame time for delta time
            
        sched.add_system(ClientState::update, system_desc); // Add the system to the schedule



        Self
    }
}

impl ClientState {
    // Make it so that the client state is added as a system to the schedule
    fn update(&mut self, io: &mut EngineIo, _query: &mut QueryResult) {
        
    }
}

/// Defines the mesh data for a cube
fn cube() -> UploadMesh {
    let size = 0.25;
    let vertices = vec![
        Vertex::new([-size, -size, -size], [0.0, 1.0, 1.0]),
        Vertex::new([size, -size, -size], [1.0, 0.0, 1.0]),
        Vertex::new([size, size, -size], [1.0, 1.0, 0.0]),
        Vertex::new([-size, size, -size], [0.0, 1.0, 1.0]),
        Vertex::new([-size, -size, size], [1.0, 0.0, 1.0]),
        Vertex::new([size, -size, size], [1.0, 1.0, 0.0]),
        Vertex::new([size, size, size], [0.0, 1.0, 1.0]),
        Vertex::new([-size, size, size], [1.0, 0.0, 1.0]),
    ];

    let indices = vec![
        3, 1, 0, 2, 1, 3, 2, 5, 1, 6, 5, 2, 6, 4, 5, 7, 4, 6, 7, 0, 4, 3, 0, 7, 7, 2, 3, 6, 2, 7,
        0, 5, 4, 1, 5, 0,
    ];

    UploadMesh { 
        mesh: Mesh {vertices, indices },
        id: CUBE_HANDLE,
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn im_a_test() {}
}
