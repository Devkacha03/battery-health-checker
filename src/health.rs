/// Health rating buckets based on capacity retention percentage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthRating {
    Excellent,
    Good,
    Fair,
    Poor,
    Unknown,
}

impl HealthRating {
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthRating::Excellent => "Excellent",
            HealthRating::Good => "Good",
            HealthRating::Fair => "Fair",
            HealthRating::Poor => "Poor",
            HealthRating::Unknown => "Unknown",
        }
    }
}

/// Computes health percentage as (full_charge_capacity / design_capacity) * 100.
/// Returns None if design capacity is zero or unavailable (avoids div-by-zero).
pub fn compute_health_percent(full_charge: f32, design: f32) -> Option<f32> {
    if design <= 0.0 {
        return None;
    }
    Some((full_charge / design) * 100.0)
}

/// Buckets a health percentage into a human-readable rating.
pub fn health_rating(percent: Option<f32>) -> HealthRating {
    match percent {
        None => HealthRating::Unknown,
        Some(p) if p > 90.0 => HealthRating::Excellent,
        Some(p) if p >= 80.0 => HealthRating::Good,
        Some(p) if p >= 60.0 => HealthRating::Fair,
        Some(_) => HealthRating::Poor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_health_percent_normal() {
        let health = compute_health_percent(45000.0, 50000.0).unwrap();
        assert!((health - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_health_percent_zero_design() {
        assert_eq!(compute_health_percent(45000.0, 0.0), None);
    }

    #[test]
    fn test_health_rating_buckets() {
        assert_eq!(health_rating(Some(95.0)), HealthRating::Excellent);
        assert_eq!(health_rating(Some(85.0)), HealthRating::Good);
        assert_eq!(health_rating(Some(65.0)), HealthRating::Fair);
        assert_eq!(health_rating(Some(40.0)), HealthRating::Poor);
        assert_eq!(health_rating(None), HealthRating::Unknown);
    }

    #[test]
    fn test_health_rating_boundaries() {
        // Boundaries are inclusive on the lower bound per the match arms above.
        assert_eq!(health_rating(Some(90.0)), HealthRating::Good); // not > 90
        assert_eq!(health_rating(Some(80.0)), HealthRating::Good);
        assert_eq!(health_rating(Some(60.0)), HealthRating::Fair);
    }
}
