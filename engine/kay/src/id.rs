use super::type_registry::ShortTypeId;

/// An ID that uniquely identifies an `Actor`, or even a `Actor` within a `Swarm`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID {
    /// An ID for the type of the identified `Actor`, used to dispatch messages
    /// to the message handling functions registered for this type
    pub type_id: ShortTypeId,
    /// For future use: ID for the machine in a computing cluster
    /// or multiplayer environment that the identified `Actor` lives on
    pub machine: u8,
    /// For future use: allows safe reuse of an ID after `Actor`/`Actor` death.
    /// The version is incremented to make the new (otherwise same) ID distinguishable
    /// from erroneous references to the `Actor`/`Actor` previously identified
    pub version: u8,
    /// Used to identify instances within a top-level `Actor`. The main use-case is
    /// `Swarm` identifying and dispatching to its `Instances` using this field
    pub instance_id: u32,
}

pub fn broadcast_instance_id() -> u32 {
    u32::max_value()
}

pub fn broadcast_machine_id() -> u8 {
    u8::max_value()
}

impl ID {
    /// Create a new ID
    pub fn new(type_id: ShortTypeId, instance_id: u32, machine: u8, version: u8) -> Self {
        ID {
            type_id: type_id,
            machine: machine,
            version: version,
            instance_id: instance_id,
        }
    }

    /// Get a version of an actor ID that signals that a message
    /// should be delivered to all machine-local instances.
    pub fn local_broadcast(&self) -> ID {
        ID {
            instance_id: broadcast_instance_id(),
            ..*self
        }
    }

    /// Get a version of an actor ID that signals that a message
    /// should be delivered globally (to all instances on all machines).
    pub fn global_broadcast(&self) -> ID {
        ID {
            machine: broadcast_machine_id(),
            ..self.local_broadcast()
        }
    }

    /// Check whether this ID signals a local or global broadcast.
    pub fn is_broadcast(&self) -> bool {
        self.instance_id == broadcast_instance_id()
    }

    /// Check whether this ID signals specifically a global broadcast.
    pub fn is_global_broadcast(&self) -> bool {
        self.machine == broadcast_machine_id()
    }
}

impl ::std::fmt::Debug for ID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}_{}.{}@{}",
            u16::from(self.type_id),
            self.instance_id,
            self.version,
            self.machine,
        )
    }
}
