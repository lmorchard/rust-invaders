# Rust Invaders

So I decided to learn some more about Rust by making a video game. It might end
up being something like Space Invaders. Or Asteroids. Or Centipede. Or Missile
Command.

You know what? I just don't really know, but it'll be a mess.

## TODO

- [ ] Reorganize things into component/system/resource/etc plugin bundles?
    - [ ] Collect sprite stuff into its own rendering plugin

- [ ] Switch ThrusterSet from string indexing to enum indexing?

- [ ] Methods for adding events to damage & despawn event queues, rather than manipulating those vecs directly

- [ ] Weapons

- [ ] Baddies

- [ ] Proportional throttle control tied to controller joystick axes

- [ ] Screen shake transforms for visual juice

- [ ] Sound effects in a bleepy bloopy style

- [ ] Pause menu

- [ ] Settings menu - resolution, keybindings, etc

- [ ] Index Position components in a quadtree for collision queries & etc
