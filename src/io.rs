macro_rules! bitwise {
    ($bits:expr, $bit:path) => (($bits & $bit) == $bit)
}