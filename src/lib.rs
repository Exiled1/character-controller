#[macro_use]
extern crate cimvr_engine_interface;

use cimvr_common::{
    desktop::{ElementState, InputEvent, KeyCode, KeyboardEvent},
    glam::Vec3,
    render::{Mesh, MeshHandle, UploadMesh, Vertex},
    utils::input_helper::InputHelper,
    Transform,
};
use cimvr_derive_macros::Component;
use cimvr_engine_interface::FrameTime;
pub use cimvr_engine_interface::{
    prelude::{UserState as PluginEntry, *},
    println as log,
};
// use cimvr_engine_interface::{prelude::*};

// All state associated with client-side behaviour

#[derive(Default)]
struct ClientState {
    input: InputHelper,
}

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

#[derive(Message, Serialize, Deserialize)]
#[locality("Remote")]
struct RemoteTrans(Transform);

#[derive(Component, Serialize, Deserialize, Default, Copy, Clone)]
struct Scale(Transform);

#[derive(Component, Serialize, Deserialize, Default, Clone, Copy)]
struct Speed(f32);

#[derive(Component, Serialize, Deserialize, Default, Copy, Clone)]
struct Player;

impl PluginEntry for ClientState {
    fn new(io: &mut EngineIo, sched: &mut EngineSchedule<Self>) -> Self {
        let cube_mesh = cube(); // Create an upload mesh
        io.send(&cube_mesh); // Upload it to the engine

        // Subscribe to Input events (keyboard, mouse, etc) and frame time
        // This makes it so our ClientState update function is called with the ability to receive these events.
        // SystemDescriptor seems to be a System Manager of some sort.
        let system_desc = SystemDescriptor::new(Stage::Update)
            .subscribe::<InputEvent>() // Subscribe to input events
            .subscribe::<FrameTime>()
            .query::<Player>(Access::Read)
            .query::<Transform>(Access::Write) // Subscribe to frame time for delta time
            .query::<Speed>(Access::Write);
        sched.add_system(ClientState::update, system_desc); // Add the system to the schedule

        // Add the transform component to the cube mesh
        let character = io.create_entity();
        // Add the transform component to the cube mesh
        io.add_component(character, Transform::default());
        io.add_component(character, Scale::default());
        io.add_component(character, Player::default());
        io.add_component(character, Speed(10.));

        Self::default() // This works cuz default is baller
    }
}

impl ClientState {
    // Make it so that the client state is added as a system to the schedule
    fn update(&mut self, io: &mut EngineIo, query: &mut QueryResult) {
        self.input.handle_input_events(io);
        // TODO: WASD translates to changing the transform
        // TODO: Send transform as message.
        let frame_time = io.inbox_first::<FrameTime>().unwrap(); // Get frame time or bust.
                                                                 // EntityID for character.
        let character_entity = query.iter().next().unwrap();
        // Every frame the input helper is updated. So we should be good to just do wasd to
        // transform changing.
        let speed = query.read::<Speed>(character_entity);
        let mut local_transform = query.read::<Transform>(character_entity);
        let mut move_vector = Vec3::ZERO;
        const FORWARD: Vec3 = Vec3::new(1., 0., 0.);
        const BACKWARD: Vec3 = Vec3::new(-1., 0., 0.);
        const LEFT: Vec3 = Vec3::new(0., 0., -1.);
        const RIGHT: Vec3 = Vec3::new(0., 0., 1.);
        // Now that we have the character entity's transform, we can just read WASD and change it
        // based on the pressed key. We'll just use if statements since pattern matching won't help
        // us if there's multiple keys pressed like W+A which is perfectly valid to go horizontally
        if self.input.key_down(KeyCode::W) {
            // Go forward.
            move_vector += FORWARD;
        }
        if self.input.key_down(KeyCode::A) {
            // Go left.
            move_vector += LEFT;
        }
        if self.input.key_down(KeyCode::S) {
            // Go backwards.
            move_vector += BACKWARD;
        }
        if self.input.key_down(KeyCode::D) {
            // Go right.
            move_vector += RIGHT;
        }

        // Now that we know by how much we want to move. Lets apply that to our local transform and
        // send that to the server as a message. NOTE! We can also change this to work server side
        // by making it so that instead of modifying a local transform and sending that, we can
        // instead send our move vector as a message to the server. That would probably make better
        // use of the sync component as well.
        let distance_moved = move_vector.normalize() * frame_time.delta * speed.0;
        local_transform.pos += distance_moved;
        // Now we send our position over to the server
        io.send(&RemoteTrans(local_transform))
    }
}

impl PluginEntry for ServerState {
    // Implement a constructor&Player::default());
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
