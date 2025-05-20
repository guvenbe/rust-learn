fn main() {
    let matrix =  [
        [2,4,6],
        [8, 9 ,10],
        [12,14,16],
    ];
    
    'rows: for row in matrix.iter() {
        'cols: for col in row{
            if col % 2 == 1 {
                println!("odd: {}", col);
                break 'rows;
            }
            println!("{}", col);
        }
    }
}