// Modulo, not remainder. modulo(-1, 10) -> 9
pub fn modulo(a: isize, b: isize) -> usize {
    (((a % b) + b) % b) as usize
}
