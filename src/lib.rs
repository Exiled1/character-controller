#[macro_use]
extern crate cimvr_engine_interface;

use cimvr_common::{
    desktop::{InputEvent, InputEvents, KeyboardEvent},
    glam::Vec3,
    render::{Mesh, MeshHandle, UploadMesh, Vertex},
    Transform,
};
use cimvr_derive_macros::ComponentDerive;
use cimvr_engine_interface::FrameTime;
pub use cimvr_engine_interface::{
    prelude::{UserState as PluginEntry, *},
    println as log,
};
// use cimvr_engine_interface::{prelude::*};

// All state associated with client-side behaviour
struct ClientState;

// All state associated with server-side behaviour
struct ServerState;

// Defines entry points for the engine to hook into.
make_app_state!(ClientState, ServerState);

// Stuff for the engine to use ^^^

// Stuff for me to use vvv

// Calls new() for the appropriate state.
// Step 1: Upload mesh data
const CUBE_HANDLE: MeshHandle = MeshHandle::new(pkg_namespace!("Cube"));

// #[derive(ComponentDerive, Serialize, Deserialize, Clone, Copy, Default)]
// struct CharacterTransform3d {
//     transform: Transform,
//     scale: f32, // Maybe later: https://en.wikipedia.org/wiki/Scale_(geometry)
// }

// #[derive(Serialize, Deserialize)]
// struct CharacterTransform3dRemote(CharacterTransform3d);
//
// impl Message for CharacterTransform3d {
//     const CHANNEL: ChannelIdStatic = ChannelIdStatic {
//         id: pkg_namespace!("CharacterTransform3d"),
//         locality: Locality::Remote,
//     };
// }

#[derive(Serialize, Deserialize)]
struct TransRemote(Transform);

#[derive(ComponentDerive, Serialize, Deserialize, Default, Copy, Clone)]
struct Scale(Vec3);

impl PluginEntry for ClientState {
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

        // Add the transform component to the cube mesh
        let cube_entity = io.create_entity();
        // Add the transform component to the cube mesh
        io.add_component(cube_entity, &Transform::default());
        io.add_component(cube_entity, &Scale::default());
        Self
    }
}

impl ClientState {
    // Make it so that the client state is added as a system to the schedule
    fn update(&mut self, io: &mut EngineIo, _query: &mut QueryResult) {
        //
        // TODO: WASD translates to changing the transform
        // TODO: Send transform as message.
        let frame_time = io.inbox_first::<FrameTime>().unwrap();
        for item in io
            .inbox::<InputEvents>()
            .map(|event| {
                event
                    .keyboard_events()
                    .cloned()
                    .collect::<Vec<KeyboardEvent>>()
            })
            .into_iter()
        {}
    }
}

impl PluginEntry for ServerState {
    // Implement a constructor
    fn new(_io: &mut EngineIo, _sched: &mut EngineSchedule<Self>) -> Self {
        // let cube_entity = _io.create_entity();

        log!("Hello, server!");
        Self
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
    // Return the mesh data
    UploadMesh {
        mesh: Mesh { vertices, indices },
        id: CUBE_HANDLE,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn im_a_test() {}
}
