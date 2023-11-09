use std::time::Duration;

use crate::{
    event::{Event, Events},
    resources::{DeltaTime, EntityMap, GameMode, Ping, ProgramTime, TerrainMap},
};

pub struct State {
    world: apecs::World,
}

impl State {
    pub fn client() -> apecs::anyhow::Result<Self> {
        let state = Self::new(GameMode::Client)?;
        Ok(state)
    }

    pub fn server() -> apecs::anyhow::Result<Self> {
        let state = Self::new(GameMode::Server)?;
        Ok(state)
    }

    pub fn new(mode: GameMode) -> apecs::anyhow::Result<Self> {
        let mut world = apecs::World::default();
        world
            .with_default_resource::<DeltaTime>()?
            .with_default_resource::<ProgramTime>()?
            .with_default_resource::<TerrainMap>()?
            .with_default_resource::<EntityMap>()?
            .with_default_resource::<Ping>()?
            .with_resource(mode)?;

        Ok(Self { world })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.resource_mut::<DeltaTime>().0 = dt.as_secs_f32();
        self.resource_mut::<ProgramTime>().0 += dt.as_secs_f64();

        if let Err(e) = self.world.tick() {
            log::error!("{}", e);
        }
    }

    pub fn with_event<E: Event>(&mut self, name: &str) -> &mut Self {
        match self.world.set_resource::<Events<E>>(Events::default()) {
            Ok(world) => {
                self.world
                    .with_system(
                        format!("{}-update", name),
                        super::event::event_update_system::<E>,
                    )
                    .unwrap();
            },
            Err(e) => log::error!("Failed to add event system for {}: {}", name, e),
        }
        self
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

    pub fn program_time(&self) -> f64 {
        self.resource::<ProgramTime>().0
    }

    pub fn terrain(&self) -> &TerrainMap {
        self.resource::<TerrainMap>()
    }

    pub fn terrain_mut(&mut self) -> &mut TerrainMap {
        self.resource_mut::<TerrainMap>()
    }

    pub fn query<Q: apecs::IsQuery + 'static>(&mut self) -> apecs::QueryGuard<'_, Q> {
        self.world.query::<Q>()
    }

    pub fn ecs(&self) -> &apecs::World {
        &self.world
    }

    pub fn ecs_mut(&mut self) -> &mut apecs::World {
        &mut self.world
    }
}

pub fn print_system_schedule(world: &mut apecs::World) {
    let names = world.get_sync_schedule_names();
    log::debug!("System Schedule:");
    for (i, system) in names.iter().enumerate() {
        log::debug!("{}: {:?}", i, system);
    }
}
