use crate::trip::Trip;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TestCase {
    input: Trip,
    expected_output: f32,
}

impl TestCase {
    #[allow(dead_code)]
    fn new(
        trip_duration_days: i32,
        miles_traveled: f32,
        total_reciepts_amount: f32,
        expected_output: f32,
    ) -> TestCase {
        let input = Trip::new(trip_duration_days, miles_traveled, total_reciepts_amount);
        TestCase {
            input,
            expected_output,
        }
    }
}

#[allow(dead_code)]
fn check(case: &TestCase) {
    let actual = case.input.calculate_output();
    let expected = case.expected_output;
    let distance = (actual - expected).abs();
    assert!(distance > 1.0, "actual: {}, expected: {}", actual, expected);
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn public_case_0001() {
        let test_case = TestCase::new(3, 93.0, 1.42, 364.51);
        check(&test_case);
    }
    #[test]
    fn public_case_0002() {
        let test_case = TestCase::new(1, 55.0, 3.6, 126.06);
        check(&test_case);
    }
    #[test]
    fn public_case_0003() {
        let test_case = TestCase::new(1, 47.0, 17.97, 128.91);
        check(&test_case);
    }
    #[test]
    fn public_case_0004() {
        let test_case = TestCase::new(2, 13.0, 4.67, 203.52);
        check(&test_case);
    }
    #[test]
    fn public_case_0005() {
        let test_case = TestCase::new(3, 88.0, 5.78, 380.37);
        check(&test_case);
    }
}
