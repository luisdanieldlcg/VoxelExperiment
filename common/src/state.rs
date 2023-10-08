use std::time::Duration;

use apecs::IsResource;

use crate::resources::DeltaTime;

pub struct State {
    world: apecs::World,
}

#[allow(clippy::new_without_default)]
impl State {
    pub fn new() -> Self {
        let world = apecs::World::default();
        let mut this = Self { world };
        this.add_resource(DeltaTime::default());
        this
    }

    pub fn tick(&mut self, dt: Duration) {
        self.resource_mut::<DeltaTime>().0 = dt.as_secs_f32();
        if let Err(e) = self.world.tick() {
            log::error!("{}", e);
        }
    }

    pub fn add_resource<R: IsResource>(&mut self, resource: R) {
        self.world.with_resource::<R>(resource).expect(
            "Tried to add a resource that already exists. \
            This is a bug in the code",
        );
    }

    pub fn resource<R: IsResource>(&self) -> &R {
        self.world
            .resource::<R>()
            .expect("Tried to fetch an invalid resource")
    }

    pub fn resource_mut<R: IsResource>(&mut self) -> &mut R {
        self.world
            .resource_mut::<R>()
            .expect("Tried to fetch an invalid resource")
    }
}
