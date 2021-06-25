pub mod sessions;
pub mod index;
pub mod p_var;

pub use index::Index;
pub use p_var::PVar;
use uuid::Uuid;

// ------ ActorId ------

#[derive(Debug, Clone, Copy)]
pub struct ActorId(Uuid);

impl ActorId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

// ------ Actor ------

pub trait Actor {
    const KEY: &'static str;

    fn new_actor_id() -> ActorId {
        ActorId::new()
    }

    fn actor_id(&self) -> ActorId;

    fn revive(actor_id: ActorId) -> Self;

    fn remove(&self);
}