# Система рынка и торговли (Market System)

Система рынка в Cityrade предоставляет механизмы для торговли ресурсами, формирования цен на основе спроса и предложения, а также установления торговых маршрутов между городами.

## Основные структуры данных

### StockLevel (Уровень запасов)

Перечисление `StockLevel` определяет уровень запасов ресурса на рынке, что влияет на его цену:

```rust
pub enum StockLevel {
    Shortage,    // Острая нехватка (цены очень высокие)
    Low,         // Низкий уровень (цены выше нормы)
    Normal,      // Нормальный уровень (цены близки к базовым)
    Abundant,    // Большие запасы (цены ниже нормы)
    Surplus,     // Избыток (цены очень низкие)
}
```

### PriceTrend (Тренд цены)

Перечисление `PriceTrend` показывает текущий тренд изменения цены ресурса:

```rust
pub enum PriceTrend {
    StrongRise,  // Сильный рост
    Rise,        // Рост
    Stable,      // Стабильность
    Fall,        // Падение
    StrongFall,  // Сильное падение
}
```

### MarketItem (Товар на рынке)

Структура `MarketItem` представляет ресурс на рынке:

```rust
pub struct MarketItem {
    pub resource_type: ResourceType, // Тип ресурса
    pub quantity: u32,              // Количество
    pub base_price: f32,            // Базовая цена
    pub current_price: f32,         // Текущая цена
    pub stock_level: StockLevel,    // Уровень запасов
    pub volatility: f32,            // Волатильность цены (0.0-1.0)
    pub last_price_change: f32,     // Последнее изменение цены
}
```

### Market (Рынок)

Структура `Market` представляет рынок города:

```rust
pub struct Market {
    items: HashMap<ResourceType, MarketItem>,    // Товары на рынке
    demand_factor: HashMap<ResourceType, f32>,   // Фактор спроса для каждого ресурса
    supply_factor: HashMap<ResourceType, f32>,   // Фактор предложения для каждого ресурса
    market_health: f32,                          // Здоровье рынка (0.0-1.0)
    last_update: u64,                            // Время последнего обновления
}
```

### TradeRoute (Торговый маршрут)

Структура `TradeRoute` представляет торговый маршрут между двумя городами:

```rust
pub struct TradeRoute {
    pub source_city: String,        // Город-источник
    pub target_city: String,        // Город-получатель
    pub resource_type: ResourceType, // Перевозимый ресурс
    pub quantity: u32,              // Количество
    pub price_per_unit: f32,        // Цена за единицу
    pub duration: u32,              // Длительность в ходах
}
```

### TradeManager (Менеджер торговли)

Структура `TradeManager` управляет всеми торговыми маршрутами и рынками городов:

```rust
pub struct TradeManager {
    pub trade_routes: Vec<TradeRoute>,                // Активные торговые маршруты
    pub city_markets: HashMap<String, Market>,        // Рынки городов
}
```

## Основные методы

### Создание рынка

```rust
let market = Market::new();
```

Конструктор `new()` создает новый рынок с предопределенным набором стандартных ресурсов и начальных цен.

### Обновление цен на рынке

```rust
market.update_prices(current_turn);
```

Метод `update_prices()` обновляет цены всех ресурсов на рынке в зависимости от спроса, предложения, волатильности ресурсов и общего состояния рынка.

### Покупка и продажа ресурсов

```rust
// Покупка ресурса
if let Some(total_cost) = market.buy(&ResourceType::Wood, 25) {
    println!("Куплено дерева: 25 единиц за {} золота", total_cost);
    player_resources.gold -= total_cost as u32;
    player_resources.wood += 25;
} else {
    println!("Невозможно купить дерево: недостаточно на рынке");
}

// Продажа ресурса
if let Some(revenue) = market.sell(&ResourceType::Iron, 10) {
    println!("Продано железа: 10 единиц за {} золота", revenue);
    player_resources.gold += revenue as u32;
    player_resources.iron -= 10;
} else {
    println!("Ошибка при продаже железа");
}
```

### Получение информации о ресурсе

```rust
// Получить текущую цену покупки
if let Some(price) = market.get_buy_price(&ResourceType::Food) {
    println!("Текущая цена покупки еды: {} за единицу", price);
}

// Получить текущую цену продажи (обычно ниже цены покупки)
if let Some(price) = market.get_sell_price(&ResourceType::Food) {
    println!("Текущая цена продажи еды: {} за единицу", price);
}

// Получить информацию о тренде цены
if let Some(trend) = market.get_price_trend(&ResourceType::Gold) {
    match trend {
        PriceTrend::StrongRise => println!("Цена на золото стремительно растет!"),
        PriceTrend::Rise => println!("Цена на золото растет"),
        PriceTrend::Stable => println!("Цена на золото стабильна"),
        PriceTrend::Fall => println!("Цена на золото падает"),
        PriceTrend::StrongFall => println!("Цена на золото стремительно падает!"),
    }
}
```

