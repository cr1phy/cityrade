use crate::diplomacy::{DiplomacyManager, DiplomaticAction, DiplomaticRelation, RelationType};

#[test]
fn test_diplomatic_relation_creation() {
    let relation = DiplomaticRelation::new();
    
    // Проверяем начальные значения
    assert_eq!(relation.relation_type, RelationType::Neutral);
    assert_eq!(relation.reputation, 0);
    assert_eq!(relation.trade_modifier, 1.0);
    assert!(relation.treaties.is_empty());
    assert!(relation.last_actions.is_empty());
}

#[test]
fn test_reputation_change() {
    let mut relation = DiplomaticRelation::new();
    
    // Проверяем изменение репутации в положительную сторону
    relation.change_reputation(30);
    assert_eq!(relation.reputation, 30);
    assert_eq!(relation.relation_type, RelationType::Friendly);
    
    // Проверяем изменение репутации в отрицательную сторону
    relation.change_reputation(-60);
    assert_eq!(relation.reputation, -30);
    assert_eq!(relation.relation_type, RelationType::Tense);
    
    // Проверяем ограничение репутации снизу
    relation.change_reputation(-80);
    assert_eq!(relation.reputation, -100);
    assert_eq!(relation.relation_type, RelationType::Conflict);
    
    // Проверяем ограничение репутации сверху
    relation.change_reputation(210);
    assert_eq!(relation.reputation, 100);
    assert_eq!(relation.relation_type, RelationType::Alliance);
}

#[test]
fn test_trade_modifier_update() {
    let mut relation = DiplomaticRelation::new();
    
    // Проверяем начальное нейтральное состояние
    assert_eq!(relation.reputation, 0);
    assert_eq!(relation.relation_type, RelationType::Neutral);
    assert_eq!(relation.trade_modifier, 1.0);
    
    // Устанавливаем репутацию для альянса
    relation.reputation = 80;
    relation.update_relation_type();
    assert_eq!(relation.relation_type, RelationType::Alliance);
    assert_eq!(relation.trade_modifier, 1.5);
    
    // Устанавливаем репутацию для дружественных отношений
    relation.reputation = 30;
    relation.update_relation_type();
    assert_eq!(relation.relation_type, RelationType::Friendly);
    assert_eq!(relation.trade_modifier, 1.2);
    
    // Устанавливаем репутацию для нейтральных отношений
    relation.reputation = 0;
    relation.update_relation_type();
    assert_eq!(relation.relation_type, RelationType::Neutral);
    assert_eq!(relation.trade_modifier, 1.0);
    
    // Устанавливаем репутацию для напряженных отношений
    relation.reputation = -30;
    relation.update_relation_type();
    assert_eq!(relation.relation_type, RelationType::Tense);
    assert_eq!(relation.trade_modifier, 0.8);
    
    // Устанавливаем репутацию для конфликтных отношений
    relation.reputation = -80;
    relation.update_relation_type();
    assert_eq!(relation.relation_type, RelationType::Conflict);
    assert_eq!(relation.trade_modifier, 0.0);
}

#[test]
fn test_add_action() {
    let mut relation = DiplomaticRelation::new();
    let action = DiplomaticAction::TradeAgreement;
    let time = 12345;
    
    // Добавляем действие и проверяем, что оно сохранено
    relation.add_action(action, time);
    assert_eq!(relation.last_actions.len(), 1);
    assert_eq!(relation.last_actions[0].0, DiplomaticAction::TradeAgreement);
    assert_eq!(relation.last_actions[0].1, 12345);
    
    // Проверяем ограничение истории действий
    for i in 1..15 {
        relation.add_action(DiplomaticAction::ResourceGift, time + i);
    }
    
    // Должно остаться только 10 последних действий
    assert_eq!(relation.last_actions.len(), 10);
    // Первое действие должно быть ResourceGift (старые действия удалены)
    assert_eq!(relation.last_actions[0].0, DiplomaticAction::ResourceGift);
}

#[test]
fn test_diplomacy_manager() {
    let mut manager = DiplomacyManager::new();
    
    // Проверяем, что изначально отношений нет
    assert!(manager.get_relation("faction1", "faction2").is_none());
    
    // Изменяем репутацию между фракциями
    manager.change_reputation("faction1", "faction2", 50);
    
    // Проверяем, что отношения созданы и обновлены
    let relation = manager.get_relation("faction1", "faction2");
    assert!(relation.is_some());
    assert_eq!(relation.unwrap().reputation, 50);
    assert_eq!(relation.unwrap().relation_type, RelationType::Friendly);
    
    // Проверяем коммутативность доступа к отношениям
    let relation_reversed = manager.get_relation("faction2", "faction1");
    assert!(relation_reversed.is_some());
    assert_eq!(relation_reversed.unwrap().reputation, 50);
    
    // Добавляем действие
    manager.register_action("faction1", "faction2", DiplomaticAction::TradeAgreement, 12345);
    
    // Проверяем, что действие добавлено
    let relation = manager.get_relation("faction1", "faction2").unwrap();
    assert_eq!(relation.last_actions.len(), 1);
    assert_eq!(relation.last_actions[0].0, DiplomaticAction::TradeAgreement);
}

#[test]
fn test_set_relation() {
    let mut manager = DiplomacyManager::new();
    let mut relation = DiplomaticRelation::new();
    
    // Меняем тип отношений на дружественные
    relation.change_reputation(30);
    
    // Устанавливаем отношения
    manager.set_relation("faction1", "faction2", relation);
    
    // Проверяем, что отношения установлены правильно
    let stored_relation = manager.get_relation("faction1", "faction2").unwrap();
    assert_eq!(stored_relation.relation_type, RelationType::Friendly);
    assert_eq!(stored_relation.reputation, 30);
    
    // Проверяем коммутативность операции установки отношений
    let stored_relation_reversed = manager.get_relation("faction2", "faction1").unwrap();
    assert_eq!(stored_relation_reversed.relation_type, RelationType::Friendly);
} 