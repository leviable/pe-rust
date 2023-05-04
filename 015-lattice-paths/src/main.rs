// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?
use std::collections::HashMap;

struct Grid {
    origin: Point,
    end: Point,
    routes: HashMap<Point, Vec<Route>>,
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

#[derive(Clone)]
struct Route {
    points: Vec<Point>,
}

impl Route {
    fn new(point: Point) -> Self {
        Self {
            points: vec![point],
        }
    }

    fn add(&mut self, point: Point) {
        self.points.push(point);
    }
}

fn find_routes_at_point(grid: &mut Grid, point: &Point) -> Vec<Route> {
    println!("00000000000000000000000000000000000000");
    if *point == grid.origin {
        return vec![Route::new(point.clone())];
    }

    if !grid.routes.contains_key(point) {
        println!("11111111111111111111111111111111111111");
        println!("{:?}", point);
        let mut found_routes: Vec<Route> = vec![];
        if grid.point_above(point) {
            println!("33333333333333333333333333333333333333");
            let p = grid.get_point_above(point);
            found_routes.extend(find_routes_at_point(grid, &p));
            println!("44444444444444444444444444444444444444");
        }
        if grid.point_left(point) {
            println!("55555555555555555555555555555555555555");
            let p = grid.get_point_left(point);
            found_routes.extend(find_routes_at_point(grid, &p));
            println!("66666666666666666666666666666666666666");
        }

        println!(
            "77777777777777777777777777777777777777 - {}",
            found_routes.len()
        );
        for idx in 0..found_routes.len() {
            found_routes[idx].add(point.clone());
            // println!("88888888888888888888888888888888888888");
        }
        println!("88888888888888888888888888888888888888");

        grid.routes.insert(point.clone(), found_routes.clone());
    } else {
        println!("22222222222222222222222222222222222222");
        println!("{:?}", point);
    }

    println!("99999999999999999999999999999999999999");
    let my_routes: Vec<Route> = grid.routes.get(point).unwrap().clone();
    println!("Point {:?} - Route Count = {}", point, my_routes.len());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");

    my_routes
}

#[test]
fn test_find_routes_at_point() {
    let mut test_grid = Grid::new(2);
    let starting_point = test_grid.end.clone();
    let routes = find_routes_at_point(&mut test_grid, &starting_point);
    assert_eq!(routes.len(), 6);
}

fn main() {
    let mut grid = Grid::new(3);
    let starting_point = grid.end.clone();
    let routes = find_routes_at_point(&mut grid, &starting_point);
    println!("Total routes for 20: {}", routes.len());
}
