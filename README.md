# Planet Explorer

An experimental program to explore Mars (and maybe others planets and moons in the future) made with the [Bevy](https://github.com/bevyengine/bevy) game engine.

The topographic data of mars is obtained from the [The MOLA Mission Experiment Gridded Data Records](https://pds-geosciences.wustl.edu/missions/mgs/megdr.html).

## How to use

Create a folder named `assets` and dowload the topographic data from [here](https://pds-geosciences.wustl.edu/mgs/mgs-m-mola-5-megdr-l3-v1/mgsl_300x/meg032/megt90n000fb.img) in it.

Then just run the apllication with:

```
cargo run --release
```

You can move the camera using the arrow keys and rotate clicking the right mouse button and dragging.
Pressing space increases the distance from the planet and left control decreases it.
