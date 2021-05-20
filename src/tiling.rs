use log::debug;
use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    StairsUp,
    StairsDown,
    Character(&'static str),
    Player,
}

#[derive(Clone, Debug)]
pub struct Tile {
    tile_type: TileType,
    visible: bool,
    opaque: bool,
    lit: bool,
}

impl Tile {
    pub fn new(tile_type: TileType, visible: bool, opaque: bool, lit: bool) -> Self {
        Tile {
            tile_type,
            visible,
            opaque,
	    lit,
        }
    }

    pub fn get_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_lit(&self) -> bool {
	self.lit
    }

    pub fn lit(&mut self, lit: bool) {
	self.lit = lit;
    }

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn opacity(&mut self, opaque: bool) {
        self.opaque = opaque
    }
}

impl From<TileType> for Tile {
    fn from(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            visible: false, // <--- TODO: this set the default beaviour
            //            - true: all tiles of world and entities will be drawn
            //            - false: only draw tiles visible for the player
            opaque: match tile_type {
                TileType::Empty => true,
                TileType::Wall => true,
                _ => false,
            },
	    lit: false,
        }
    }
}

pub struct TileGrid {
    grid: Vec<Vec<Tile>>,
    xsize: usize,
    ysize: usize,
}

impl TileGrid {
    pub fn new(xsize: usize, ysize: usize) -> TileGrid {
        let mut grid = TileGrid {
            grid: Vec::with_capacity(ysize),
            xsize,
            ysize,
        };

        for _ in 0..ysize {
            let mut subvec = Vec::with_capacity(xsize);
            for _ in 0..xsize {
                subvec.push(Tile::from(TileType::Empty));
            }
            grid.grid.push(subvec);
        }

        grid
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y][x] = tile;
    }

    /// Sets a tile if nothing lies underneath it.
    pub fn set_empty_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.set_tile(
            x,
            y,
            match self.grid[y][x].tile_type {
                TileType::Empty => tile,
                _ => self.grid[y][x].clone(),
            },
        )
    }

    pub fn raw_data(&self) -> &Vec<Vec<Tile>> {
        &self.grid
    }

    pub fn tile_at(&self, x: usize, y: usize) -> &Tile {
        &self.grid[y][x]
    }

    pub fn block_at(&self, x: usize, y: usize) -> &Tile {
        //Needed to integrate with the terminal numbering
        &self.grid[y + 1][x]
    }

    pub fn xsize(&self) -> usize {
        self.xsize
    }

    pub fn ysize(&self) -> usize {
        self.ysize
    }

    fn reveal(&mut self, x: usize, y: usize) {
        self.grid[y + 1][x].visibility(true);
    }

    fn light(&mut self, x: usize, y: usize) {
	self.grid[y + 1][x].lit(true);
    }

    /// Walk around the perimeter of the line of sight and ray-trace to clear tiles
    /// up to the nearest obstacle.
    pub fn clear_fog_of_war(&mut self, center: &(usize, usize), radius: usize) {
	// Unlight everything first.
	for x in 0..self.xsize {
	    for y in 0..self.ysize {
		self.grid[y][x].lit(false)
	    }
	}

	let start: (usize, usize) = (center.0.saturating_sub(radius), center.1.saturating_sub(radius));
	let end: (usize, usize) = (center.0.saturating_add(radius), center.1.saturating_add(radius));

	println!("From {:?} to {:?}", start, end);

	for x in start.0..=end.0 {
	    for y in start.1..=end.1 {
		self.reveal(x, y);
		self.light(x, y);
	    }
	}
    }
}

pub fn tile_to_str(tile: &Tile) -> &str {
    if tile.is_visible() {
        match tile.tile_type {
            TileType::Floor => match tile.is_lit() {
		true => ".",
		false => " "
	    },
            TileType::Wall => "#",
            TileType::Empty => " ",
            TileType::StairsDown => ">",
            TileType::StairsUp => "<",
            TileType::Player => "@",
            TileType::Character(t) => t,
        }
    } else {
        " "
    }
}

pub trait Tileable {
    fn tile(&self, grid: &mut TileGrid) -> Result<(), String>;
}

// fn circle(center: &(usize, usize), radius: usize) -> Vec<(usize, usize)> {
//     let mut x: i32 = radius as i32;
//     let mut y: i32 = 0;
//     let mut err: i32 = 0;

