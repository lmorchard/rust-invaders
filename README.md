# Rust Invaders

So I decided to learn some more about Rust by making a video game. It might end
up being something like Space Invaders. Or Asteroids. Or Centipede. Or Missile
Command.

You know what? I just don't really know, but it'll be a mess.

## TODO

- [ ] Black out area of screen outside playfield?

- [ ] Implement elastic collisions so asteroids bounce -
        http://en.m.wikipedia.org/wiki/Elastic_collision

- [ ] Animated sprite meshes

- [ ] Switch ThrusterSet from string indexing to enum indexing?

- [ ] Methods for adding events to damage & despawn event queues, rather than
      manipulating those vecs directly

- [ ] Debug HUD - easy way to monitor some variables, offer some switches & knobs

- [ ] Game HUD - score, shields, planet health, etc

- [ ] Weapons

- [ ] Baddies

- [ ] Goals - score?
    - [ ] Planet limb below, has health & takes damage from asteroids, don't let it die
    - [ ] Survive time limit?
    - [ ] High score?

- [ ] Player death

- [ ] Levels / level restart on death

- [ ] Proportional throttle control tied to controller joystick axes

- [ ] Screen shake transforms for visual juice

- [ ] Sound effects in a bleepy bloopy style

- [ ] Pause menu

- [ ] Settings menu - resolution, keybindings, etc

- [ ] Index Position components in a quadtree for collision queries & etc

- [ ] Figure out how to build & distribute binaries of this thing

- [ ] Get this thing working in a browser with WASM someday