### Применение рыночного шока

```rust
// Применение глобального шока (например, экономический кризис)
market.apply_market_shock(0.3, None); // 30% шок для всех ресурсов

// Применение шока к конкретным ресурсам (например, засуха)
let affected_resources = vec![ResourceType::Food, ResourceType::Wood];
market.apply_market_shock(0.2, Some(affected_resources)); // 20% шок для еды и дерева
```

### Создание торгового маршрута

```rust
let mut trade_manager = TradeManager::new();

// Создаем рынки для двух городов
let capital_market = trade_manager.create_city_market("Столица");
let port_market = trade_manager.create_city_market("Порт");

// Устанавливаем торговый маршрут между городами
let route = TradeRoute {
    source_city: "Столица".to_string(),
    target_city: "Порт".to_string(),
    resource_type: ResourceType::Wood,
    quantity: 50,
    price_per_unit: 8.5,
    duration: 3, // 3 хода
};

trade_manager.add_trade_route(route);
```

### Обновление торговых маршрутов и рынков

```rust
// В основном игровом цикле, каждый ход
trade_manager.update_all_markets(current_turn);
```

## Примеры использования

### Пример 1: Простой рыночный интерфейс

```rust
fn display_market_screen(market: &Market, player_resources: &Resources) {
    println!("=== РЫНОК ===\n");
    println!("Ваше золото: {}", player_resources.get(&ResourceType::Gold));
    println!("\nДоступные товары:");
    
    let resources = [
        ResourceType::Wood,
        ResourceType::Stone,
        ResourceType::Food,
        ResourceType::Iron,
        ResourceType::Crystal,
    ];
    
    for resource in resources.iter() {
        if let Some(item) = market.get_market_item(resource) {
            let buy_price = market.get_buy_price(resource).unwrap_or(0.0);
            let sell_price = market.get_sell_price(resource).unwrap_or(0.0);
            let trend = market.get_price_trend(resource).unwrap_or(PriceTrend::Stable);
            let trend_symbol = match trend {
                PriceTrend::StrongRise => "↑↑",
                PriceTrend::Rise => "↑",
                PriceTrend::Stable => "→",
                PriceTrend::Fall => "↓",
                PriceTrend::StrongFall => "↓↓",
            };
            
            let stock_status = match item.stock_level {
                StockLevel::Shortage => "Дефицит!",
                StockLevel::Low => "Мало",
                StockLevel::Normal => "Норма",
                StockLevel::Abundant => "Много",
                StockLevel::Surplus => "Избыток",
            };
            
            println!("{:?} {} | Покупка: {:.1}, Продажа: {:.1} | В наличии: {} ({}) {}",
                resource, trend_symbol, buy_price, sell_price, item.quantity, stock_status,
                if item.quantity < 100 { "⚠️" } else { "" }
            );
        }
    }
    
    println!("\nВведите команду (buy/sell [ресурс] [количество] или exit):");
}

fn handle_market_command(cmd: &str, market: &mut Market, player_resources: &mut Resources) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    
    if parts.len() < 3 {
        println!("Неверная команда. Используйте: buy/sell [ресурс] [количество]");
        return;
    }
    
    let action = parts[0];
    let resource_name = parts[1];
    let amount = match parts[2].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Неверное количество");
            return;
        }
    };
    
    let resource_type = match resource_name.to_lowercase().as_str() {
        "wood" | "дерево" => ResourceType::Wood,
        "stone" | "камень" => ResourceType::Stone,
        "food" | "еда" => ResourceType::Food,
        "iron" | "железо" => ResourceType::Iron,
        "crystal" | "кристалл" => ResourceType::Crystal,
        _ => {
            println!("Неизвестный ресурс: {}", resource_name);
            return;
        }
    };
    
    match action.to_lowercase().as_str() {
        "buy" => {
            let gold = player_resources.get(&ResourceType::Gold);
            if let Some(total_cost) = market.get_buy_price(&resource_type).map(|p| p * amount as f32) {
                if gold >= total_cost as u32 {
                    if let Some(_) = market.buy(&resource_type, amount) {
                        player_resources.subtract(&ResourceType::Gold, total_cost as u32);
                        player_resources.add(&resource_type, amount);
                        println!("Куплено {} ед. {:?} за {:.1} золота", amount, resource_type, total_cost);
                    } else {
                        println!("Недостаточно ресурса на рынке");
                    }
                } else {
                    println!("Недостаточно золота (требуется {:.1})", total_cost);
                }
            } else {
                println!("Ресурс недоступен для покупки");
            }
        },
        "sell" => {
            let owned_amount = player_resources.get(&resource_type);
            if owned_amount >= amount {
                if let Some(revenue) = market.sell(&resource_type, amount) {
                    player_resources.subtract(&resource_type, amount);
                    player_resources.add(&ResourceType::Gold, revenue as u32);
                    println!("Продано {} ед. {:?} за {:.1} золота", amount, resource_type, revenue);
                } else {
                    println!("Не удалось продать ресурс");
                }
            } else {
                println!("Недостаточно ресурса (у вас {} ед.)", owned_amount);
            }
        },
        _ => println!("Неизвестное действие: {}. Используйте buy или sell", action),
    }
}
```

