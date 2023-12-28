pub mod tile_mapper {
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError::{ContentNotDiscovered, WorldNotDiscovered};
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::runner::Runnable;

    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;
    use std::collections::HashMap;
    use std::error::Error;
    use std::mem::{discriminant, Discriminant};
    use std::ops::Range;

    pub struct TileMapper {}

    impl Tools for TileMapper {}

    pub(crate) type ContentQuantity = (Option<usize>, Option<Range<usize>>);

    impl TileMapper {
        /// The `collection` function stores the number of elements found in any tile discovered by the robot and returns them in a hashmap.
        ///
        /// # Arguments
        ///
        /// * `world` - A reference to the `World`.
        ///
        /// # Returns
        ///
        /// Returns `Some(HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>>)` if the world has been discovered,
        /// the tuples store the coordinates of a tile and the number of elements contained in that tile.
        /// Returns `None` if the robot has not discovered anything yet.
        ///
        /// # Notes
        ///
        /// The usage of `mem::Discriminant<T>` allows to store in the hashmap
        /// tiles with different `Contents` quantity within the same key
        ///
        /// # Example
        /// ```ignore
        /// use tile_resource_mapper_tool::tool::tile_mapper::TileMapper;
        ///
        /// let mapper = TileMapper{};
        ///
        /// let result = mapper::collection(world);
        ///
        /// match result {
        ///     Some(hashmap) => // handle hashmap,
        ///     None => // handle case where the robot has not discovered anything yet
        /// }
        /// ```

        pub fn collection(
            world: &World,
        ) -> Option<HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>>> {
            // HashMap instantiation
            let mut object_list: HashMap<
                Discriminant<Content>,
                Vec<(MapCoordinate, ContentQuantity)>,
            > = HashMap::new();

            // check whether the world has been already discovered or not
            match robot_map(&world) {
                None => {
                    return None;
                }
                Some(robot_world) => {
                    // iterate through every tile in the world

                    for (row, row_vector) in robot_world.iter().enumerate() {
                        for (column, element) in row_vector.iter().enumerate() {
                            match element {
                                None => {}
                                Some(tile) => {
                                    // call the `insert_in_map` function defined below if content was found in the tile
                                    if tile.content != Content::None {
                                        TileMapper::insert_in_map(
                                            tile,
                                            &mut object_list,
                                            row,
                                            column,
                                        )
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
        /// This function inserts the coordinates of a `tile` and the number of elements in that `tile`
        fn insert_in_map(
            tile: &Tile,
            list: &mut HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>>,
            row: usize,
            col: usize,
        ) {
            let coord = (col, row);
            let value: ContentQuantity = tile.content.get_value();
            let content = discriminant(&tile.content);

            // if no tile with `content` is in the list, it creates a new entry with that keyword
            // otherwise coordinates and value are added to the already existing vector
            list.entry(content)
                .and_modify(|v| v.push((coord.into(), value.clone())))
                .or_insert(vec![(coord.into(), value)]);
        }

        /// Finds the closest Tile to the Robot at that time, containing a certain Content.
        ///
        /// # Arguments
        ///
        /// * `world` - A reference to the World in which the coordinates are computed.
        /// * `robot` - A reference to an object implementing the "Runnable" trait.
        /// * `content` - The Content we are looking for.
        ///
        /// # Returns
        ///
        /// Returns the Map coordinates of the closest Tile containing the requested Content.
        /// Returns an error if it fails to find a tile.
        ///
        /// # Example
        /// ```ignore
        /// use tile_resource_mapper_tool::tool::tile_mapper::TileMapper;
        ///
        /// let mapper = TileMapper{};
        ///
        /// let result = mapper.find_closest(world, robot, content);
        ///
        /// match result {
        ///     Ok(coord) => println!("x:{} y:{}",coord.get_width(),coord.get_height()),
        ///     Err(e) => println!("{}",e)
        /// }
        /// ```

        pub fn find_closest(
            &self,
            world: &World,
            robot: &impl Runnable,
            content: Content,
        ) -> Result<MapCoordinate, Box<dyn Error>> {
            let hashmap = TileMapper::collection(world);
            match hashmap {
                Some(map) => {
                    // check if the hashmap contains the searched content
                    let cont = discriminant(&content);
                    return if map.contains_key(&cont) {
                        let vec = map.get(&cont);
                        let robot_coordinates = MapCoordinate::new(
                            robot.get_coordinate().get_col(),
                            robot.get_coordinate().get_row(),
                        );

                        let mut closest_coordinates = MapCoordinate::new(100000, 100000);
                        if let Some(v) = vec {
                            // iterate through the vector and search for the closest tile
                            for element in v.iter() {
                                // search for the smallest distance between the tiles and the robot
                                let old_distance =
                                    closest_coordinates.get_distance(&robot_coordinates);
                                let new_distance = element.0.get_distance(&robot_coordinates);
                                if new_distance < old_distance {
                                    closest_coordinates = element.0;
                                }
                            }
                        }
                        Ok(closest_coordinates)
                    } else {
                        Err(Box::new(ContentNotDiscovered))
                    };
                }
                None => Err(Box::new(WorldNotDiscovered)),
            }
        }
        /// Find the most loaded Tile, given a type of Content.
        ///
        /// # Arguments
        ///
        /// * `world` - A reference to the World in which the coordinates are computed.
        /// * `robot` - A reference to an object implementing the "Runnable" trait.
        /// * `content` - The Content we're searching for.
        ///
        /// # Returns
        ///
        /// Returns the Map coordinates of the tile having the most Content. If no tile is found an error is returned.
        ///
        /// # Notes
        ///    
        /// In case of two Tiles with the same amount of Content, the closest one to the robot is returned.
        ///
        /// # Example
        /// ```ignore
        /// use tile_resource_mapper_tool::tool::tile_mapper::TileMapper;
        ///
        /// let mapper = TileMapper{};
        ///
        /// let result = mapper.find_most_loaded(world, robot, content);
        ///
        /// match result {
        ///     Ok(coord) => println!("x:{} y:{}",coord.get_width(),coord.get_height()),
        ///     Err(e) => println!("{}",e)
        /// }
        /// ```
        pub fn find_most_loaded(
            &self,
            world: &World,
            robot: &impl Runnable,
            content: Content,
        ) -> Result<MapCoordinate, Box<dyn Error>> {
            let hashmap = TileMapper::collection(world);
            // check if the world has already been discovered
            match hashmap {
                Some(map) => {
                    // check if the hashmap contains the searched content
                    let item = discriminant(&content);
                    return if map.contains_key(&item) {
                        let vec = map.get(&item);
                        let mut target_tile_coordinates = MapCoordinate::new(0, 0);
                        if let Some(v) = vec {
                            // instantiate quantity and range
                            let mut quantity: usize = 0;
                            let mut range: Range<usize> = 0..0;
                            // iterate through the vector and search for the most loaded tile
                            for element in v.iter() {
                                // get and set coordinates of the last discovered tile with the higher amount of content
                                // and if two tiles have the same quantity, set the closest tile
                                match &element.1 {
                                    (Some(q), None) => {
                                        if q > &quantity {
                                            quantity = *q;
                                            target_tile_coordinates = element.0;
                                        } else if q == &quantity {
                                            // If two tiles have the same quantity, set the closest tile
                                            let robot_coordinates = MapCoordinate::new(
                                                robot.get_coordinate().get_col(),
                                                robot.get_coordinate().get_row(),
                                            );
                                            let old_distance = target_tile_coordinates
                                                .get_distance(&robot_coordinates);
                                            let new_distance =
                                                element.0.get_distance(&robot_coordinates);
                                            if new_distance < old_distance {
                                                quantity = *q;
                                                target_tile_coordinates = element.0;
                                            }
                                        }
                                    }
                                    (None, Some(span)) => {
                                        if span.len() > range.len() {
                                            range = span.clone();
                                            target_tile_coordinates = element.0;
                                        } else if span.len() == range.len() {
                                            // If two tiles have the same range, set the closest tile
                                            let robot_coordinates = MapCoordinate::new(
                                                robot.get_coordinate().get_col(),
                                                robot.get_coordinate().get_row(),
                                            );
                                            let old_distance = target_tile_coordinates
                                                .get_distance(&robot_coordinates);
                                            let new_distance =
                                                element.0.get_distance(&robot_coordinates);
                                            if new_distance < old_distance {
                                                range = span.clone();
                                                target_tile_coordinates = element.0;
                                            }
                                        }
                                    }
                                    (_, _) => {}
                                }
                            }
                        }
                        Ok(target_tile_coordinates)
                    } else {
                        Err(Box::new(ContentNotDiscovered))
                    };
                }
                None => Err(Box::new(WorldNotDiscovered)),
            }
        }
    }
}
