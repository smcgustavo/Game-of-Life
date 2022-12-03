use rand::{thread_rng, Rng};


// Create and return a matrix 200x200
pub fn create_matrix () -> [[i8;200]; 200] {
    let matrix : [[i8; 200]; 200] = [[0;200];200];
    return matrix;
}

// Populates the matrix with 1s and 0s.
pub fn populate_matrix(mut matrix : [[i8; 200]; 200]) -> [[i8; 200]; 200]{
    let mut rng = thread_rng();
    for columns in 1..matrix.len()-1{
        for rows in 1..matrix[columns].len() - 1{
            let value = rng.gen_range(0..2);
            matrix[columns][rows] = value;
        }
    }
    return matrix;
}

// Calculate the number of alive neighbors around the cell
fn alive_neighbors(columns : usize, rows : usize, matrix : [[i8;200];200]) -> i8{
    let mut number_of_neighbors : i8 = 0;
    number_of_neighbors += matrix[columns + 1][rows];
    number_of_neighbors += matrix[columns + 1][rows + 1];
    number_of_neighbors += matrix[columns + 1][rows - 1];
    number_of_neighbors += matrix[columns][rows + 1];
    number_of_neighbors += matrix[columns][rows - 1];
    number_of_neighbors += matrix[columns - 1][rows];
    number_of_neighbors += matrix[columns - 1][rows + 1];
    number_of_neighbors += matrix[columns - 1][rows - 1];
    return number_of_neighbors;
}

// Compute game of life rules
pub fn compute_matrix(matrix : [[i8;200]; 200]) -> [[i8; 200]; 200] {

    let mut final_matrix : [[i8;200]; 200] = [[0; 200]; 200];

    for columns in 1..matrix.len() - 1 {
        for rows in 1..matrix[columns].len() - 1{
            let alive_neighbors = alive_neighbors(columns, rows, matrix);
            match (matrix[columns][rows] == 1, alive_neighbors) {
                (true, 0) | (true, 1) => {
                    final_matrix[columns][rows] = 0;
                }
                (true, 3) | (true, 2) => {
                    final_matrix[columns][rows] = matrix[columns][rows];
                }
                (false, 3) => {
                    final_matrix[columns][rows] = 1;
                }
                (false, _) => {
                    final_matrix[columns][rows] = 0;
                }
                _  => {}
            }
        }
    }
    return final_matrix;
}