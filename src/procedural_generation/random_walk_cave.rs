use rand::Rng;
use rand::SeedableRng;

#[allow(dead_code)]
/// Initializes a new cave map and generates it using the Random Walk algorithm.
pub fn generate_random_walk_cave(
    width: usize,
    height: usize,
    seed: u32,
    required_empty_percent: f32,
) -> Vec<Vec<i32>> {
    let mut map = initialize_map(width, height); // Initialize the map with walls.
    random_walk_cave(&mut map, seed, required_empty_percent); // Generate the cave.
    map
}

/// Initializes the cave map filled with walls (1).
fn initialize_map(width: usize, height: usize) -> Vec<Vec<i32>> {
    vec![vec![1; height]; width] // Create a 2D vector filled with walls.
}

/// Generates a cave-like map using the Random Walk Algorithm.
fn random_walk_cave(map: &mut Vec<Vec<i32>>, seed: u32, required_empty_percent: f32) {
    let (width, height) = (map.len(), map[0].len());
    let total_tiles = width * height;
    let required_empty_tiles =
        (total_tiles as f32 * required_empty_percent / 100.0).round() as usize;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);

    let (mut floor_x, mut floor_y) = random_start_position(width, height, &mut rng);
    let mut empty_tile_count = 1; // Starting position is already counted as empty
    const MAX_ITERATIONS: usize = 10000;

    map[floor_x][floor_y] = 0; // Set starting position to empty

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while empty_tile_count < required_empty_tiles && empty_tile_count < MAX_ITERATIONS {
        let (dx, dy) = directions[rng.gen_range(0..4)];
        let (new_x, new_y) = (
            (floor_x as isize + dx) as usize,
            (floor_y as isize + dy) as usize,
        );

        if is_within_bounds(new_x, new_y, width, height) && map[new_x][new_y] == 1 {
            map[new_x][new_y] = 0; // Convert wall to empty
            empty_tile_count += 1; // Increment empty tile count
        }

        floor_x = new_x; // Update current position
        floor_y = new_y;
    }
}

/// Generates a random starting position within the map's bounds.
fn random_start_position(width: usize, height: usize, rng: &mut impl Rng) -> (usize, usize) {
    let x = rng.gen_range(1..width - 1);
    let y = rng.gen_range(1..height - 1);
    (x, y)
}

/// Checks if the given coordinates are within the bounds of the map.
fn is_within_bounds(x: usize, y: usize, width: usize, height: usize) -> bool {
    x > 0 && x < width - 1 && y > 0 && y < height - 1
}
