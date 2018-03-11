use specs::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Name>();
    world.register::<Tags>();
    dispatcher
}

#[derive(Component)]
pub struct Name(pub &'static str);

// TODO: Figure out how to switch from strings to an Enum while decoupling generic library from
// game logic?
#[derive(Component)]
pub struct Tags(pub Vec<&'static str>);
impl Tags {
    pub fn new(tags: Vec<&'static str>) -> Tags {
        Tags(tags)
    }
}
