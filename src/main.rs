pub mod tile_mapper {

    use std::collections::HashMap;
    use std::ops::Range;
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;

    pub struct TileMapper {}

    impl Tools for TileMapper {}

    impl TileMapper {

        /// The `collection` function stores the number of elements found in any tile discovered by the robot
        /// It returns a `HashMap` where `key` is the element searched and `value` is a vector of tuples,
        /// The tuple stores the coordinates of a tile in another tuple, and the number of elements contained in that tile

        fn collection(&self, world: &World) -> Option<HashMap<Content, Vec<((usize, usize) , (Option<usize>, Option<Range<usize>>))>>> {

            // HashMap instantiation
            let mut object_list: HashMap<Content, Vec<((usize, usize) , (Option<usize>, Option<Range<usize>>))>> = HashMap::new();

            // check whether the world has been already discovered or not
            match robot_map(&world) {
                None => {return None;},
                Some(robot_world) => {
                    let mut row = 0;

                    // iterate through every tile in the world

                    for rows in robot_world.iter() {
                        let mut col = 0;
                        for element in rows.iter() {
                            //check if the tile has been discovered
                            match element {
                                None => {}
                                Some(tile) => {
                                    // call the `insert_in_map` function defined below
                                    TileMapper::insert_in_map(tile, &mut object_list, row, col)
                                }
                            }
                            col += 1;
                        }
                        row += 1;
                    }
                }
            }
            // returns the HashMap
            Some(object_list)
        }

        fn insert_in_map(tile: &Tile , list: &mut HashMap<Content, Vec<((usize, usize) , (Option<usize>, Option<Range<usize>>))>>, row: usize, col: usize) {

            /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`

            let coord = (row, col);
            let value = tile.content.get_value();
            let content = tile.content.clone();

            // if no tile with `content` is in the list, it creates a new entry with that keyword
            // otherwise coordinates and value are added to the already existing vector

            if !list.contains_key(&content) {
                let mut vec: Vec<((usize, usize) , (Option<usize>, Option<Range<usize>>))> = Vec::new();
                vec.push((coord, value));
                list.insert(content, vec);
            }
            else if list.contains_key(&content) {
                if let Some(v) = list.get(&content) {
                    let mut vec = v.clone();
                    vec.push((coord, value));
                    list.insert(content, vec);
                }
            }
        }
    }
}


fn main() {
    println!("PLEASE BUY ME UNCLE PEAR :)")
}