pub fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && b + c > a && c + a > b
}

#[cfg(test)]
mod tests {
    use super::is_triangle;

    #[test]
    fn is_vaild_triangle() {
        assert_eq!(is_triangle(3, 4, 5), true);
        assert_eq!(is_triangle(5, 12, 13), true);
        assert_eq!(is_triangle(1, 2, 3), false);
    }
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Should not divide by zero");
    }
    return a / b;
}

#[cfg(test)]
mod test_divide {
    use super::divide;
    #[test]
    #[should_panic]
    fn divide_by_zero() {
        divide(1, 0);
    }
}