### Пример 2: Торговая система с несколькими городами

```rust
struct GameWorld {
    cities: HashMap<String, City>,
    trade_manager: TradeManager,
    current_turn: u64,
}

impl GameWorld {
    fn new() -> Self {
        let mut world = GameWorld {
            cities: HashMap::new(),
            trade_manager: TradeManager::new(),
            current_turn: 0,
        };
        
        // Создаем несколько городов
        let city_names = ["Столица", "Порт", "Горный город", "Лесное поселение"];
        for name in city_names.iter() {
            world.cities.insert(name.to_string(), City::new(name));
            world.trade_manager.create_city_market(name);
        }
        
        // Устанавливаем специализацию городов
        world.trade_manager.get_city_market_mut("Лесное поселение").map(|market| {
            market.add_resource(ResourceType::Wood, 8.0, 1000, 0.05);
        });
        
        world.trade_manager.get_city_market_mut("Горный город").map(|market| {
            market.add_resource(ResourceType::Stone, 12.0, 800, 0.08);
            market.add_resource(ResourceType::Iron, 20.0, 500, 0.1);
        });
        
        world.trade_manager.get_city_market_mut("Порт").map(|market| {
            market.add_resource(ResourceType::Food, 4.0, 1200, 0.12);
        });
        
        world
    }
    
    fn update(&mut self) {
        self.current_turn += 1;
        
        // Обновляем все города
        for city in self.cities.values_mut() {
            city.update();
        }
        
        // Обновляем все рынки
        self.trade_manager.update_all_markets(self.current_turn);
        
        // Каждые 5 ходов создаем новый торговый маршрут между случайными городами
        if self.current_turn % 5 == 0 {
            self.generate_random_trade_route();
        }
    }
    
    fn generate_random_trade_route(&mut self) {
        let mut rng = rand::thread_rng();
        let city_names: Vec<String> = self.cities.keys().cloned().collect();
        
        if city_names.len() < 2 {
            return;
        }
        
        let source_idx = rng.gen_range(0..city_names.len());
        let mut target_idx = rng.gen_range(0..city_names.len());
        while target_idx == source_idx {
            target_idx = rng.gen_range(0..city_names.len());
        }
        
        let source_city = &city_names[source_idx];
        let target_city = &city_names[target_idx];
        
        // Определяем ресурсы, доступные в городе-источнике
        if let Some(source_market) = self.trade_manager.get_city_market(source_city) {
            let available_resources: Vec<ResourceType> = source_market.get_all_resources().into_iter()
                .filter(|r| *r != ResourceType::Gold && source_market.has_resource(r, 100))
                .collect();
            
            if available_resources.is_empty() {
                return;
            }
            
            let resource = available_resources[rng.gen_range(0..available_resources.len())].clone();
            let quantity = rng.gen_range(20..100);
            
            if let Some(buy_price) = source_market.get_buy_price(&resource) {
                let price_per_unit = buy_price * 0.9; // Небольшая скидка для торгового маршрута
                let duration = rng.gen_range(2..6);
                
                let route = TradeRoute {
                    source_city: source_city.clone(),
                    target_city: target_city.clone(),
                    resource_type: resource.clone(),
                    quantity,
                    price_per_unit,
                    duration,
                };
                
                self.trade_manager.add_trade_route(route);
                
                println!("Создан торговый маршрут: {} -> {} ({} ед. {:?} за {} ходов)",
                    source_city, target_city, quantity, resource, duration);
            }
        }
    }
}
```

## Интеграция с другими системами

