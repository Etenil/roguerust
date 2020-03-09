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
}

impl Tile {
    pub fn new(tile_type: TileType, visible: bool, opaque: bool) -> Self {
        Tile {
            tile_type,
            visible,
            opaque,
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
        self.grid[y][x].visibility(true);
    }

    /// Clears all blocks in a single line of sight ray; stop when encountering a wall
    /// This uses the bresenham algorithm, see:
    ///     https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn clear_los(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        let dx = (end.0 as isize - start.0 as isize).abs();
        let sx: isize = if start.0 < end.0 { 1 } else { -1 };
        let dy = -(end.1 as isize - start.1 as isize).abs();
        let sy: isize = if start.1 < end.1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = start.0;
        let mut y = start.1;

        // the tile we're standing on needs to be visible.
        self.reveal(start.0, start.1);

        while x != end.0 && y != end.1 {
            let err2 = 2 * err;
            if err2 >= dy {
                err += dy;
                x = (x as isize + sx).max(0) as usize;
            }
            if err2 <= dx {
                err += dx;
                y = (y as isize + sy).max(0) as usize;
            }

            self.reveal(x, y);

            if self.tile_at(x, y).is_opaque() {
                break;
            }
        }
    }

    /// Walk around the perimeter of the line of sight and ray-trace to clear tiles
    /// up to the nearest obstacle.
    pub fn clear_fog_of_war(&mut self, center: &(usize, usize), radius: usize) {
        let perimeter = circle(&(center.0, center.1 + 1), radius);

        for point in perimeter.iter() {
            self.clear_los(
                center,
                &(point.0.min(self.xsize), point.1.min(self.ysize())),
            );
        }
    }
}

pub fn tile_to_str(tile: &Tile) -> &str {
    if tile.is_visible() {
        match tile.tile_type {
            TileType::Floor => ".",
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

fn circle(center: &(usize, usize), radius: usize) -> Vec<(usize, usize)> {
    let mut x: i32 = radius as i32;
    let mut y: i32 = 0;
    let mut err: i32 = 0;

    let signs: [i32; 2] = [-1, 1];
    let mut points: Vec<(usize, usize)> = vec![];

    while x >= y {
        for xsign in signs.iter() {
            for ysign in signs.iter() {
                points.push((
                    (center.0 as i32 + xsign * x).max(0) as usize,
                    (center.1 as i32 + ysign * y).max(0) as usize,
                ));
                points.push((
                    (center.0 as i32 + xsign * y).max(0) as usize,
                    (center.1 as i32 + ysign * x).max(0) as usize,
                ));
            }
        }

        if err <= 0 {
            y += 1;
            err += 2 * y + 1;
        }

        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
    points.sort();
    points.dedup();
    points
}

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
    fn test_clear_los_clears_to_wall_on_vertical_up() {
        let mut grid = TileGrid::new(9, 9);
        grid.set_tile(5, 1, Tile::from(TileType::Wall));
        grid.set_tile(5, 2, Tile::from(TileType::Floor));
        grid.set_tile(5, 3, Tile::from(TileType::Floor));

        grid.clear_los(&(5, 3), &(5, 0));
        assert_eq!(grid.tile_at(5, 4).is_visible(), false);
        assert_eq!(grid.tile_at(5, 3).is_visible(), true);
        assert_eq!(grid.tile_at(6, 2).is_visible(), false);
        assert_eq!(grid.tile_at(5, 2).is_visible(), true);
        assert_eq!(grid.tile_at(4, 2).is_visible(), false);
        assert_eq!(grid.tile_at(5, 1).is_visible(), true);
        assert_eq!(grid.tile_at(5, 0).is_visible(), false);
    }

    #[test]
    fn test_clear_los_stops_at_contiguous_wall_up() {
        let mut grid = TileGrid::new(9, 9);
        grid.set_tile(5, 1, Tile::from(TileType::Floor));
        grid.set_tile(5, 2, Tile::from(TileType::Wall));
        grid.set_tile(5, 3, Tile::from(TileType::Floor));

        grid.clear_los(&(5, 3), &(5, 0));
        assert_eq!(grid.tile_at(5, 0).is_visible(), false);
        assert_eq!(grid.tile_at(5, 1).is_visible(), false);
        assert_eq!(grid.tile_at(5, 2).is_visible(), true);
        assert_eq!(grid.tile_at(5, 3).is_visible(), true);
    }

    #[test]
    fn test_clear_los_clears_to_wall_on_vertical_down() {
        let mut grid = TileGrid::new(9, 9);
        grid.set_tile(5, 3, Tile::from(TileType::Wall));
        grid.set_tile(5, 2, Tile::from(TileType::Floor));
        grid.set_tile(5, 1, Tile::from(TileType::Floor));

        grid.clear_los(&(5, 1), &(5, 4));
        assert_eq!(grid.tile_at(5, 0).is_visible(), false);
        assert_eq!(grid.tile_at(4, 1).is_visible(), false);
        assert_eq!(grid.tile_at(5, 1).is_visible(), true);
        assert_eq!(grid.tile_at(6, 1).is_visible(), false);
        assert_eq!(grid.tile_at(5, 2).is_visible(), true);
        assert_eq!(grid.tile_at(5, 3).is_visible(), true);
        assert_eq!(grid.tile_at(5, 4).is_visible(), false);
    }

    #[test]
    fn test_clear_los_clears_to_wall_on_horizontal_right() {
        let mut grid = TileGrid::new(9, 9);
        grid.set_tile(3, 5, Tile::from(TileType::Wall));
        grid.set_tile(2, 5, Tile::from(TileType::Floor));
        grid.set_tile(1, 5, Tile::from(TileType::Floor));

        grid.clear_los(&(1, 5), &(4, 5));
        assert_eq!(grid.tile_at(0, 5).is_visible(), false);
        assert_eq!(grid.tile_at(1, 5).is_visible(), true);
        assert_eq!(grid.tile_at(2, 4).is_visible(), false);
        assert_eq!(grid.tile_at(2, 5).is_visible(), true);
        assert_eq!(grid.tile_at(2, 6).is_visible(), false);
        assert_eq!(grid.tile_at(3, 5).is_visible(), true);
        assert_eq!(grid.tile_at(3, 6).is_visible(), false);
    }

    #[test]
    fn test_clear_los_clears_to_wall_on_horizontal_left() {
        let mut grid = TileGrid::new(9, 9);
        grid.set_tile(1, 5, Tile::from(TileType::Wall));
        grid.set_tile(2, 5, Tile::from(TileType::Floor));
        grid.set_tile(3, 5, Tile::from(TileType::Floor));

        grid.clear_los(&(3, 5), &(0, 5));
        assert_eq!(grid.tile_at(4, 5).is_visible(), false);
        assert_eq!(grid.tile_at(3, 5).is_visible(), true);
        assert_eq!(grid.tile_at(2, 4).is_visible(), false);
        assert_eq!(grid.tile_at(2, 5).is_visible(), true);
        assert_eq!(grid.tile_at(2, 6).is_visible(), false);
        assert_eq!(grid.tile_at(1, 5).is_visible(), true);
        assert_eq!(grid.tile_at(0, 6).is_visible(), false);
    }

    #[test]
    fn circle_creates_a_circle() {
        let circ = circle(&(10, 10), 3);
        assert_eq!(
            circ.as_slice(),
            [
                (7, 10),
                (8, 9),
                (8, 11),
                (9, 8),
                (9, 12),
                (10, 7),
                (10, 13),
                (11, 8),
                (11, 12),
                (12, 9),
                (12, 11),
                (13, 10)
            ]
        )
    }
}
