# Rust Invaders

So I decided to learn some more about Rust by making a video game. It might end
up being something like Space Invaders. Or Asteroids. Or Centipede. Or Missile
Command.

You know what? I just don't really know, but it'll be a mess.

## TODO

- [ ] Add text centering to font drawing

- [ ] Asteroid sizes that break up & spawn more on despawn
    - [ ] 1 big -> 2 medium
    - [ ] 1 medium -> 4 small
    - [ ] 4 small -> none
    - [ ] 1 giant slow mover -> spawns other sizes as it's damaged

- [ ] Asteroids should do variable damage based on size

- [ ] Goals - score?
    - [ ] Planet limb below, has health & takes damage from asteroids, don't let it die
    - [ ] Survive time limit?
    - [ ] High score?

- [ ] Particle splash when asteroid hits planet

- [ ] Shield shimmer when asteroid hits player

- [ ] Dramatic explosion on player or planet death

- [ ] Rework tags component with some utility methods - e.g. no more tags.0.contains()

- [ ] Rework tags component to use Enum rather than arbitrary strings

- [ ] Bullet entities should identify owner source for damage

- [ ] Implement despawn reasons and different tombstones (or lack thereof) based on reason
    - [ ] Asteroids should explode when shot, but just vanish when off playfield

- [ ] Black out area of screen outside playfield?

- [ ] Animated sprite meshes

- [ ] Switch ThrusterSet from string indexing to enum indexing?

- [ ] Methods for adding events to damage & despawn event queues, rather than
      manipulating those vecs directly

- [ ] Debug HUD - easy way to monitor some variables, offer some switches & knobs

- [ ] Proportional throttle control tied to controller joystick axes

- [ ] Sound effects in a bleepy bloopy style

- [ ] Pause menu

- [ ] Settings menu - resolution, keybindings, etc

- [ ] Index Position components in a quadtree for collision queries & etc

- [ ] Figure out how to build & distribute binaries of this thing

- [ ] Get this thing working in a browser with WASM someday
