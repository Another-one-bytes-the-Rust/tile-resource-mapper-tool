
# Tile Resource Mapper Tool

The Tile Resource Mapper Tool is a Rust library that creates a Hashmap with all the Contents discovered by the robot, including  their quantity and coordinates. It is designed to work with the [robotics_lib](https://advancedprogramming.disi.unitn.it/crate?name=robotics_lib) library.

## Features

- Find the Tile, between the ones that have been discovered, that has the higher number of a specific Content. 
- Find the closest Tile that has at least one of the Content that we want.
- Handle errors, including cases where the robot has not discovered the specific Content we are looking for or any Tile of the world.

## Usage

```rust
// 'world' is a mutable reference to World
// 'self' is a mutable reference to the robot 
// 'robot' is a mutable reference of an object with the runnable trait
use tile_resource_mapper_tool::tool::tile_resource_mapper_tool{ContentQuantity, TileMapper};

// Create TileMapper instance
let mut mappertool = TileMapper {};

// Generate and access the HashMap
let map = TileMapper::collection(world);

// the format of 'content_to_search' is:
// Example number case:
let content_to_search = Content::Rock(n); // where n is any random number 
// Example range case:
let content_to_search = Content::Bin(n..m); // where n..m is any random range 

// get the closest tile with a specific content
let closest_coordinates = mappertool.find_closest(world, robot, content_to_search);

// get the most loaded tile with a specific content
let most_loaded_coordinates = mappertool.find_most_loaded(world, robot, content_to_search);
```

## Contributing

Feel free to actively contribute by opening GitHub issues to report problems, suggest enhancements, or discuss any aspect of the tool's development.
