fn main() {
    let num1= 5;
    let num2= 10;
    println!("Before swap: num1 = {}, num2 = {}", num1, num2);
    let (num1,num2) = swap(num1,num2);
    println!("After swap: num1 = {}, num2 = {}", num1, num2);
}
fn swap(num3: i32, num4: i32) -> (i32, i32) {
    (num4, num3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap(-1, -2), (-2, -1));
        assert_eq!(swap(0, 100), (100, 0));
        assert_eq!(swap(12345, 54321), (54321, 12345));
    }
}



//Reference
//https://doc.rust-lang.org/rustdoc/advanced-features.html