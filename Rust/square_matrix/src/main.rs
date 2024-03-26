use square_matrix::square_matrix::SquareMatrix;

fn main() {
    let mut matrix: SquareMatrix<i32> = SquareMatrix::new(5);
    matrix.set_all_to(10);
    matrix.print();
}
