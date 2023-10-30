use std::time::Duration;

use crate::resources::{DeltaTime, TerrainMap};

pub type SysResult = apecs::anyhow::Result<apecs::ShouldContinue>;

pub struct State {
    world: apecs::World,
}

#[allow(clippy::new_without_default)]
impl State {
    pub fn new() -> apecs::anyhow::Result<Self> {
        let mut world = apecs::World::default();
        world.with_default_resource::<DeltaTime>()?;
        world.with_default_resource::<TerrainMap>()?;
        apecs::anyhow::Result::Ok(Self { world })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.resource_mut::<DeltaTime>().0 = dt.as_secs_f32();
        if let Err(e) = self.world.tick() {
            log::error!("failed to tick ecs: {:?}", e);
        }
    }

    pub fn resource<R: apecs::IsResource>(&self) -> &R {
        self.world
            .resource::<R>()
            .expect("Tried to fetch an invalid resource")
    }

    pub fn resource_mut<R: apecs::IsResource>(&mut self) -> &mut R {
        self.world
            .resource_mut::<R>()
            .expect("Tried to fetch an invalid resource")
    }

    pub fn fetch<T: apecs::CanFetch>(&mut self) -> T {
        self.world
            .fetch::<T>()
            .expect("Failed to fetch resource. This is most likely a bug")
    }

    pub fn query<Q: apecs::IsQuery + 'static>(&mut self) -> apecs::QueryGuard<'_, Q> {
        self.world.query::<Q>()
    }

    pub fn ecs_mut(&mut self) -> &mut apecs::World {
        &mut self.world
    }

    pub fn ecs(&self) -> &apecs::World {
        &self.world
    }
}
