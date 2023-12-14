#[macro_export]
macro_rules! bitwise {
    ($bits:expr, $bit:path) => (($bits & $bit) == $bit)
}

#[macro_export]
macro_rules! vklog {
    ($content:expr) => (println!("[GPU] {}", $content))
}