# Chestnut

Some sort of ~~strategy~~ city-building game I guess (I need to make up my mind)

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

## Some sort of TODOs

- [x] Buildings
  - [x] separate construction sprites
  - [ ] construction highlight
- [x] map generation (tiles)
  - [ ] Infinite map generation? (Or at least larger map generation)
  - [ ] Resource generation (trees/rocks/etc...)
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
- [x] camera system
  - [x] camera controls
  - [x] switch all positioning to float based?
  - [ ] camera scaling
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
- [ ] keyboard shortcuts
- [ ] game UI
  - [ ] selected entity info
- [ ] Profiler