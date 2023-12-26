#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem;
    use std::mem::Discriminant;
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::tool::tile_mapper::{ContentQuantity, TileMapper};
    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::{go, Direction};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runnable, Runner};
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use robotics_lib::world::tile::Content::{Bin, Coin, Rock};
    use robotics_lib::world::world_generator::Generator;
    use robotics_lib::world::world_generator::World as WorldType;
    use robotics_lib::world::World;


    #[test]
    fn test_new_map_coordinate() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);
    }

    #[test]
    fn test_set_width() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_width(15);
        assert_eq!(coordinates.get_width(), 15);
    }

    #[test]
    fn test_set_height() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_height(25);
        assert_eq!(coordinates.get_height(), 25);
    }

    #[test]
    fn test_equality() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(10, 20);
        let coordinates3 = MapCoordinate::new(15, 25);

        assert_eq!(coordinates1, coordinates2);
        assert_ne!(coordinates1, coordinates3);
    }

    #[test]
    fn test_addition() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 + coordinates2;
        assert_eq!(result, MapCoordinate::new(15, 30));
    }

    #[test]
    fn test_subtraction() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 - coordinates2;
        assert_eq!(result, MapCoordinate::new(5, 10));
    }

    #[test]
    fn test_from_into_conversion() {
        let tuple_coordinates: (usize, usize) = (10, 20);
        let coordinates: MapCoordinate = tuple_coordinates.into();
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);

        let converted_tuple: (usize, usize) = coordinates.into();
        assert_eq!(converted_tuple, (10, 20));
    }

    #[test]
    fn test_get_distance() {
        let c1 = MapCoordinate::new(4,0);
        let c2 = MapCoordinate::new(1,1);
        let distance = c1.get_distance(&c2);
        let same_distance = c2.get_distance(&c1);
        assert_eq!(distance, 3.1622776601683795);
        assert_eq!(distance, same_distance);

        let c3 = MapCoordinate::new(4,0);
        let c4 = MapCoordinate::new(3,1);
        let distance = c3.get_distance(&c4);
        assert_eq!(distance, 3.1622776601683795);
    }
    #[test]
    fn test_tool_hashmap_discriminant() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                // expected results from the tool
                let mut expected_results: HashMap<Discriminant<Content>, Vec<(MapCoordinate, ContentQuantity)>> = HashMap::new();

                let coord_2_2 = MapCoordinate::new(2,2);
                let coord_1_3 = MapCoordinate::new(3,1);
                let mut v_rocks = vec![];
                v_rocks.push((coord_1_3, (Some(17), None)));
                v_rocks.push((coord_2_2, (Some(2), None)));
                expected_results.insert(mem::discriminant(&Rock(12)), v_rocks);

                let coord_1_2 = MapCoordinate::new(2,1);
                let mut v_coins = vec![];
                v_coins.push((coord_1_2, (Some(3), None)));
                expected_results.insert(mem::discriminant(&Coin(20)), v_coins);

                let coord_1_1 = MapCoordinate::new(1,1);
                let mut v_bin = vec![];
                v_bin.push((coord_1_1, (None, Some(0..4))));
                expected_results.insert(mem::discriminant(&Bin(0..2)), v_bin);

                let result = TileMapper::collection(world);
                match result {
                    Some(res) => {
                        println!("{:?}", res);

                        assert_eq!(res, expected_results);
                        // assert_eq!(res.len(), 1);
                    },
                    None =>  panic!("error while matching the hashmap")
                }
            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks in (2,2)
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(2),
                    elevation: 0,
                };
                // add rocks in (1,3)
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(17),
                    elevation: 0,
                };
                // add coins in (1,2)
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(3),
                    elevation: 0,
                };
                // add Bin in (1,1)
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..4),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }


    // #[test]
    // fn test_tool_hashmap() {
    //     struct TestRobot(Robot);
    //     impl Runnable for TestRobot {
    //         fn process_tick(&mut self, world: &mut World) {
    //             let tool = TileMapper {};
    //             // path the robot must follow
    //             let directions = [
    //                 Direction::Right,
    //                 Direction::Down,
    //                 Direction::Right,
    //                 Direction::Down,
    //                 Direction::Up,
    //                 Direction::Right,
    //                 Direction::Up,
    //                 Direction::Right,
    //             ];
    //
    //             //move the robot
    //             for (phase, dir) in directions.iter().enumerate() {
    //                 go(self, world, dir.to_owned()).expect("");
    //             }
    //
    //             // expected results from the tool
    //             let mut expected_results: HashMap<ToolContent, Vec<(MapCoordinate, ContentQuantity)>> = HashMap::new();
    //
    //             let coord_2_2 = MapCoordinate::new(2,2);
    //             let coord_1_3 = MapCoordinate::new(3,1);
    //             let mut v_rocks = vec![];
    //             v_rocks.push((coord_1_3, (Some(17), None)));
    //             v_rocks.push((coord_2_2, (Some(2), None)));
    //             expected_results.insert(ToolContent::Rock, v_rocks);
    //
    //             let coord_1_2 = MapCoordinate::new(2,1);
    //             let mut v_coins = vec![];
    //             v_coins.push((coord_1_2, (Some(3), None)));
    //             expected_results.insert(ToolContent::Coin, v_coins);
    //
    //             let coord_1_1 = MapCoordinate::new(1,1);
    //             let mut v_bin = vec![];
    //             v_bin.push((coord_1_1, (None, Some(0..4))));
    //             expected_results.insert(ToolContent::Bin, v_bin);
    //
    //             let result = TileMapper::collection(world);
    //             match result {
    //                 Some(res) => {
    //                     println!("{:?}", res);
    //
    //                     assert_eq!(res, expected_results);
    //                     // assert_eq!(res.len(), 1);
    //                 },
    //                 None =>  panic!("error while matching the hashmap")
    //             }
    //         }
    //         fn handle_event(&mut self, event: Event) {
    //             println!();
    //             println!("{:?}", event);
    //             println!();
    //         }
    //         fn get_energy(&self) -> &Energy {
    //             &self.0.energy
    //         }
    //         fn get_energy_mut(&mut self) -> &mut Energy {
    //             &mut self.0.energy
    //         }
    //         fn get_coordinate(&self) -> &Coordinate {
    //             &self.0.coordinate
    //         }
    //         fn get_coordinate_mut(&mut self) -> &mut Coordinate {
    //             &mut self.0.coordinate
    //         }
    //         fn get_backpack(&self) -> &BackPack {
    //             &self.0.backpack
    //         }
    //         fn get_backpack_mut(&mut self) -> &mut BackPack {
    //             &mut self.0.backpack
    //         }
    //     }
    //
    //     struct WorldGenerator {
    //         size: usize,
    //         spawn_x: usize,
    //         spawn_y: usize,
    //         tile_type: TileType,
    //     }
    //
    //     impl WorldGenerator {
    //         fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
    //             Self {
    //                 size,
    //                 spawn_x,
    //                 spawn_y,
    //                 tile_type,
    //             }
    //         }
    //     }
    //
    //     impl Generator for WorldGenerator {
    //         fn gen(&mut self) -> WorldType {
    //             let mut map: Vec<Vec<Tile>> = Vec::new();
    //             // Initialize the map with default tiles
    //             for _ in 0..self.size {
    //                 let mut row: Vec<Tile> = Vec::new();
    //                 for _ in 0..self.size {
    //                     let tile = Tile {
    //                         tile_type: self.tile_type,
    //                         content: Content::None,
    //                         elevation: 0,
    //                     };
    //                     row.push(tile);
    //                 }
    //                 map.push(row);
    //             }
    //             // add rocks in (2,2)
    //             map[2][2] = Tile {
    //                 tile_type: self.tile_type,
    //                 content: Content::Rock(2),
    //                 elevation: 0,
    //             };
    //             // add rocks in (1,3)
    //             map[1][3] = Tile {
    //                 tile_type: self.tile_type,
    //                 content: Content::Rock(17),
    //                 elevation: 0,
    //             };
    //             // add coins in (1,2)
    //             map[1][2] = Tile {
    //                 tile_type: self.tile_type,
    //                 content: Content::Coin(3),
    //                 elevation: 0,
    //             };
    //             // add Bin in (1,1)
    //             map[1][1] = Tile {
    //                 tile_type: self.tile_type,
    //                 content: Content::Bin(0..4),
    //                 elevation: 0,
    //             };
    //
    //
    //             let environmental_conditions =
    //                 EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
    //             // implementation
    //             return (
    //                 map,
    //                 (self.spawn_x, self.spawn_y),
    //                 environmental_conditions,
    //                 10.0,
    //                 None,
    //             );
    //         }
    //     }
    //
    //     let r = TestRobot(Robot::new());
    //     let runner = Runner::new(
    //         Box::new(r),
    //         &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
    //     );
    //     let _ = runner.unwrap().game_tick();
    // }

    #[test]
    fn test_find_most_loaded_usize() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_most_loaded(world, self, Content::Rock(0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(2),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(17),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(5),
                    elevation: 0,
                    };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_find_most_loaded_range() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_most_loaded(world, self, Content::Bin(0..0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };
            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..2),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..8),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..5),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..3),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_find_most_loaded_with_duplicates_range() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_most_loaded(world, self, Content::Bin(0..0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_find_most_loaded_with_duplicates_usize() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_most_loaded(world, self, Content::Rock(0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_find_most_loaded_not_discovered() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_error = "Content not discovered yet".to_string();

                match tool.find_most_loaded(world, self, Content::Bush(0)) {
                    Ok(result) => {},
                    // Err(e) => panic!("{}", e)
                    Err(e) => assert_eq!(e.to_string(), expected_error),
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(2),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Scarecrow,
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::JollyBlock(1),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }


    #[test]
    fn test_find_closest() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_closest(world, self, Content::Rock(0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(2),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(6),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(5),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Rock(9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_find_closest_with_duplicates_range() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let tool = TileMapper {};
                // path the robot must follow
                let directions = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                ];

                //move the robot
                for (phase, dir) in directions.iter().enumerate() {
                    go(self, world, dir.to_owned()).expect("");
                }

                let expected_result = MapCoordinate::new(3,1);

                match tool.find_closest(world, self, Content::Bin(0..0)) {
                    Ok(result) => {
                        assert_eq!(result, expected_result);
                    }
                    Err(e) => panic!("{}", e)
                };

            }
            fn handle_event(&mut self, event: Event) {
                println!();
                println!("{:?}", event);
                println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add rocks
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[1][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(0..9),
                    elevation: 0,
                };
                map[0][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };


                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_x, self.spawn_y),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 0, 0, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }
}