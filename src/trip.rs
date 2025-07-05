use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Trip {
    trip_duration_days: i32,
    miles_traveled: f32,
    total_receipts_amount: f32,
}

impl Trip {
    pub fn new<A: Into<i32>, B: Into<f32>>(
        trip_duration_days: A,
        miles_traveled: B,
        total_reciepts_amount: B,
    ) -> Self {
        Self {
            trip_duration_days: trip_duration_days.into(),
            miles_traveled: miles_traveled.into(),
            total_receipts_amount: total_reciepts_amount.into(),
        }
    }
    pub fn calculate_output(&self) -> f32 {
        (self.trip_duration_days as f32 * 116.62828)
            + (self.miles_traveled * 0.15293)
            + (self.total_receipts_amount * 0.28346)
    }
}

impl fmt::Display for Trip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!(
            "trip duration (days): {}\n",
            self.trip_duration_days
        ));
        output.push_str(&format!("miles traveled: {}\n", self.miles_traveled));
        output.push_str(&format!(
            "total receipts amount: {}\n",
            self.total_receipts_amount
        ));
        output.push_str(&format!(
            "reimbursement amount: {}\n",
            self.calculate_output()
        ));
        write!(f, "{output}")
    }
}
