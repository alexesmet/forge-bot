use enigo::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

use super::conf;


const COLLAPSE: usize = 3;

pub struct Bot {
    enigo: Enigo,
    config: conf::Config,
    pub field: [[usize; conf::FIELD_SIZE.1]; conf::FIELD_SIZE.0],
    pub tasks: Vec<[usize; 2]> //TODO: Make private in future, move fyll cycle to this module
}

impl Bot {
    pub fn new(config: conf::Config) -> Self {
        Bot {
            enigo: Enigo::new(),
            config: config,
            field: [[0; conf::FIELD_SIZE.1]; conf::FIELD_SIZE.0],
            tasks: Vec::new()
        }
    }

    pub fn create_order(&mut self, level: usize, coords: [usize; 2]) {
        let mut provided = self.path_provider_for_level(coords, Vec::new(), level - 1).unwrap();
        self.tasks.append(&mut provided);
    }

    pub fn yelid_task(&mut self) {
        if let Some(task) = self.tasks.pop() {
            println!("Executing at {:?}", task);
            self.place_ore(task[0], task[1]);
        }
        //TODO: Return result
    }

    // TODO: Remove this method if it's not needed in future
    #[allow(dead_code)]
    fn count_adjustent_type(&self, level: usize, coords: [usize; 2], checked: &mut Vec<[usize; 2]>) ->u32 {
        checked.push(coords);
        let mut counted: u32 = 0;
        if self.field[coords[0]][coords[1]] == level {
            counted += 1;
            for tile_opt in &adjustent_tiles(coords) {
                if let Some(tile) = tile_opt {
                    if self.is_cell_has_level(*tile, level) && checked.iter().all(|&c| c != *tile) {
                        counted += self.count_adjustent_type(level, *tile, checked);
                    }
                }
            }
        }
        return counted;
    }

    pub fn place_ore(&mut self, x: usize , y: usize) {
        let coords = self.config.to_screen([x, y]);
        self.enigo.mouse_move_to(coords[0], coords[1]);
        self.enigo.mouse_click(MouseButton::Left);
    }

    fn path_provider(&self, start: [usize; 2], blocked: Vec<[usize; 2]>) -> Vec<[[usize; 2]; COLLAPSE]> {
        let mut result: Vec<([[usize;2]; COLLAPSE], usize)> = Vec::new();
        let empt = [0, 0];
        result.push(([start, empt, empt], 1)); 
        // list of paths is consideted refined when all paths have length of COLLAPSE
        let mut is_refined = false;
        while !is_refined {
            is_refined = true;
            let mut i = 0;
            while i < result.len() {
                if result[i].1 != COLLAPSE {
                    is_refined = false;
                    let to_refine = result.remove(i);
                    let adjustent = adjustent_tiles(to_refine.0[to_refine.1 - 1]);
                    for opt_adj in &adjustent {
                        if let Some(adj) = opt_adj {
                            if self.is_cell_has_level(*adj, 0)
                            && to_refine.0.iter().take(to_refine.1).all(|e| *e != *adj)
                            && blocked.iter().all(|e| *e != *adj) {
                                let mut to_add = to_refine.clone();
                                to_add.0[to_add.1] = *adj;
                                to_add.1 += 1;
                                result.push(to_add);
                            }
                        }
                    }
                } else {
                    i += 1;
                }
            }
        }
        return result.iter().map(|e| e.0).collect();
    }

    // returns possible sequences of how lowest ore should be layed out to get
    // desired ore.
    fn path_provider_for_level(
        &mut self,
        start: [usize; 2],
        blocked: Vec<[usize; 2]>,
        level: usize
    ) -> Option<Vec<[usize; 2]>> {
        let mut rng = thread_rng();
        if level > 1 {
            let mut primary_paths = self.path_provider(start, blocked.clone());
            primary_paths.shuffle(&mut rng);
            'primary: for primary_path in primary_paths.iter() {
                let mut result = Vec::new();
                for i in 0..primary_path.len() {
                    let secondary_path = self.path_provider_for_level(
                            primary_path[i],
                            [primary_path.iter().skip(i+1).map(|x| *x).collect(), blocked.clone()].concat(),
                            level-1
                    );
                    match secondary_path {
                        Some(path) => result.append(&mut path.clone()),
                        None => continue 'primary,
                    }
                }
                return Some(result);
            }
            return None;
        } else {
            let res = self.path_provider(start, blocked);
            return res.choose(&mut rng).map(|e| e.to_vec());
        }
    }


    fn is_cell_has_level(&self, tile: [usize; 2], level: usize) -> bool {
        return (tile[0]) < self.field.len()
            && (tile[1]) < self.field[tile[0]].len()
            && self.field[tile[0]][tile[1]] == level;
    }

}


fn adjustent_tiles(c: [usize; 2]) -> [Option<[usize; 2]>; 4] {
    return [
        Some([c[0]  , c[1]+1]),
        Some([c[0]+1, c[1]  ]),
        if c[1] > 0 { Some([c[0]  , c[1]-1]) } else { None },
        if c[0] > 0 { Some([c[0]-1, c[1]  ]) } else { None }
    ];
}


// ================================================================================


