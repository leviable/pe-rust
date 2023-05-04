// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?
use std::collections::HashMap;
use std::time::Instant;

struct Grid {
    origin: Point,
    end: Point,
    routes: HashMap<Point, usize>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            origin: Point { x: 0, y: 0 },
            end: Point { x: size, y: size },
            routes: HashMap::new(),
        }
    }

    fn point_above(&self, point: &Point) -> bool {
        point.y > 0
    }

    fn point_left(&self, point: &Point) -> bool {
        point.x > 0
    }

    fn get_point_above(&self, point: &Point) -> Point {
        Point {
            x: point.x,
            y: point.y - 1,
        }
    }

    fn get_point_left(&self, point: &Point) -> Point {
        Point {
            x: point.x - 1,
            y: point.y,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn find_routes_at_point(grid: &mut Grid, point: &Point) -> usize {
    if *point == grid.origin {
        return 1;
    }

    if !grid.routes.contains_key(point) {
        let mut found_routes: usize = 0;
        if grid.point_above(point) {
            let p = grid.get_point_above(point);
            found_routes += find_routes_at_point(grid, &p);
        }
        if grid.point_left(point) {
            let p = grid.get_point_left(point);
            found_routes += find_routes_at_point(grid, &p);
        }

        grid.routes.insert(point.clone(), found_routes);
    }

    let my_routes = grid.routes.get(point).unwrap().clone();

    my_routes
}

#[test]
fn test_find_routes_at_point() {
    let mut test_grid = Grid::new(2);
    let starting_point = test_grid.end.clone();
    let routes = find_routes_at_point(&mut test_grid, &starting_point);
    assert_eq!(routes, 6);
}

fn main() {
    let start = Instant::now();
    let mut grid = Grid::new(20);
    let starting_point = grid.end.clone();
    let routes = find_routes_at_point(&mut grid, &starting_point);
    println!("Total routes for 20: {}", routes);
    println!("Took {}s to complete", start.elapsed().as_secs_f64());
}
