#[derive(Debug, Clone, PartialEq)]
pub struct Knot {
    pub id: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

impl Knot {
    pub fn new(id: u64, timestamp: u64, payload: Vec<u8>) -> Self {
        Self { id, timestamp, payload }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_struct_construction() {
        let test_id: u64 = 1;
        let test_timestamp: u64 = 1_000_000_000;
        let test_payload: Vec<u8> = vec![0x42];

        let knot = Knot::new(test_id, test_timestamp, test_payload.clone());

        assert_eq!(knot.id, test_id);
        assert_eq!(knot.timestamp, test_timestamp);
        assert_eq!(knot.payload, test_payload);
    
        assert!(knot.clone() == knot);
        assert!(format!("{:?}", knot).contains("Knot"));
    }
}