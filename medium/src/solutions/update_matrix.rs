use crate::models::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix_dp(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::min;

        let rows = mat.len();
        let cols = mat[0].len();
        let max_distance = rows + cols; // Maximum possible distance in the matrix

        // First pass: Traverse from top-left to bottom-right
        for row in 0..rows {
            for col in 0..cols {
                // If the cell is not a 0, update its value based on top and left neighbors
                if mat[row][col] != 0 {
                    let mut top = max_distance as i32;
                    let mut left = max_distance as i32;

                    // Check the top neighbor, if it exists
                    if row > 0 {
                        top = mat[row - 1][col];
                    }

                    // Check the left neighbor, if it exists
                    if col > 0 {
                        left = mat[row][col - 1];
                    }

                    // Update the current cell with the minimum distance from top or left + 1
                    mat[row][col] = min(top, left) + 1;
                }
            }
        }

        for row in 0..rows {
            println!("{:?}", mat[row]);
        }

        // Second pass: Traverse from bottom-right to top-left
        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                // If the cell is not a 0, update its value based on bottom and right neighbors
                if mat[row][col] != 0 {
                    let mut bottom = max_distance as i32;
                    let mut right = max_distance as i32;

                    // Check the bottom neighbor, if it exists
                    if row + 1 < rows {
                        bottom = mat[row + 1][col];
                    }

                    // Check the right neighbor, if it exists
                    if col + 1 < cols {
                        right = mat[row][col + 1];
                    }

                    // Update the current cell with the minimum of its current value, bottom, or right + 1
                    mat[row][col] = min(mat[row][col], min(bottom, right) + 1);
                }
            }
        }

        // Return the updated matrix with minimum distances
        mat
    }
}

impl Solution {
    pub fn update_matrix_bfs(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = mat.len();
        let cols = mat[0].len();

        // Directions array representing moves: right, down, left, up
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Queue to hold coordinates of cells to be processed
        let mut queue = VecDeque::new();

        // Step 1: Initialize the queue with all '0' cells and mark '1' cells as -1 (unvisited)
        for row in 0..rows {
            for col in 0..cols {
                if mat[row][col] == 0 {
                    // Enqueue all '0' cells as starting points for BFS
                    queue.push_back((row, col));
                } else {
                    // Mark '1' cells as -1 to indicate they haven't been processed
                    mat[row][col] = -1;
                }
            }
        }

        // Step 2: Perform BFS to calculate minimum distances
        while let Some((row, col)) = queue.pop_front() {
            // Explore all 4 possible directions
            for (dir_row, dir_col) in &directions {
                let neighbour_row = row as isize + dir_row;
                let neighbour_col = col as isize + dir_col;

                // Check if the new coordinates are within bounds and the cell is unvisited
                if neighbour_row >= 0
                    && neighbour_row < rows as isize
                    && neighbour_col >= 0
                    && neighbour_col < cols as isize
                {
                    let (new_row, new_col) = (neighbour_row as usize, neighbour_col as usize);

                    if mat[new_row][new_col] == -1 {
                        // Update the distance of the new cell to be 1 more than the current cell
                        mat[new_row][new_col] = mat[row][col] + 1;

                        // Enqueue the new cell to continue BFS from there
                        queue.push_back((new_row, new_col));
                    }
                }
            }
        }

        // Return the updated matrix with minimum distances
        mat
    }
}
