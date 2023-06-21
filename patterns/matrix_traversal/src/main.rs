// Given a 2D array (i.e., a matrix) containing only 1s (land) and 0s (water), count the number of
// islands in it.

// An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water). Each
//  cell is considered connected to other cells horizontally or vertically (not diagonally).

fn main() {
    // 1 island
    let matrix: Vec<Vec<i32>> = vec![
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    // 2 island
    let matrix_2: Vec<Vec<i32>> = vec![
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    // 3 island
    let matrix_3: Vec<Vec<i32>> = vec![
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1],
    ];

    check_islands(matrix);
    check_islands(matrix_2);
    check_islands(matrix_3);
}

fn check_islands(mut matrix: Vec<Vec<i32>>) {
    let mut islands = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                follow_island(&mut matrix, i as i32, j as i32);
                islands += 1;
            }
        }
    }
    println!("A total of {} island have been found", islands);
}

fn follow_island(matrix: &mut Vec<Vec<i32>>, x: i32, y: i32) {
    let surroundings = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    matrix[x as usize][y as usize] = 0;
    for point in surroundings {
        let point_x = point.0;
        let point_y = point.1;

        // Invalid range
        if point_x < 0
            || point_x >= matrix[0].len() as i32
            || point_y < 0
            || point_y >= matrix.len() as i32
        {
            continue;
        }

        // Not part of the island
        if matrix[point_x as usize][point_y as usize] == 0 {
            continue;
        }

        follow_island(matrix, point_x, point_y);
    }
}
