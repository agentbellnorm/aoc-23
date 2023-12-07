DAY=$1

echo "generating day $DAY skeleton"

echo "
use crate::util::{read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a(\"src/input/${DAY}_short.txt\"), 0);
    assert_eq!(get_a(\"src/input/${DAY}.txt\"), 0)
}

#[test]
fn b() {
    assert_eq!(get_b(\"src/input/${DAY}_short.txt\"), 0);
    assert_eq!(get_b(\"src/input/${DAY}.txt\"), 0)
}

pub fn get_b(file: &str) -> i32 {
    split_lines(read_file(file).as_str()).into_iter().len() as i32
}

pub fn get_a(file: &str) -> i32 {
    split_lines(read_file(file).as_str()).into_iter().len() as i32
}
" > src/$DAY.rs

echo "mod $DAY;\n$(cat src/main.rs)" > src/main.rs
touch src/input/$DAY.txt
touch src/input/$DAY\_short.txt

git add .

cargo test