# Chestnut

Some sort of ~~strategy~~ city-building game I guess (I need to make up my mind)

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

## Some sort of TODOs

- [x] Buildings
  - [x] separate construction sprites
  - [ ] construction highlight
- [ ] map generation (tiles)
- [ ] resource system (game)
- [ ] UI system
  - [ ] load UI from json file (maybe? investigate)
  - [ ] scaling system for consistency
- [ ] Tech trees
- [x] Resource manager (engine)
  - [x] texture loading
  - [ ] font engine
  - [ ] get texture by path
- [x] Delta time management
- [ ] testing????
- [ ] collisions
  - [ ] seperate collision box vs 'selection' box
- [ ] camera system
- [ ] serialization
  - [ ] game state serialization
  - [ ] component serialization
- [x] rendering order
- [x] text rendering
- [ ] audio??
- [ ] rendering
  - [ ] consistent sprite placement based on position
  - [ ] camera system
  - [ ] sprite scaling (more low res sprites)