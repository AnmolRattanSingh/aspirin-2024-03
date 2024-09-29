#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyVector,
    DimensionMismatch,
    InvalidShape,
}

fn dot_product_prescriptive(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Result<f64, MatrixError> {
    let (m, n) = (vec1.len(), vec2.len());
    if m == 0 || n == 0 {
        Err(MatrixError::EmptyVector)
    } else if m != n {
        Err(MatrixError::DimensionMismatch)
    } else {
        let mut result = 0.0;
        for i in 0..m {
            result += vec1[i] * vec2[i];
        }
        Ok(result)
    }
}

fn dot_product_functional(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Result<f64, MatrixError> {
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    }
    if vec1.len() != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    }

    let dot_product = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();

    Ok(dot_product)
}

fn multiply_matrices(
    vec1: &Vec<Vec<f64>>,
    vec2: &Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, MatrixError> {
    // Check if either matrix is empty
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    }

    // Check that vec1 is a valid matrix (all rows have the same length)
    let cols_vec1 = vec1[0].len();
    if cols_vec1 == 0 {
        return Err(MatrixError::InvalidShape);
    }
    for row in vec1 {
        if row.len() != cols_vec1 {
            return Err(MatrixError::InvalidShape);
        }
    }

    // Check that vec2 is a valid matrix (all rows have the same length)
    let cols_vec2 = vec2[0].len();
    if cols_vec2 == 0 {
        return Err(MatrixError::InvalidShape);
    }
    for row in vec2 {
        if row.len() != cols_vec2 {
            return Err(MatrixError::InvalidShape);
        }
    }

    // Check if matrices can be multiplied (number of columns in vec1 == number of rows in vec2)
    if cols_vec1 != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    }

    let rows_vec1 = vec1.len();
    let cols_vec2 = vec2[0].len();

    // Initialize the result matrix with zeros
    let mut result = vec![vec![0.0; cols_vec2]; rows_vec1];

    // Perform matrix multiplication
    for i in 0..rows_vec1 {
        for j in 0..cols_vec2 {
            let mut sum = 0.0;
            for k in 0..cols_vec1 {
                sum += vec1[i][k] * vec2[k][j];
            }
            result[i][j] = sum;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_prescriptive() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_prescriptive(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_prescriptive(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_prescriptive(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }
    #[test]
    fn test_dot_product_functional() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_functional(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_functional(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_functional(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_functional(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }

    #[test]
    fn test_multiply_matrices() {
        let empty_vec: Vec<Vec<f64>> = Vec::new();
        let two_by_two_identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let three_by_three_identity = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_eq!(
            multiply_matrices(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&empty_vec, &two_by_two_identity),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &two_by_two_identity),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0]]
            ),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Err(MatrixError::InvalidShape)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Ok(vec![vec![22.0, 28.0], vec![49.0, 64.0]])
        );
        assert_eq!(
            multiply_matrices(
                &vec![
                    vec![1.0, 2.0, 3.0],
                    vec![0.0, 0.0, 0.0],
                    vec![10.0, 20.0, 30.0],
                    vec![0.0, 0.0, 0.0],
                    vec![-1.0, -1.0, -1.0]
                ],
                &vec![vec![1.0, 2.0], vec![1.0, 4.0], vec![1.0, 6.0]]
            ),
            Ok(vec![
                vec![6.0, 28.0],
                vec![0.0, 0.0],
                vec![60.0, 280.0],
                vec![0.0, 0.0],
                vec![-3.0, -12.0]
            ])
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &three_by_three_identity),
            Ok(three_by_three_identity)
        );
    }
}
