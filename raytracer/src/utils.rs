pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}
