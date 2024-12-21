use crate::types::*;
use tokio::fs;

#[tokio::test]
async fn test_to_building() {
    let shop_item = ShopItem {
        id: 1,
        name: "Test Shop".to_string(),
        purpose: BuildingPurpose::Other("Commercial".to_string()),
        size: (10, 20),
        cost: 100.0,
    };

    let building = shop_item.to_building();
    assert_eq!(building.id, shop_item.id);
    assert_eq!(building.name, shop_item.name);
    assert_eq!(building.purpose, shop_item.purpose);
    assert_eq!(building.size, shop_item.size);
    assert_eq!(building.status, BuildingStatus::Planning);
    assert!(building.coordinates.is_none());
}

#[tokio::test]
async fn test_new_from_json() {
    let json_data = r#"
    [
        {
            "id": 1,
            "name": "Test Shop",
            "purpose": "Residential",
            "size": [10, 20],
            "cost": 100.0
        }
    ]
    "#;

    let path = "test_shop.json";
    fs::write(path, json_data).await.expect("Unable to write test JSON");

    let shop = Shop::new_from_json(path);
    assert_eq!(shop.items.len(), 1);
    assert_eq!(shop.items[0].name, "Test Shop");

    fs::remove_file(path).await.expect("Unable to delete test JSON");
}

#[test]
fn test_purchase() {
    let shop_item = ShopItem {
        id: 1,
        name: "Test Shop".to_string(),
        purpose: BuildingPurpose::Other("Commercial".to_string()),
        size: (10, 20),
        cost: 100.0,
    };

    let shop = Shop {
        items: vec![shop_item],
    };

    let mut player_money = 150.0;
    let building = shop.purchase(1, &mut player_money);
    assert!(building.is_some());
    assert_eq!(player_money, 50.0);

    let mut player_money = 50.0;
    let building = shop.purchase(1, &mut player_money);
    assert!(building.is_none());
    assert_eq!(player_money, 50.0);
}