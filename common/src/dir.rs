use vek::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn vec(self) -> Vec3<i32> {
        match self {
            Direction::North => Vec3::unit_z(),
            Direction::South => -Vec3::unit_z(),
            Direction::East => Vec3::unit_x(),
            Direction::West => -Vec3::unit_x(),
            Direction::Up => Vec3::unit_y(),
            Direction::Down => -Vec3::unit_y(),
        }
    }
}
