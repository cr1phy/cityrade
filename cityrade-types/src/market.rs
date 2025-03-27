use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;
use crate::resources::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketItem {
    pub resource_type: ResourceType,
    pub quantity: u32,
    pub base_price: f32,
    pub current_price: f32,
    pub stock_level: StockLevel,
    pub volatility: f32,
    pub last_price_change: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StockLevel {
    Shortage,
    Low,
    Normal,
    Abundant,
    Surplus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    items: HashMap<ResourceType, MarketItem>,
    demand_factor: HashMap<ResourceType, f32>,
    supply_factor: HashMap<ResourceType, f32>,
    market_health: f32,
    last_update: u64,
}

impl Market {
    pub fn new() -> Self {
        let mut items = HashMap::new();
        let mut demand_factor = HashMap::new();
        let mut supply_factor = HashMap::new();
        
        let resources = [
            (ResourceType::Gold, 1.0, 1000, 0.05),
            (ResourceType::Wood, 10.0, 500, 0.1),
            (ResourceType::Stone, 15.0, 300, 0.08),
            (ResourceType::Food, 5.0, 800, 0.15),
            (ResourceType::Iron, 25.0, 200, 0.12),
            (ResourceType::Crystal, 50.0, 100, 0.2),
            (ResourceType::Energy, 20.0, 250, 0.1),
        ];
        
        for (res_type, base_price, initial_quantity, volatility) in resources.iter() {
            items.insert(
                res_type.clone(),
                MarketItem {
                    resource_type: res_type.clone(),
                    quantity: *initial_quantity,
                    base_price: *base_price,
                    current_price: *base_price,
                    stock_level: StockLevel::Normal,
                    volatility: *volatility,
                    last_price_change: 0.0,
                },
            );
            
            demand_factor.insert(res_type.clone(), 1.0);
            supply_factor.insert(res_type.clone(), 1.0);
        }

        Market {
            items,
            demand_factor,
            supply_factor,
            market_health: 1.0,
            last_update: 0,
        }
    }
    
    pub fn update_prices(&mut self, turn: u64) {
        let mut rng = rand::rng();
        
        if self.last_update >= turn {
            return;
        }
        
        for (resource_type, item) in self.items.iter_mut() {
            let demand = self.demand_factor.get(resource_type).cloned().unwrap_or(1.0);
            let supply = self.supply_factor.get(resource_type).cloned().unwrap_or(1.0);
            
            let supply_demand_ratio = demand / supply;
            
            let random_factor = 1.0 + (rng.random::<f32>() - 0.5) * item.volatility;
            
            let old_price = item.current_price;
            let price_factor = supply_demand_ratio * random_factor * self.market_health;
            let new_price = item.base_price * price_factor;
            
            let max_change = old_price * 0.3;
            let price_change = (new_price - old_price).min(max_change).max(-max_change);
            
            item.current_price = (old_price + price_change).max(item.base_price * 0.5).min(item.base_price * 3.0);
            item.last_price_change = price_change;
            
            item.stock_level = match item.quantity {
                q if q < 100 => StockLevel::Shortage,
                q if q < 300 => StockLevel::Low,
                q if q < 700 => StockLevel::Normal,
                q if q < 1000 => StockLevel::Abundant,
                _ => StockLevel::Surplus,
            };
            
            item.current_price *= match item.stock_level {
                StockLevel::Shortage => 1.5,
                StockLevel::Low => 1.2,
                StockLevel::Normal => 1.0,
                StockLevel::Abundant => 0.8,
                StockLevel::Surplus => 0.6,
            };
            
            let demand_change = 1.0 + (rng.random::<f32>() - 0.5) * 0.1;
            let supply_change = 1.0 + (rng.random::<f32>() - 0.5) * 0.1;
            
            *self.demand_factor.entry(resource_type.clone()).or_insert(1.0) *= demand_change;
            *self.supply_factor.entry(resource_type.clone()).or_insert(1.0) *= supply_change;
            
            *self.demand_factor.get_mut(resource_type).unwrap() = self.demand_factor[resource_type].max(0.5).min(2.0);
            *self.supply_factor.get_mut(resource_type).unwrap() = self.supply_factor[resource_type].max(0.5).min(2.0);
        }
        
        self.last_update = turn;
    }

    pub fn buy(&mut self, resource_type: &ResourceType, amount: u32) -> Option<f32> {
        let item = self.items.get_mut(resource_type)?;
        
        if item.quantity < amount {
            return None; // Недостаточно ресурсов на рынке
        }
        
        let total_price = item.current_price * amount as f32;
        
        // Уменьшаем количество и обновляем факторы спроса и предложения
        item.quantity -= amount;
        
        // Повышаем спрос при покупке
        *self.demand_factor.entry(resource_type.clone()).or_insert(1.0) += 0.05;
        
        // Обновляем уровень запасов
        item.stock_level = match item.quantity {
            q if q < 100 => StockLevel::Shortage,
            q if q < 300 => StockLevel::Low,
            q if q < 700 => StockLevel::Normal,
            q if q < 1000 => StockLevel::Abundant,
            _ => StockLevel::Surplus,
        };
        
        Some(total_price)
    }

    pub fn sell(&mut self, resource_type: &ResourceType, amount: u32) -> Option<f32> {
        let item = self.items.get_mut(resource_type)?;
        
        // Продажная цена немного ниже текущей рыночной
        let sell_price = item.current_price * 0.85;
        let total_price = sell_price * amount as f32;
        
        // Увеличиваем количество и обновляем факторы спроса и предложения
        item.quantity += amount;
        
        // Повышаем предложение при продаже
        *self.supply_factor.entry(resource_type.clone()).or_insert(1.0) += 0.05;
        
        // Обновляем уровень запасов
        item.stock_level = match item.quantity {
            q if q < 100 => StockLevel::Shortage,
            q if q < 300 => StockLevel::Low,
            q if q < 700 => StockLevel::Normal,
            q if q < 1000 => StockLevel::Abundant,
            _ => StockLevel::Surplus,
        };
        
        Some(total_price)
    }

    pub fn get_market_item(&self, resource_type: &ResourceType) -> Option<&MarketItem> {
        self.items.get(resource_type)
    }

    pub fn get_buy_price(&self, resource_type: &ResourceType) -> Option<f32> {
        self.items.get(resource_type).map(|item| item.current_price)
    }

    pub fn get_sell_price(&self, resource_type: &ResourceType) -> Option<f32> {
        self.items.get(resource_type).map(|item| item.current_price * 0.85)
    }

    pub fn has_resource(&self, resource_type: &ResourceType, amount: u32) -> bool {
        if let Some(item) = self.items.get(resource_type) {
            item.quantity >= amount
        } else {
            false
        }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, base_price: f32, initial_quantity: u32, volatility: f32) {
        let resource_type_clone = resource_type.clone();
        
        self.items.insert(
            resource_type_clone.clone(),
            MarketItem {
                resource_type,
                quantity: initial_quantity,
                base_price,
                current_price: base_price,
                stock_level: StockLevel::Normal,
                volatility,
                last_price_change: 0.0,
            },
        );
        
        self.demand_factor.insert(resource_type_clone.clone(), 1.0);
        self.supply_factor.insert(resource_type_clone, 1.0);
    }

    pub fn apply_market_shock(&mut self, severity: f32, affected_resources: Option<Vec<ResourceType>>) {
        let shock_factor = 1.0 + severity;
        
        match affected_resources {
            Some(resources) => {
                // Шок влияет только на указанные ресурсы
                for resource_type in resources {
                    if let Some(item) = self.items.get_mut(&resource_type) {
                        item.current_price *= shock_factor;
                        item.volatility *= 1.2; // Увеличиваем волатильность
                    }
                }
            },
            None => {
                // Шок влияет на весь рынок
                for item in self.items.values_mut() {
                    item.current_price *= shock_factor;
                    item.volatility *= 1.1;
                }
                
                // Уменьшаем здоровье рынка
                self.market_health = (self.market_health - severity * 0.2).max(0.5);
            }
        }
    }

    pub fn recover_market_health(&mut self, amount: f32) {
        self.market_health = (self.market_health + amount).min(1.0);
    }

    pub fn get_price_trend(&self, resource_type: &ResourceType) -> Option<PriceTrend> {
        let item = self.items.get(resource_type)?;
        
        let change_percent = (item.last_price_change / item.base_price) * 100.0;
        
        match change_percent {
            x if x > 10.0 => Some(PriceTrend::StrongRise),
            x if x > 3.0 => Some(PriceTrend::Rise),
            x if x < -10.0 => Some(PriceTrend::StrongFall),
            x if x < -3.0 => Some(PriceTrend::Fall),
            _ => Some(PriceTrend::Stable),
        }
    }
}

// Тренды цен для отображения в интерфейсе
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceTrend {
    StrongRise,  // Сильный рост
    Rise,        // Рост
    Stable,      // Стабильность
    Fall,        // Падение
    StrongFall,  // Сильное падение
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub source_city: String,
    pub target_city: String,
    pub resource_type: ResourceType,
    pub quantity: u32,
    pub price_per_unit: f32,
    pub duration: u32, // в ходах
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeManager {
    pub trade_routes: Vec<TradeRoute>,
    pub city_markets: HashMap<String, Market>,
}

impl TradeManager {
    pub fn new() -> Self {
        TradeManager {
            trade_routes: Vec::new(),
            city_markets: HashMap::new(),
        }
    }
    
    // Создание рынка для города
    pub fn create_city_market(&mut self, city_name: &str) -> &mut Market {
        self.city_markets.entry(city_name.to_string()).or_insert_with(Market::new)
    }
    
    // Добавление торгового маршрута
    pub fn add_trade_route(&mut self, route: TradeRoute) {
        self.trade_routes.push(route);
    }
    
    // Получение рынка города
    pub fn get_city_market(&self, city_name: &str) -> Option<&Market> {
        self.city_markets.get(city_name)
    }
    
    // Получение рынка города (изменяемый)
    pub fn get_city_market_mut(&mut self, city_name: &str) -> Option<&mut Market> {
        self.city_markets.get_mut(city_name)
    }
    
    // Обновление всех рынков городов
    pub fn update_all_markets(&mut self, turn: u64) {
        for market in self.city_markets.values_mut() {
            market.update_prices(turn);
        }
        
        // Медленное восстановление здоровья рынков
        for market in self.city_markets.values_mut() {
            market.recover_market_health(0.01);
        }
    }
}

impl Default for TradeManager {
    fn default() -> Self {
        Self::new()
    }
}
