
# Tile Resource Mapper Tool

The Tile Resource Mapper Tool is a Rust library that creates a Hashmap with all the Contents discovered by the robot, including  their quantity and coordinates. It is designed to work with the [robotics_lib](https://advancedprogramming.disi.unitn.it/crate?name=robotics_lib) library.

## Features

- Find the Tile, between the ones that have been discovered, that has the higher number of a specific Content. 
- Find the closest Tile that has at least one of the Content that we want.
- Handle errors, including cases where the robot has not discovered the specific Content we are looking for or any Tile of the world.

## Usage

```rust
use tile_resource_mapper_tool::tool::tile_resource_mapper_tool{ContentQuantity, TileMapper};

// Create TileMapper instance
let mut mappertool = TileMapper {};

// Generate and access the HashMap
let map = TileMapper::collection(world);

// 'world' is a mutable reference to World
// 'self' is a mutable reference to the robot 

// get the closest tile with a specific content
let closest_coordinates = mappertool.find_closest(world, self, content_to_search);

// get the most loaded tile with a specific content
let most_loaded_coordinates = mappertool.find_most_loaded(world, self, content_to_search);

// the format of 'content_to_search' is:
// Example number case:
let content_to_search = Content::Rock(n); // where n is any random number 
// Example range case:
let content_to_search = Content::Bin(n..m); // where n..m is any random range 

```

## Examples

```rust
// Test Tool Hashmap
let mut expected_results: HashMap<Discriminant<Content>,Vec<(MapCoordinate, ContentQuantity)>,> = HashMap::new();
    expected_results.insert(mem::discriminant(&Rock(12)), v_rocks);
       let result = TileMapper::collection(world);
                        println!("{:?}", res);
                        assert_eq!(res, expected_results);
// Test most loaded Tile
let expected_result = MapCoordinate::new(3, 1);

                match tool.find_most_loaded(world, self, Content::Rock(0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e),
                };

// Test closest Tile
let expected_result = MapCoordinate::new(3, 1);

                match tool.find_closest(world, self, Content::Rock(0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e),
                };
            }




```


## Contributing

Feel free to actively contribute by opening GitHub issues to report problems, suggest enhancements, or discuss any aspect of the tool's development.