//     let signs: [i32; 2] = [-1, 1];
//     let mut points: Vec<(usize, usize)> = vec![];

//     while x >= y {
//         for xsign in signs.iter() {
//             for ysign in signs.iter() {
//                 points.push((
//                     (center.0 as i32 + xsign * x).max(0) as usize,
//                     (center.1 as i32 + ysign * y).max(0) as usize,
//                 ));
//                 points.push((
//                     (center.0 as i32 + xsign * y).max(0) as usize,
//                     (center.1 as i32 + ysign * x).max(0) as usize,
//                 ));
//             }
//         }

//         if err <= 0 {
//             y += 1;
//             err += 2 * y + 1;
//         }

//         if err > 0 {
//             x -= 1;
//             err -= 2 * x + 1;
//         }
//     }
//     points.sort();
//     points.dedup();
//     points
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tilegrid_is_populated_by_empty_invisible_tiles() {
        const GRID_SIZE: usize = 2;
        let grid = TileGrid::new(GRID_SIZE, GRID_SIZE);
        for x in 0..2 {
            for y in 0..2 {
                assert!(match grid.tile_at(x, y).tile_type {
                    TileType::Empty => true,
                    _ => false,
                });
                assert_eq!(grid.tile_at(x, y).is_visible(), false);
            }
        }
    }

    #[test]
    fn tiles_can_be_revealed() {
        let mut tile = Tile::from(TileType::Wall);
        assert_eq!(tile.visible, false);
        assert_eq!(tile.is_visible(), false);
        tile.visibility(true);
        assert_eq!(tile.visible, true);
        assert_eq!(tile.is_visible(), true);
    }

    #[test]
    fn tilegrid_can_reveal_tiles() {
        let mut grid = TileGrid::new(1, 1);
        grid.reveal(0, 0);
        assert_eq!(grid.grid[0][0].is_visible(), true);
        assert_eq!(grid.tile_at(0, 0).is_visible(), true);
    }

    #[test]
    fn test_clear_fog_of_war() {
	let mut grid = TileGrid::new(5, 5);
	
	grid.clear_fog_of_war(&(2, 2), 1);

	println!("test");

	for x in 0..5 {
	    for y in 0..5 {
		if grid.tile_at(x, y).is_visible() {
		    print!("x");
		} else {
		    print!(" ");
		}
	    }
	    print!("\n");
	}
	
	assert_eq!(grid.tile_at(0, 0).is_visible(), false);
	assert_eq!(grid.tile_at(1, 0).is_visible(), false);
	assert_eq!(grid.tile_at(2, 0).is_visible(), false);
	assert_eq!(grid.tile_at(3, 0).is_visible(), false);
	assert_eq!(grid.tile_at(4, 0).is_visible(), false);

	assert_eq!(grid.tile_at(0, 1).is_visible(), false);
	assert_eq!(grid.tile_at(1, 1).is_visible(), true);
	assert_eq!(grid.tile_at(2, 1).is_visible(), true);
	assert_eq!(grid.tile_at(3, 1).is_visible(), true);
	assert_eq!(grid.tile_at(4, 1).is_visible(), false);

	assert_eq!(grid.tile_at(0, 2).is_visible(), false);
	assert_eq!(grid.tile_at(1, 2).is_visible(), true);
	assert_eq!(grid.tile_at(2, 2).is_visible(), true);
	assert_eq!(grid.tile_at(3, 2).is_visible(), true);
	assert_eq!(grid.tile_at(4, 2).is_visible(), false);

	assert_eq!(grid.tile_at(0, 3).is_visible(), false);
	assert_eq!(grid.tile_at(1, 3).is_visible(), true);
	assert_eq!(grid.tile_at(2, 3).is_visible(), true);
	assert_eq!(grid.tile_at(3, 3).is_visible(), true);
	assert_eq!(grid.tile_at(4, 3).is_visible(), false);

	assert_eq!(grid.tile_at(0, 4).is_visible(), false);
	assert_eq!(grid.tile_at(1, 4).is_visible(), false);
	assert_eq!(grid.tile_at(2, 4).is_visible(), false);
	assert_eq!(grid.tile_at(3, 4).is_visible(), false);
	assert_eq!(grid.tile_at(4, 4).is_visible(), false);
    }
}
