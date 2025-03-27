// cityrade-types/src/tests/faction_tests.rs
use crate::faction::{Faction, FactionBonus, FactionManager, FactionSpecialization};
use crate::resources::ResourceType;

#[test]
fn test_faction_creation() {
    let faction = Faction::new(
        "test_faction".to_string(),
        "Тестовая Фракция".to_string(),
        FactionSpecialization::Trade,
    );

    assert_eq!(faction.id, "test_faction");
    assert_eq!(faction.name, "Тестовая Фракция");
    assert_eq!(faction.specialization, FactionSpecialization::Trade);
    assert!(!faction.bonuses.is_empty());
}

#[test]
fn test_resource_production_modifier() {
    let mut faction = Faction::new(
        "test_faction".to_string(),
        "Тестовая Фракция".to_string(),
        FactionSpecialization::Industry,
    );

    // Добавляем дополнительный бонус к производству железа
    faction.bonuses.push(FactionBonus::ResourceProduction(ResourceType::Iron, 10));

    // Теперь должен быть суммарный бонус 30% (20% по умолчанию + 10% новый)
    let iron_bonus = faction.get_resource_production_modifier(&ResourceType::Iron);
    assert_eq!(iron_bonus, 30);

    // Проверяем стандартный бонус камня для промышленной фракции
    let stone_bonus = faction.get_resource_production_modifier(&ResourceType::Stone);
    assert_eq!(stone_bonus, 20);

    // Проверяем, что для других ресурсов без бонусов возвращается 0
    let food_bonus = faction.get_resource_production_modifier(&ResourceType::Food);
    assert_eq!(food_bonus, 0);
}

#[test]
fn test_modifier_for_type() {
    let faction = Faction::new(
        "test_faction".to_string(),
        "Тестовая Фракция".to_string(),
        FactionSpecialization::Military,
    );

    // Проверяем военный бонус для военной фракции
    let military_bonus = faction.get_modifier_for_type(|bonus| match bonus {
        FactionBonus::MilitaryStrength(value) => Some(*value),
        _ => None,
    });
    assert_eq!(military_bonus, 25);

    // Проверяем дипломатический штраф для военной фракции
    let diplomatic_bonus = faction.get_modifier_for_type(|bonus| match bonus {
        FactionBonus::DiplomaticInfluence(value) => Some(*value),
        _ => None,
    });
    assert_eq!(diplomatic_bonus, -15);
}

#[test]
fn test_faction_manager() {
    let mut manager = FactionManager::new();
    
    // Создаем тестовую фракцию
    let faction = Faction::new(
        "test_faction".to_string(),
        "Тестовая Фракция".to_string(),
        FactionSpecialization::Trade,
    );
    
    // Добавляем фракцию
    manager.add_faction(faction);
    
    // Проверяем, что фракция добавлена
    let retrieved_faction = manager.get_faction("test_faction");
    assert!(retrieved_faction.is_some());
    assert_eq!(retrieved_faction.unwrap().name, "Тестовая Фракция");
    
    // Проверяем, что изначально нет фракции игрока
    assert!(manager.get_player_faction().is_none());
    
    // Устанавливаем фракцию игрока
    let result = manager.set_player_faction("test_faction");
    assert!(result);
    
    // Проверяем, что фракция игрока установлена правильно
    let player_faction = manager.get_player_faction();
    assert!(player_faction.is_some());
    assert_eq!(player_faction.unwrap().id, "test_faction");
}

#[test]
fn test_create_default_factions() {
    let mut manager = FactionManager::new();
    manager.create_default_factions();
    
    // Проверяем, что созданы все 5 стандартных фракций
    let factions = manager.get_all_factions();
    assert_eq!(factions.len(), 5);
    
    // Проверяем, что все основные фракции присутствуют
    assert!(manager.get_faction("trade_alliance").is_some());
    assert!(manager.get_faction("industrial_guild").is_some());
    assert!(manager.get_faction("naturalist_order").is_some());
    assert!(manager.get_faction("technocrats").is_some());
    assert!(manager.get_faction("military_coalition").is_some());
    
    // Проверяем, что фракция имеет правильную специализацию
    let trade_faction = manager.get_faction("trade_alliance").unwrap();
    assert_eq!(trade_faction.specialization, FactionSpecialization::Trade);
    
    // Проверяем, что фракция имеет правильное название
    let military_faction = manager.get_faction("military_coalition").unwrap();
    assert_eq!(military_faction.name, "Военная Коалиция");
} 