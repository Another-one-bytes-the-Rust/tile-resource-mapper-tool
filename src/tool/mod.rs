pub mod tile_mapper {

    use std::collections::HashMap;
    use std::error::Error;
    use std::ops::Range;
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::runner::{Runnable};
    use robotics_lib::utils::LibError;
    use robotics_lib::utils::LibError::NoContent;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError::{ContentNotDiscovered, WorldNotDiscovered};


    pub struct TileMapper {}

    impl Tools for TileMapper {}


    // type Coordinates = (usize, usize);
    type ContentQuantity = (Option<usize>, Option<Range<usize>>);


    impl TileMapper {

        /// The `collection` function stores the number of elements found in any tile discovered by the robot
        /// It returns a `HashMap` where `key` is the element searched and `value` is a vector of tuples,
        /// The tuple stores the coordinates of a tile in another tuple, and the number of elements contained in that tile

        pub fn collection(world: &World) -> Option<HashMap<Content, Vec<(MapCoordinate, ContentQuantity)>>> {

            // HashMap instantiation
            let mut object_list: HashMap<Content, Vec<(MapCoordinate, ContentQuantity)>> = HashMap::new();

            // check whether the world has been already discovered or not
            match robot_map(&world) {
                None => {return None;},
                Some(robot_world) => {

                    // iterate through every tile in the world

                    for (row,row_vector) in robot_world.iter().enumerate() {
                        for (column, element) in row_vector.iter().enumerate() {
                            match element {
                                None => {}
                                Some(tile) => {
                                    // call the `insert_in_map` function defined below
                                    TileMapper::insert_in_map(tile, &mut object_list, row, column)
                                }
                            }
                        }
                    }
                }
            }
            // return the HashMap
            Some(object_list)
        }

        fn insert_in_map(tile: &Tile , list: &mut HashMap<Content, Vec<(MapCoordinate, ContentQuantity)>>, row: usize, col: usize) {

            /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`

            let coord = (row, col);
            let value: ContentQuantity = tile.content.get_value();
            let content = tile.content.clone();

            // if no tile with `content` is in the list, it creates a new entry with that keyword
            // otherwise coordinates and value are added to the already existing vector
            list.entry(content)
                .and_modify(|v| v.push((coord.into(), value.clone())))
                .or_insert(vec![(coord.into(), value)]);
        }

        pub fn find_closest(world: &World, robot: & impl Runnable, content: Content) -> Result<MapCoordinate, LibError> {
            let map = TileMapper::collection(world);
            todo!()
        }

        pub fn find_most_loaded(world: &World, robot: & impl Runnable, content: Content) -> Result<MapCoordinate, Box<dyn Error>> {
            let map = TileMapper::collection(world);

            // check if the world has already been discovered
            match map {
                Some(Map) => {
                    // check if the hashmap contains the searched content
                    return if Map.contains_key(&content) {
                        let vec = Map.get(&content);
                        if let Some(v) = vec {

                            // instantiate some variables
                            let mut quantity: usize = 0;
                            let mut range: Range<usize> = 0..0;
                            let mut coordinates = MapCoordinate::new(0,0);
                            // iterate through the vector and search for the most loaded tile
                            for Content_Info in v.iter() {

                                // get and set coordinates of the last discovered tile with the higher amount of content
                                match &Content_Info.1 {
                                    (Some(q), None) => {
                                        if q >= &quantity {
                                            quantity = q.clone();
                                            coordinates.set_width(Content_Info.0.get_width());
                                            coordinates.set_height(Content_Info.0.get_height());
                                        }
                                    },
                                    (None, Some(r)) => {
                                        if r.clone().cmp(&mut range) == std::cmp::Ordering::Greater
                                            || r.clone().cmp(&mut range) == std::cmp::Ordering::Equal {
                                            range = r.clone();
                                            coordinates.set_width(Content_Info.0.get_width());
                                            coordinates.set_height(Content_Info.0.get_height());
                                        }
                                    },
                                    (_, _) => {}
                                }
                            }
                        }
                        Ok(MapCoordinate::new(0, 0))
                    } else {
                        Err(Box::new(ContentNotDiscovered))
                    }
                }
                None => Err(Box::new(WorldNotDiscovered))
            }
        }
    }
}