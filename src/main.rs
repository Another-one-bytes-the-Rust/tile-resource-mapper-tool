pub mod tile_mapper {
    use std::collections::HashMap;
    use std::ops::Range;
    use robotics_lib::interface::{robot_map, Tools};
    use robotics_lib::runner::Runnable;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::tile::{Content, Tile};
    use robotics_lib::world::World;

    pub struct TileMapper {}

    impl Tools for TileMapper {}

    /// This tool stores the number of elements found in any tile by the robot
    /// It returns a `HashMap` where `key` is the element searched and `value` is a vector of tuples,
    /// The tuple stores the coordinates of a tile and the number of elements contained in that tile


    impl TileMapper {

        fn foo(&self, robot: &impl Runnable ,world: &World) -> Option<HashMap<Content, Vec<(Coordinate , usize)>>> {


            match robot_map(&world) {
                None => {return None;},
                Some(robot_world) => {
                    let mut object_list: HashMap<Content, Vec<(Coordinate , usize)>> = HashMap::new();
                    let mut row = 0;

                    for rows in robot_world.iter() {
                        let mut col = 0;
                        for element in rows.iter() {
                            match element {
                                None => {}
                                Some(tile) => {
                                    TileMapper::insert_in_map(tile, &mut object_list, row, col)
                                }
                            }
                            col += 1;
                        }
                        row += 1;
                    }


                }
            }




            None
        }

        fn insert_in_map(tile: &Tile , list: &mut HashMap<Content, Vec<(Coordinate , usize)>>, row: i32, col: i32) {


        }

    }
}


fn main() {


    // let v1 = vec![1,2,3,4];
    // let v2 = vec![5,6,7,8];
    // let v3 = vec![9,10,11,12];
    //
    // let vec: Vec<Vec<i32>> = vec![v1, v2, v3];
    //
    // for row in vec.iter() {
    //     for el in row {
    //         println!("{}", el);
    //     }
    // }
}