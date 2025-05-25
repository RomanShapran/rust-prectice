use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

/// Обчислює фактичну зайняту площу з урахуванням перекриття
fn area_occupied(rectangles: &[Rectangle]) -> usize {
    let mut occupied_cells = HashSet::new();

    for rect in rectangles {
        let x_start = rect.top_left.x.min(rect.bottom_right.x);
        let x_end = rect.top_left.x.max(rect.bottom_right.x);
        let y_start = rect.top_left.y.min(rect.bottom_right.y);
        let y_end = rect.top_left.y.max(rect.bottom_right.y);

        for x in x_start..x_end {
            for y in y_start..y_end {
                occupied_cells.insert((x, y));
            }
        }
    }

    occupied_cells.len()
}

/// Тестові прямокутники
fn sample_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            top_left: Point { x: 2, y: 9 },
            bottom_right: Point { x: 5, y: 3 },
        },
        Rectangle {
            top_left: Point { x: 5, y: 7 },
            bottom_right: Point { x: 11, y: 8 },
        },
        Rectangle {
            top_left: Point { x: 9, y: 10 },
            bottom_right: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let data = sample_data();
    let occupied_area = area_occupied(&data);
    println!("Occupied area: {}", occupied_area);
}

#[test]
fn test_area_occupied() {
    let data = sample_data();
    assert_eq!(area_occupied(&data), 50);
}
