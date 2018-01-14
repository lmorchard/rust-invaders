# Rust Invaders

So I decided to learn some more about Rust by making a video game. It might end
up being something like Space Invaders. Or Asteroids. Or Centipede. Or Missile
Command.

You know what? I just don't really know, but it'll be a mess.

## TODO

- [ ] Reorganize things into component/system/resource/etc plugin bundles?

- [ ] Stateful sprites, so the asteroids remember their shape rather than changing every frame

- [ ] Damage on collision from bullets, to destroy asteroids and baddies

- [ ] Components should have ::new() constructors? Or implement Default trait

- [ ] Switch ThrusterSet from string indexing to enum indexing?

- [ ] Weapons

- [ ] Baddies

- [ ] Proportional throttle control tied to controller joystick axes

- [ ] Screen shake transforms for visual juice

- [ ] Sound effects in a bleepy bloopy style

- [ ] Pause menu

- [ ] Settings menu - resolution, keybindings, etc

- [ ] Index Position components in a quadtree for collision queries & etc
