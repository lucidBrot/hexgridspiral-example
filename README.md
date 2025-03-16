This is an example for the [hexgridspiral](https://github.com/lucidBrot/hexgridspiral) rust crate.
It is a separate repository because it has many additional dependencies (for the GUI).

See the start of `src/main.rs` for an overview of the examples included.

The webpage to showcase it will be available under https://lucidbrot.github.com/hexgridspiral-eample/

## Usage

* Every tile shows its spiral index centered on the tile.
* Every tile has the cube coordinates displayed further on the outside.
    * Top-Left: q
      The q-coordinate remains constant as you move along that diagonal.
    * Right: r
      The r-coordinate remains constant as you move along that horizontal.
    * Bottom-Left: s
      The s-coordinate remains constant as you move along that diagonal.
* When you **click** a tile  
    * The Movement Range with distance 2 is coloed yellow.
    * The tiles that can be reached by moving straight in one direction are darkened.
    * (Other colored tiles from the past get reset)
* When you hover a tile
    * Its color gets set to light blue
* When you unhover a tile
    * Its color gets reset to blue