### Интеграция с системой ресурсов

```rust
// В классе города
fn update_resources(&mut self) {
    // Базовое производство ресурсов
    for (resource_type, production_rate) in &self.resource_production {
        self.resources.add(resource_type, *production_rate);
    }
    
    // Обновляем цены на рынке
    if let Some(market) = self.world.trade_manager.get_city_market_mut(&self.name) {
        market.update_prices(self.world.current_turn);
    }
    
    // Продаем излишки и покупаем недостающие ресурсы, если включена автоторговля
    if self.auto_trade_enabled {
        self.auto_trade();
    }
}

fn auto_trade(&mut self) {
    if let Some(market) = self.world.trade_manager.get_city_market_mut(&self.name) {
        let resources_to_check = [
            ResourceType::Food,
            ResourceType::Wood,
            ResourceType::Stone,
        ];
        
        for resource in resources_to_check.iter() {
            let current = self.resources.get(resource);
            let min_required = self.min_resource_requirements.get(resource).cloned().unwrap_or(0);
            let max_storage = self.max_resource_storage.get(resource).cloned().unwrap_or(1000);
            
            // Если ресурсов слишком много, продаем излишки
            if current > max_storage * 0.9 {
                let amount_to_sell = (current - max_storage * 0.8) as u32;
                if let Some(revenue) = market.sell(resource, amount_to_sell) {
                    self.resources.subtract(resource, amount_to_sell);
                    self.resources.add(&ResourceType::Gold, revenue as u32);
                    println!("{}: Автопродажа {} ед. {:?} за {:.1} золота", 
                        self.name, amount_to_sell, resource, revenue);
                }
            }
            
            // Если ресурсов слишком мало, покупаем
            if current < min_required * 0.5 {
                let amount_to_buy = (min_required * 0.8 - current) as u32;
                if let Some(price) = market.get_buy_price(resource).map(|p| p * amount_to_buy as f32) {
                    if self.resources.get(&ResourceType::Gold) >= price as u32 {
                        if let Some(_) = market.buy(resource, amount_to_buy) {
                            self.resources.subtract(&ResourceType::Gold, price as u32);
                            self.resources.add(resource, amount_to_buy);
                            println!("{}: Автопокупка {} ед. {:?} за {:.1} золота", 
                                self.name, amount_to_buy, resource, price);
                        }
                    }
                }
            }
        }
    }
}
```

### Интеграция с системой случайных событий

```rust
// В обработчике событий
fn handle_event(city: &mut City, event: &RandomEvent, event_manager: &EventManager) {
    // ... существующий код ...
    
    // Проверяем, является ли событие экономическим
    if event.category == EventCategory::Economic {
        if let Some(market) = city.world.trade_manager.get_city_market_mut(&city.name) {
            match event.severity {
                EventSeverity::Positive => {
                    // Положительное экономическое событие улучшает рынок
                    market.recover_market_health(0.1);
                    println!("Рынок города {} восстанавливается", city.name);
                },
                EventSeverity::Major | EventSeverity::Disaster => {
                    // Серьезное экономическое событие создает шок
                    let severity = match event.severity {
                        EventSeverity::Major => 0.2,
                        EventSeverity::Disaster => 0.4,
                        _ => 0.1,
                    };
                    
                    // Находим затронутые ресурсы
                    let affected_resources: Vec<ResourceType> = event.effects.iter()
                        .filter_map(|effect| effect.resource_type.clone())
                        .collect();
                    
                    if !affected_resources.is_empty() {
                        market.apply_market_shock(severity, Some(affected_resources));
                        println!("Экономический шок затронул рынок города {}", city.name);
                    } else {
                        market.apply_market_shock(severity, None);
                        println!("Общий экономический шок для рынка города {}", city.name);
                    }
                },
                _ => {}
            }
        }
    }
}
```

## Советы по использованию

1. **Баланс рыночных цен**: Настраивайте базовые цены и волатильность ресурсов для создания сбалансированной экономики, где торговля выгодна, но не слишком прибыльна.

2. **Специализация городов**: Создавайте города со специализацией на определенных ресурсах, чтобы стимулировать торговлю между ними.

3. **Динамические события**: Используйте рыночные шоки и события для создания интересных экономических ситуаций, требующих адаптации стратегии игрока.

4. **Визуализация трендов**: Предоставьте игрокам наглядную информацию о трендах цен, чтобы они могли планировать свои торговые стратегии.

5. **Торговые федерации**: Позвольте игрокам создавать торговые федерации или альянсы для получения бонусов к торговле между городами-союзниками. 