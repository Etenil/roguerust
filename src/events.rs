use crate::world::Movement;

#[derive(Copy, Clone, Debug)]
pub enum ViewportEvent {
    Quit,
    Help,
    MovePlayer(Movement),
    DownStairs,
    UpStairs,
}
