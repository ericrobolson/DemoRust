# Map Generation

- [x] Implement mapping of numbers from one range to another
- [x] Implement RNG using u32s
- [x] Implement basic Grid container
- [x] [Noise Grid](https://dens.website/articles/procedural-generation/grid-basics)
- [x] [Perlin Noise generator](https://dens.website/articles/procedural-generation/perlin-noise)
- [x] [Fractal noise](https://dens.website/articles/procedural-generation/fractal-noise)
- [x] Add in u64 hashing of &'a strs and pass to command line then render world. Use this for seeds.

# Fractals

- [ ] Implement fractals
      https://www.youtube.com/watch?v=svLzmFuSBhk&t=0s&ab_channel=CodeParade
- [ ] https://www.reddit.com/r/videos/comments/adwdbk/this_guy_made_a_video_game_out_of_his_game_engine/
- [ ] Make a 3d game with portals that uses fractals

# Soft Bodies

Implement a soft body simulator purely with springs and spheres. This could easily be added into a Verlet integrator

# Complex generation

Implement some extras to make generation more complex.

- [ ] https://www.gamedeveloper.com/programming/a-real-time-procedural-universe-part-one-generating-planetary-bodies

- [ ] Simplify noise api
- [ ] [Poison disk](http://devmag.org.za/2009/05/03/poisson-disk-sampling/)
- [ ] [Generating terrain](https://www.redblobgames.com/maps/terrain-from-noise/)
- [ ] [Fractals w space](http://blog.hvidtfeldts.net/index.php/2011/08/distance-estimated-3d-fractals-iii-folding-space/)
- [ ] [SDFs](https://iquilezles.org/articles/distfunctions/)
- [ ] [Simulate world generation](https://gamedev.stackexchange.com/a/186197)
- [ ] Goal is a 2d image of an overworld. This can then be translated from pixels to Voxels
- [ ] Add in caves?
- [ ] Make entire world

# Gods

Create a 'deity' generator. This can inspire religous wars, miracles, etc.

# Mechanics

- Portal gun
- Climbing like in Breath of the Wild

# Profiling

- [ ] Implement custom allocators
- [ ] Convert to no_std for all crates
- [ ] Maybe pass in allocators to anything that requires allocating?
