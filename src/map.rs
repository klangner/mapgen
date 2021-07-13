//! Map structure contains information about tiles and other elements on the map.
//! 
//! Map is created with generators and then can by modified with MapModifiers.
//! 
//! This structure is not intented to be your map in the game (But can be used as one). 
//! Rather the information from this map will be copied to the structures required by
//! specific game.
//! 

use std::fmt;
use super::geometry::{Point, Rect, usize_abs};


#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub struct Tile {
    is_blocked: bool,
    index: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Symmetry { None, Horizontal, Vertical, Both }


/// Map data
#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<Tile>,
    pub width : usize,
    pub height : usize,
    pub starting_point: Option<Point>,
    pub exit_point: Option<Point>,
    pub rooms: Vec<Rect>,
    pub corridors: Vec<Vec<Point>>,
}

impl Tile {
    pub fn new(is_blocked: bool, index: usize) -> Tile {
        Tile { is_blocked, index}
    }

    pub fn wall() -> Tile {
        Tile::new(true, 0)
    }
    
    pub fn floor() -> Tile {
        Tile::new(false, 0)
    }

    pub fn is_walkable(&self) -> bool {
        !self.is_blocked
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Map {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> Map {
        let map_tile_count = width*height;
        Map{
            tiles : vec![Tile::wall(); map_tile_count],
            width,
            height,
            starting_point: None,
            exit_point: None,
            rooms: Vec::new(),
            corridors: Vec::new()
        }
    }

    /// Create map from given string
    pub fn from_string(map_string: &str) -> Map {
        let lines: Vec<&str> = map_string.split("\n")
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .collect();
        let cols = lines.iter().map(|l| l.len()).max().get_or_insert(1).to_owned();
        let rows = lines.len();
        let mut map = Map::new(cols, rows);

        for i in 0..rows {
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                if line[j] as char == ' ' {
                    map.set_tile(j, i, Tile::floor());
                }
            }
        }
        map
    }

    /// Get TileType at the given location
    pub fn at(&self, x: usize, y: usize) -> Tile {
        if x >= self.width || y >= self.height {
            Tile::wall()
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.tiles[idx]
        }
    }

    /// Get available exists from the given tile
    pub fn get_available_exits(&self, x: usize, y: usize) -> Vec<(usize, usize, f32)> {
        let mut exits = Vec::new();

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((x-1, y, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((x+1, y, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((x, y-1, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((x, y+1, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push((x-1, y-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push((x+1, y-1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push((x-1, y+1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push((x+1, y+1, 1.45)); }

        exits
    }    
 
    // Check if given tile can be accessed
    fn is_exit_valid(&self, x:usize, y:usize) -> bool {
        self.at(x, y).is_blocked == false
    }

    /// Modify tile at the given location
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            let idx = self.xy_idx(x as usize, y as usize);
            self.tiles[idx] = tile;
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
    
    /// Create room on the map at given location
    /// Room is created by setting all tiles in the room to the Floor
    pub fn add_room(&mut self, rect: Rect) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set_tile(x as usize, y as usize, Tile::floor());
            }
        }
        self.rooms.push(rect);
    }

    pub fn add_corridor(&mut self, from: Point, to:Point) {
        let mut corridor = Vec::new();
        let mut x = from.x;
        let mut y = from.y;

        while x != to.x || y != to.y {
            if x < to.x {
                x += 1;
            } else if x > to.x {
                x -= 1;
            } else if y < to.y {
                y += 1;
            } else if y > to.y {
                y -= 1;
            }

            if self.at(x, y).is_blocked {
                corridor.push(Point::new(x, y));
                self.set_tile(x, y, Tile::floor());
            }
        }
    }

    pub fn paint(&mut self, mode: Symmetry, brush_size: usize, x: usize, y: usize) {
        match mode {
            Symmetry::None => self.apply_paint(brush_size, x, y),
            Symmetry::Horizontal => {
                let center_x = self.width / 2;
                if x == center_x {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = usize_abs(center_x, x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                }
            }
            Symmetry::Vertical => {
                let center_y = self.height / 2;
                if y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_y = usize_abs(center_y, y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
            Symmetry::Both => {
                let center_x = self.width / 2;
                let center_y = self.height / 2;
                if x == center_x && y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = usize_abs(center_x, x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                    let dist_y = usize_abs(center_y, y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
        }
    }

    fn apply_paint(&mut self, brush_size: usize, x: usize, y: usize) {
        match brush_size {
            1 => {
                self.set_tile(x, y, Tile::floor());
            }
            _ => {
                let half_brush_size = brush_size / 2;
                for brush_y in y-half_brush_size .. y+half_brush_size {
                    for brush_x in x-half_brush_size .. x+half_brush_size {
                        if brush_x > 1 && brush_x < self.width-1 && brush_y > 1 && brush_y < self.height-1 {
                            self.set_tile(brush_x, brush_y, Tile::floor());
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes: Vec<u8> = (0..self.width)
                .map(|x| if self.at(x, y).is_blocked {'#'} else {' '} as u8)
                .collect();
            let line = String::from_utf8(bytes).expect("Can't convert map to string");
            let _ = write!(f, "{}\n", line);
        }
        Ok(())
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map() {
        let map = Map::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert!(map.at(i, j).is_blocked);
            }
        }
    }

    #[test]
    fn test_from_string() {
        let map_str = "
        ##########
        #        #
        ##########
        ";
        let map = Map::from_string(map_str);

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 3);
        for i in 0..10 {
            assert!(map.at(i, 0).is_blocked);
            assert!(map.at(i, 2).is_blocked);
            if i == 0 || i == 9 {
                assert!(map.at(i, 1).is_blocked);
            } else {
                assert!(map.at(i, 1).is_blocked == false);
            }
        }
    }

    #[test]
    fn test_exists() {
        let map_str = "
        ##########
        #        #
        #        #
        ##########
        ";
        let map = Map::from_string(map_str);
        let exists = map.get_available_exits(1, 1);
        let expected_exists = vec![(2, 1, 1.0), (1, 2, 1.0), (2, 2, 1.45)];
        assert_eq!(exists, expected_exists);
    }

        #[test]
    fn test_create_room() {
        let mut map = Map::new(5, 5);
        map.add_room(Rect::new(1, 1, 3, 3));
        for x in 0..map.width {
            for y in 0..map.height {
                if x == 0 || y == 0 || x == 4 || y == 4 {
                    assert!(map.at(x, y).is_blocked);
                } else {
                    assert!(map.at(x, y).is_blocked == false);
                }
            }
        }
    }

    #[test]
    fn test_add_corridor() {
        let map_str = "
        ##########
        #    #   #
        ##########
        ";
        let mut map = Map::from_string(map_str);
        let expected_map_str = "
        ##########
        #        #
        ##########
        ";
        let expected_map = Map::from_string(expected_map_str);

        map.add_corridor(Point::new(1, 1), Point::new(8, 1));

        assert_eq!(map.tiles, expected_map.tiles);
    }
}