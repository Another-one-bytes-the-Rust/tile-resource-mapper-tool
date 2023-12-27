pub mod tile_mapper {
    use std::collections::HashMap;
    use std::error::Error;
    use std::ops::Range;
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::runner::{Runnable};
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError::{ContentNotDiscovered, WorldNotDiscovered};
    use std::mem::{Discriminant, discriminant};
    use robotics_lib::world;


    pub struct TileMapper {}

    impl Tools for TileMapper {}

    pub(crate) type ContentQuantity = (Option<usize>, Option<Range<usize>>);



    impl TileMapper {

        /// The `collection` function stores the number of elements found in any tile discovered by the robot
        /// It returns a `HashMap` where `key` is the element searched and `value` is a vector of tuples,
        /// The tuple stores the coordinates of a tile in another tuple, and the number of elements contained in that tile
        /// The usage of `mem::Discriminant<T>` allows to store in the hashmap
        /// tiles with different `Contents` quantity within the same key

        pub fn collection(world: &World) -> Option<HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>>> {

            // HashMap instantiation
            let mut object_list: HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>> = HashMap::new();

            // check whether the world has been already discovered or not
            match robot_map(&world) {
                None => { return None; },
                Some(robot_world) => {

                    // iterate through every tile in the world

                    for (row, row_vector) in robot_world.iter().enumerate() {
                        for (column, element) in row_vector.iter().enumerate() {
                            match element {
                                None => {}
                                Some(tile) => {
                                    // call the `insert_in_map` function defined below if content was found in the tile
                                    if tile.content != Content::None {
                                        TileMapper::insert_in_map(tile, &mut object_list, row, column)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // return the HashMap
            Some(object_list)
        }

        fn insert_in_map(tile: &Tile, list: &mut HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>>, row: usize, col: usize) {
            /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`
            let coord = (col, row);
            let value: ContentQuantity = tile.content.get_value();
            let content = discriminant(&tile.content);

            // if no tile with `content` is in the list, it creates a new entry with that keyword
            // otherwise coordinates and value are added to the already existing vector
            list.entry(content)
                .and_modify(|v| v.push((coord.into(), value.clone())))
                .or_insert(vec![(coord.into(), value)]);
        }

        pub fn find_closest(&self, world: &World, robot: &impl Runnable, content: Content) -> Result<MapCoordinate, Box<dyn Error>> {
            let hashmap = TileMapper::collection(world);
            match hashmap {
                Some(map) => {
                    // check if the hashmap contains the searched content
                    let cont = discriminant(&content);
                    return if map.contains_key(&cont) {
                        let vec = map.get(&cont);
                        let robot_coordinates = MapCoordinate::new(robot.get_coordinate().get_col(), robot.get_coordinate().get_row());

                        let mut closest_coordinates = MapCoordinate::new(100000,100000);
                        if let Some(v) = vec {
                            // iterate through the vector and search for the closest tile
                            for element in v.iter() {
                                // search for the smallest distance between the tiles and the robot
                                let old_distance = closest_coordinates.get_distance(&robot_coordinates);
                                let new_distance = element.0.get_distance(&robot_coordinates);
                                if new_distance < old_distance {
                                    closest_coordinates = element.0;
                                }
                            }
                        }
                        Ok(closest_coordinates)
                    } else {
                        Err(Box::new(ContentNotDiscovered))
                    }
                }
                None => Err(Box::new(WorldNotDiscovered))
            }
        }

        pub fn find_most_loaded(&self, world: &World, robot: &impl Runnable, content: Content) -> Result<MapCoordinate, Box<dyn Error>> {
            let hashmap = TileMapper::collection(world);
            // check if the world has already been discovered
            match hashmap {
                Some(map) => {
                    // check if the hashmap contains the searched content
                    let cont = discriminant(&content);
                    return if map.contains_key(&cont) {
                        let vec = map.get(&cont);
                        let mut coordinates = MapCoordinate::new(0, 0);
                        if let Some(v) = vec {

                            // instantiate quantity and range
                            let mut quantity: usize = 0;
                            let mut range: Range<usize> = 0..0;
                            // iterate through the vector and search for the most loaded tile
                            for content_info in v.iter() {

                                // get and set coordinates of the last discovered tile with the higher amount of content
                                // and if two tiles have the same quantity, set the closest tile
                                match &content_info.1 {
                                    (Some(q), None) => {
                                        if q > &quantity {
                                            quantity = q.clone();
                                            coordinates.set_width(content_info.0.get_width());
                                            coordinates.set_height(content_info.0.get_height());
                                        }
                                        else if q == &quantity {
                                            let width = robot.get_coordinate().get_col();
                                            let height = robot.get_coordinate().get_row();
                                            let robot_coordinates = MapCoordinate::new(width, height);
                                            let old_distance = coordinates.get_distance(&robot_coordinates);
                                            let new_distance = content_info.0.get_distance(&robot_coordinates);
                                            if new_distance < old_distance {
                                                quantity = q.clone();
                                                coordinates.set_width(content_info.0.get_width());
                                                coordinates.set_height(content_info.0.get_height());
                                            }
                                        }
                                    },
                                    (None, Some(r)) => {
                                        if r.clone().len() > range.len() {
                                            range = r.clone();
                                            coordinates.set_width(content_info.0.get_width());
                                            coordinates.set_height(content_info.0.get_height());
                                        } else if r.clone().len() == range.len() {
                                            let width = robot.get_coordinate().get_col();
                                            let height = robot.get_coordinate().get_row();
                                            let robot_coordinates = MapCoordinate::new(width, height);
                                            let old_distance = coordinates.get_distance(&robot_coordinates);
                                            let new_distance = content_info.0.get_distance(&robot_coordinates);
                                            if new_distance < old_distance {
                                                range = r.clone();
                                                coordinates.set_width(content_info.0.get_width());
                                                coordinates.set_height(content_info.0.get_height());
                                            }
                                        }
                                    },
                                    (_, _) => {}
                                }
                            }
                        }
                        Ok(coordinates)
                    } else {
                        Err(Box::new(ContentNotDiscovered))
                    }
                }
                None => Err(Box::new(WorldNotDiscovered))
            }
        }


        // pub fn collection(world: &World) -> Option<HashMap<ToolContent, Vec<(MapCoordinate, ContentQuantity)>>> {
        //
        //     // HashMap instantiation
        //     let mut object_list: HashMap<ToolContent, Vec<(MapCoordinate, ContentQuantity)>> = HashMap::new();
        //
        //     // check whether the world has been already discovered or not
        //     match robot_map(&world) {
        //         None => { return None; },
        //         Some(robot_world) => {
        //
        //             // iterate through every tile in the world
        //
        //             for (row, row_vector) in robot_world.iter().enumerate() {
        //                 for (column, element) in row_vector.iter().enumerate() {
        //                     match element {
        //                         None => {}
        //                         Some(tile) => {
        //                             // call the `insert_in_map` function defined below if content was found in the tile
        //                             if tile.content != Content::None {
        //                                 TileMapper::insert_in_map(tile, &mut object_list, row, column)
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     // return the HashMap
        //     Some(object_list)
        // }
        //
        // fn insert_in_map(tile: &Tile, list: &mut HashMap<ToolContent, Vec<(MapCoordinate, ContentQuantity)>>, row: usize, col: usize) {
        //     /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`
        //
        //     let coord = (col, row);
        //     let value: ContentQuantity = tile.content.get_value();
        //     let content = TileMapper::match_content(&tile.content);
        //
        //     // if no tile with `content` is in the list, it creates a new entry with that keyword
        //     // otherwise coordinates and value are added to the already existing vector
        //     list.entry(content)
        //         .and_modify(|v| v.push((coord.into(), value.clone())))
        //         .or_insert(vec![(coord.into(), value)]);
        // }
        //
        // pub fn find_closest(&self, world: &World, robot: &impl Runnable, content: Content) -> Result<MapCoordinate, Box<dyn Error>> {
        //     let hashmap = TileMapper::collection(world);
        //     match hashmap {
        //         Some(map) => {
        //             // check if the hashmap contains the searched content
        //             let cont = TileMapper::match_content(&content);
        //             return if map.contains_key(&cont) {
        //                 let vec = map.get(&cont);
        //                 let width = robot.get_coordinate().get_col();
        //                 let height = robot.get_coordinate().get_row();
        //                 let robot_coordinates = MapCoordinate::new(width, height);
        //                 let mut coordinates = MapCoordinate::new(0, 0);
        //                 if let Some(v) = vec {
        //                     // iterate through the vector and search for the closest tile
        //                     for (index, content_info) in v.iter().enumerate() {
        //                         if index == 0 {
        //                             // set first coordinates
        //                             coordinates.set_width(content_info.0.get_width());
        //                             coordinates.set_height(content_info.0.get_height());
        //                         } else {
        //                             // search for the smallest distance between the tiles and the robot
        //                             let old_distance = coordinates.get_distance(&robot_coordinates);
        //                             let new_distance = content_info.0.get_distance(&robot_coordinates);
        //                             if new_distance < old_distance {
        //                                 coordinates.set_width(content_info.0.get_width());
        //                                 coordinates.set_height(content_info.0.get_height());
        //                             }
        //                         }
        //                     }
        //                 }
        //                 Ok(coordinates)
        //             } else {
        //                 Err(Box::new(ContentNotDiscovered))
        //             }
        //         }
        //         None => Err(Box::new(WorldNotDiscovered))
        //     }
        // }
        //
        // pub fn find_most_loaded(&self, world: &World, robot: &impl Runnable, content: Content) -> Result<MapCoordinate, Box<dyn Error>> {
        //     let hashmap = TileMapper::collection(world);
        //     // check if the world has already been discovered
        //     match hashmap {
        //         Some(map) => {
        //             // check if the hashmap contains the searched content
        //             let cont = TileMapper::match_content(&content);
        //             return if map.contains_key(&cont) {
        //                 let vec = map.get(&cont);
        //                 let mut coordinates = MapCoordinate::new(0, 0);
        //                 if let Some(v) = vec {
        //
        //                     // instantiate quantity and range
        //                     let mut quantity: usize = 0;
        //                     let mut range: Range<usize> = 0..0;
        //                     // iterate through the vector and search for the most loaded tile
        //                     for content_info in v.iter() {
        //
        //                         // get and set coordinates of the last discovered tile with the higher amount of content
        //                         // and if two tiles have the same quantity, set the closest tile
        //                         match &content_info.1 {
        //                             (Some(q), None) => {
        //                                 if q > &quantity {
        //                                     quantity = q.clone();
        //                                     coordinates.set_width(content_info.0.get_width());
        //                                     coordinates.set_height(content_info.0.get_height());
        //                                 }
        //                                 else if q == &quantity {
        //                                     let width = robot.get_coordinate().get_col();
        //                                     let height = robot.get_coordinate().get_row();
        //                                     let robot_coordinates = MapCoordinate::new(width, height);
        //                                     let old_distance = coordinates.get_distance(&robot_coordinates);
        //                                     let new_distance = content_info.0.get_distance(&robot_coordinates);
        //                                     if new_distance < old_distance {
        //                                         quantity = q.clone();
        //                                         coordinates.set_width(content_info.0.get_width());
        //                                         coordinates.set_height(content_info.0.get_height());
        //                                     }
        //                                 }
        //                             },
        //                             (None, Some(r)) => {
        //                                 if r.clone().len() > range.len() {
        //                                     range = r.clone();
        //                                     coordinates.set_width(content_info.0.get_width());
        //                                     coordinates.set_height(content_info.0.get_height());
        //                                 } else if r.clone().len() == range.len() {
        //                                     let width = robot.get_coordinate().get_col();
        //                                     let height = robot.get_coordinate().get_row();
        //                                     let robot_coordinates = MapCoordinate::new(width, height);
        //                                     let old_distance = coordinates.get_distance(&robot_coordinates);
        //                                     let new_distance = content_info.0.get_distance(&robot_coordinates);
        //                                     if new_distance < old_distance {
        //                                         range = r.clone();
        //                                         coordinates.set_width(content_info.0.get_width());
        //                                         coordinates.set_height(content_info.0.get_height());
        //                                     }
        //                                 }
        //                             },
        //                             (_, _) => {}
        //                         }
        //                     }
        //                 }
        //                 Ok(coordinates)
        //             } else {
        //                 Err(Box::new(ContentNotDiscovered))
        //             }
        //         }
        //         None => Err(Box::new(WorldNotDiscovered))
        //     }
        // }

        // fn match_content(content: &Content) -> ToolContent {
        //     match content {
        //         | Content::Rock(_) => ToolContent::Rock,
        //         | Content::Tree(_) => ToolContent::Tree,
        //         | Content::Garbage(_) => ToolContent::Garbage,
        //         | Content::Fire => ToolContent::Fire,
        //         | Content::Coin(_) => ToolContent::Coin,
        //         | Content::Bin(_) => ToolContent::Bin,
        //         | Content::Crate(_) => ToolContent::Crate,
        //         | Content::Bank(_) => ToolContent::Bank,
        //         | Content::Water(_) => ToolContent::Water,
        //         | Content::Market(_) => ToolContent::Market,
        //         | Content::Fish(_) => ToolContent::Fish,
        //         | Content::Building => ToolContent::Building,
        //         | Content::Bush(_) => ToolContent::Bush,
        //         | Content::JollyBlock(_) => ToolContent::JollyBlock,
        //         | Content::Scarecrow => ToolContent::Scarecrow,
        //         | Content::None => ToolContent::None
        //     }
        // }
    }

    // `pub enum ToolContent` is basically a copy of `pub enum Content` of the library
    // but in this case we are not paying attention at the usize, to allow the hashmap
    // to store tiles with different `Contents` quantity within the same key

    // #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    // pub enum ToolContent {
    //     Rock,
    //     Tree,
    //     Garbage,
    //     Fire,
    //     Coin,
    //     Bin,
    //     Crate,
    //     Bank,
    //     Water,
    //     Market,
    //     Fish,
    //     Building,
    //     Bush,
    //     JollyBlock,
    //     Scarecrow,
    //     None,
    // }
}