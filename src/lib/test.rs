pub fn add(x: i128, y: i128) -> i128 {
    x + y
}


// =====  divide======
pub fn div(x: f64, y: f64)->Result< f64, String> {
    if y == 0.0 {
        return Err("attempt to trigger division by zero".to_string());
    }
    Ok(x/y)


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_add_two_integers_and_it_work() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 10);
    }
    
    #[test]
    fn test_divide_two_numbers_and_it_works () {
        // arrange
        let divident: f64 = 4.0;
        let divisor: f64 =2.0;
        let expected_result: f64 = 2.0;
        // action
        let result:Result<f64, String>  = div(divident, divisor);
        // assert
        assert_eq!(result.unwrap(), expected_result);

    }

    #[test]
    fn test_should_return_error_if_divisor_is_zero () {
        // arrange
        let divident: f64 = 4.0;
        let divisor: f64 =0.0;
        let expected_result: String = ("attempt to trigger division by zero").to_string();
        // action
        let result:Result<f64, String>  = div(divident, divisor);
        // assert
        assert_eq!(result.unwrap_err(), expected_result);

    }
}
