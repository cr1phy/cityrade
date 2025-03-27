# Документация модуля Diplomacy

Модуль `diplomacy` предоставляет типы и структуры для управления дипломатическими отношениями между фракциями в Cityrade.

## Типы данных

### RelationType

Перечисление, определяющее типы дипломатических отношений между фракциями.

```rust
pub enum RelationType {
    Alliance,  // Полное сотрудничество, общая оборона, торговля без пошлин
    Friendly,  // Дружеские отношения, сниженные торговые пошлины
    Neutral,   // Стандартные отношения без бонусов и штрафов (по умолчанию)
    Tense,     // Повышенные пошлины, ограничения на передвижение
    Conflict,  // Торговое эмбарго, возможность военных действий
}
```

### DiplomaticAction

Перечисление, представляющее различные дипломатические действия, которые могут быть выполнены между фракциями.

```rust
pub enum DiplomaticAction {
    TradeAgreement,   // Улучшает экономические отношения и снижает пошлины
    CulturalExchange, // Повышает репутацию и влияние во фракции
    ResourceGift,     // Быстрое улучшение отношений
    JointResearch,    // Ускоряет развитие технологий
    Sanctions,        // Накладывает экономические ограничения
    Espionage,        // Получение информации о технологиях и ресурсах другой фракции
    Ultimatum,        // Требование с угрозой конфликта
}
```

### DiplomaticRelation

Структура, представляющая дипломатические отношения между двумя фракциями.

```rust
pub struct DiplomaticRelation {
    pub relation_type: RelationType,
    pub reputation: i32,                       // от -100 до 100
    pub trade_modifier: f32,                   // множитель торговых сделок
    pub treaties: Vec<String>,                 // активные соглашения
    pub last_actions: Vec<(DiplomaticAction, u64)>, // действие и время действия
}
```

#### Методы DiplomaticRelation

- `new() -> Self` - Создает новые отношения с нейтральным типом.
- `update_relation_type(&mut self)` - Обновляет тип отношений на основе репутации.
- `change_reputation(&mut self, amount: i32)` - Изменяет репутацию и обновляет отношения.
- `add_action(&mut self, action: DiplomaticAction, time: u64)` - Добавляет дипломатическое действие в историю.

### DiplomacyManager

Структура для управления всеми дипломатическими отношениями между фракциями.

```rust
pub struct DiplomacyManager {
    relations: HashMap<(String, String), DiplomaticRelation>,
}
```

#### Методы DiplomacyManager

- `new() -> Self` - Создает новый менеджер дипломатии.
- `get_relation(&self, faction1: &str, faction2: &str) -> Option<&DiplomaticRelation>` - Получает отношения между двумя фракциями.
- `set_relation(&mut self, faction1: &str, faction2: &str, relation: DiplomaticRelation)` - Устанавливает отношения между двумя фракциями.
- `change_reputation(&mut self, faction1: &str, faction2: &str, amount: i32)` - Изменяет репутацию между двумя фракциями.
- `register_action(&mut self, faction1: &str, faction2: &str, action: DiplomaticAction, time: u64)` - Регистрирует дипломатическое действие.

## Примеры использования

### Создание и изменение отношений

```rust
use cityrade_types::diplomacy::{DiplomacyManager, DiplomaticAction, RelationType};

// Создаем менеджер дипломатии
let mut diplomacy = DiplomacyManager::new();

// Устанавливаем дружественные отношения между фракциями
diplomacy.change_reputation("trade_alliance", "naturalist_order", 30);

// Проверяем текущие отношения
if let Some(relation) = diplomacy.get_relation("trade_alliance", "naturalist_order") {
    println!("Репутация: {}", relation.reputation); // Выведет: Репутация: 30
    println!("Тип отношений: {:?}", relation.relation_type); // Выведет: Тип отношений: Friendly
    println!("Торговый модификатор: {}", relation.trade_modifier); // Выведет: Торговый модификатор: 1.2
}

// Регистрируем дипломатическое действие
diplomacy.register_action(
    "trade_alliance",
    "naturalist_order",
    DiplomaticAction::TradeAgreement,
    12345 // время действия
);
```

### Изменение торговых отношений

```rust
use cityrade_types::diplomacy::{DiplomaticRelation, RelationType};

// Создаем отношения
let mut relation = DiplomaticRelation::new();

// По умолчанию отношения нейтральные
assert_eq!(relation.relation_type, RelationType::Neutral);
assert_eq!(relation.trade_modifier, 1.0);

// Улучшаем отношения до дружественных
relation.change_reputation(30);
assert_eq!(relation.relation_type, RelationType::Friendly);
assert_eq!(relation.trade_modifier, 1.2);

// Улучшаем отношения до альянса
relation.change_reputation(50);
assert_eq!(relation.relation_type, RelationType::Alliance);
assert_eq!(relation.trade_modifier, 1.5);

// Ухудшаем отношения до конфликта
relation.change_reputation(-180);
assert_eq!(relation.relation_type, RelationType::Conflict);
assert_eq!(relation.trade_modifier, 0.0); // Торговое эмбарго - торговля запрещена
```

## Диапазоны репутации и соответствующие типы отношений

Репутация измеряется по шкале от -100 до 100 и определяет тип отношений между фракциями:

- **Альянс** (Alliance): репутация ≥ 75
  - Торговый модификатор: 1.5 (бонус 50%)
  - Преимущества: общая оборона, торговля без пошлин, обмен технологиями

- **Дружественные** (Friendly): репутация ≥ 25
  - Торговый модификатор: 1.2 (бонус 20%)
  - Преимущества: снижение торговых пошлин, совместные торговые маршруты

- **Нейтральные** (Neutral): репутация > -25
  - Торговый модификатор: 1.0 (без бонусов/штрафов)
  - Преимущества: стандартные отношения

- **Напряженные** (Tense): репутация > -75
  - Торговый модификатор: 0.8 (штраф 20%)
  - Недостатки: повышенные пошлины, ограничения на передвижение

- **Конфликт** (Conflict): репутация ≤ -75
  - Торговый модификатор: 0.0 (торговля запрещена)
  - Недостатки: торговое эмбарго, возможность военных действий

## Рекомендации по интеграции

1. При инициализации игрового мира создайте `DiplomacyManager` для управления отношениями между фракциями.

2. Используйте метод `change_reputation` для изменения отношений между фракциями в ответ на действия игрока или события в игре.

3. Применяйте `trade_modifier` при расчете стоимости торговых сделок между фракциями.

4. Используйте тип отношений `relation_type` для определения доступности дипломатических действий и ограничений.

5. Регистрируйте дипломатические действия с помощью метода `register_action` для ведения истории отношений.

6. Имейте в виду, что отношения симметричны - при получении отношений между фракциями A и B результат будет тем же, что и между B и A. 