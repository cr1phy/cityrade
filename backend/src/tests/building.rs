use crate::types::{Building, BuildingPurpose, BuildingStatus};

#[test]
fn test_new_building() {
    let building = Building::new(1, "Test Building", BuildingPurpose::Residential, (10, 20));
    assert_eq!(building.id, 1);
    assert_eq!(building.name, "Test Building");
    assert_eq!(building.purpose, BuildingPurpose::Residential);
    assert_eq!(building.size, (10, 20));
    assert_eq!(building.status, BuildingStatus::Planning);
    assert!(building.coordinates.is_none());
}