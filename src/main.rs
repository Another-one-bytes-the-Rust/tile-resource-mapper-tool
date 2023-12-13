pub mod tile_mapper {

    use std::collections::HashMap;
    use std::ops::Range;
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::runner::{Robot, Runnable};
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;

    pub struct TileMapper {}

    impl Tools for TileMapper {}

    type Coordinates = (usize, usize);
    type ContentQuantity = (Option<usize>, Option<Range<usize>>);

    impl TileMapper {

        /// The `collection` function stores the number of elements found in any tile discovered by the robot
        /// It returns a `HashMap` where `key` is the element searched and `value` is a vector of tuples,
        /// The tuple stores the coordinates of a tile in another tuple, and the number of elements contained in that tile

        fn collection(world: &World) -> Option<HashMap<Content, Vec<(Coordinates, ContentQuantity)>>> {

            // HashMap instantiation
            let mut object_list: HashMap<Content, Vec<(Coordinates, ContentQuantity)>> = HashMap::new();

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
            // returns the HashMap
            Some(object_list)
        }

        fn insert_in_map(tile: &Tile , list: &mut HashMap<Content, Vec<(Coordinates, ContentQuantity)>>, row: usize, col: usize) {

            /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`

            let coord: Coordinates = (row, col);
            let value: ContentQuantity = tile.content.get_value();
            let content = tile.content.clone();

            // if no tile with `content` is in the list, it creates a new entry with that keyword
            // otherwise coordinates and value are added to the already existing vector
            list.entry(content)
                .and_modify(|v| v.push((coord, value.clone())))
                .or_insert(vec![(coord, value)]);
        }

        fn find_closest(world: &World, robot: & impl Runnable) -> Coordinates {
            let map = TileMapper::collection(world);
            todo!()
        }

        fn find_most_loaded(world: &World) -> (Coordinates, ContentQuantity) {
            let map = TileMapper::collection(world);
            todo!()
        }
    }
}


fn main() {}