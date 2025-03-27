use cityrade_types::resources::ResourceType;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields, parse_quote, Path};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ResourceConfig {
    density: f32,
    min_depth: u32,
    clusters: bool,
    biomes: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct StructureConfig {
    count: String,
    min_distance: u32,
}

/// Макрос для создания плагинов игрового сервера
/// 
/// Этот макрос упрощает создание плагинов, автоматически реализуя трейт Plugin.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::Plugin;
/// 
/// #[derive(Plugin)]
/// #[plugin(
///     name = "MyPlugin",
///     version = "1.0.0",
///     description = "Мой первый плагин",
///     author = "Разработчик",
///     license = "MIT"
/// )]
/// struct MyPlugin {
///     // Поля плагина
/// }
/// 
/// impl MyPlugin {
///     // Собственные методы плагина
///     fn new() -> Self {
///         Self {}
///     }
///     
///     async fn on_initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
///         // Инициализация плагина
///         Ok(())
///     }
///     
///     async fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
///         // Активация плагина
///         Ok(())
///     }
///     
///     async fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
///         // Деактивация плагина
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_derive(Plugin, attributes(plugin))]
pub fn derive_plugin(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Получаем атрибуты плагина
    let mut plugin_name = String::new();
    let mut plugin_version = String::new();
    let mut plugin_description = String::new();
    let mut plugin_author = String::new();
    let mut plugin_license = String::new();
    
    for attr in &input.attrs {
        if attr.meta.path().is_ident("plugin") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                for nested_meta in meta_list.tokens.clone().into_iter() {
                    // Упрощенная версия для демонстрации
                    // Здесь нужно использовать syn::parse2() для парсинга токенов
                }
            }
        }
    }
    
    // Если атрибуты не указаны, используем имя структуры
    if plugin_name.is_empty() {
        plugin_name = name.to_string();
    }
    
    // Генерируем код
    let expanded = quote! {
        #[async_trait::async_trait]
        impl cityrade_types::plugin::Plugin for #name {
            fn name(&self) -> String {
                #plugin_name.to_string()
            }
            
            fn version(&self) -> String {
                #plugin_version.to_string()
            }
            
            fn description(&self) -> String {
                #plugin_description.to_string()
            }
            
            fn author(&self) -> String {
                #plugin_author.to_string()
            }
            
            fn license(&self) -> String {
                #plugin_license.to_string()
            }
            
            async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            async fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            async fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для регистрации обработчиков событий в плагине
/// 
/// Этот макрос позволяет легко добавлять обработчики различных событий игрового сервера
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::{Plugin, handler};
/// 
/// #[derive(Plugin)]
/// #[plugin(name = "EventPlugin")]
/// struct EventPlugin;
/// 
/// #[handler]
/// impl EventPlugin {
///     // Обработчик события тика
///     async fn on_tick(&self, tick_count: u64) {
///         println!("Тик: {}", tick_count);
///     }
///     
///     // Обработчик события подключения игрока
///     async fn on_player_join(&self, player_id: &str, world_id: &str) {
///         println!("Игрок {} подключился к миру {}", player_id, world_id);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let self_ty = &input.self_ty;
    
    let mut event_handlers = Vec::new();
    
    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();
            
            if method_name_str.starts_with("on_") {
                let event_name = method_name_str.strip_prefix("on_").unwrap();
                let handler = quote! {
                    fn #method_name(&self) {
                        println!("Handling event: {}", #event_name);
                    }
                };
                event_handlers.push(handler);
            }
        }
    }
    
    let expanded = quote! {
        #input
        
        impl #self_ty {
            pub fn register_handlers(&self, event_system: &mut cityrade_types::events::EventSystem) {
                // Регистрация обработчиков
                #(#event_handlers)*
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания нового плагина
/// 
/// Этот макрос создает новый плагин с базовой структурой и необходимыми реализациями.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::create_plugin;
/// 
/// create_plugin! {
///     name: "MyNewPlugin",
///     version: "1.0.0",
///     description: "Мой новый плагин",
///     author: "Разработчик",
///     license: "MIT",
///     
///     // Дополнительные поля плагина
///     fields: {
///         counter: u32,
///         is_enabled: bool,
///     },
///     
///     // Инициализация плагина
///     initialize: |plugin| {
///         plugin.counter = 0;
///         plugin.is_enabled = false;
///         Ok(())
///     },
///     
///     // Активация плагина
///     enable: |plugin| {
///         plugin.is_enabled = true;
///         println!("Плагин активирован!");
///         Ok(())
///     },
///     
///     // Деактивация плагина
///     disable: |plugin| {
///         plugin.is_enabled = false;
///         println!("Плагин деактивирован!");
///         Ok(())
///     }
/// }
/// ```
#[proc_macro]
pub fn create_plugin(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    // Очень простая реализация для примера
    let expanded = quote! {
        struct GeneratedPlugin {
            name: String,
            version: String,
        }
        
        impl GeneratedPlugin {
            fn new() -> Self {
                Self {
                    name: "Generated Plugin".to_string(),
                    version: "1.0.0".to_string(),
                }
            }
        }
        
        #[async_trait::async_trait]
        impl cityrade_types::plugin::Plugin for GeneratedPlugin {
            fn name(&self) -> String {
                self.name.clone()
            }
            
            fn version(&self) -> String {
                self.version.clone()
            }
            
            fn description(&self) -> String {
                "Generated plugin".to_string()
            }
            
            fn author(&self) -> String {
                "System".to_string()
            }
            
            fn license(&self) -> String {
                "MIT".to_string()
            }
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            
            async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            async fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            async fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
        }
        
        GeneratedPlugin::new()
    };
    
    TokenStream::from(expanded)
}

/// Макрос для определения нового типа здания
/// 
/// Этот макрос позволяет декларативно определить новый тип здания
/// с необходимыми характеристиками и поведением.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::building_type;
/// 
/// building_type! {
///     name: "Лаборатория алхимии",
///     id: "alchemy_lab",
///     description: "Позволяет создавать различные зелья и эликсиры",
///     base_cost: {
///         Wood: 80,
///         Stone: 120,
///         Gold: 150,
///         Crystal: 30
///     },
///     production: {
///         level_1: { Gold: -10, Energy: -5, Research: 10 },
///         level_2: { Gold: -15, Energy: -8, Research: 18 },
///         level_3: { Gold: -25, Energy: -12, Research: 30 }
///     }
/// }
/// ```
#[proc_macro]
pub fn building_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprStruct);
    
    let mut name = String::new();
    let mut id = String::new();
    let mut description = String::new();
    
    for field in &input.fields {
        if let syn::Member::Named(ident) = &field.member {
            let field_name = ident.to_string();
            
            match field_name.as_str() {
                "name" => {
                    if let syn::Expr::Lit(expr_lit) = &field.expr {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            name = lit_str.value();
                        }
                    }
                },
                "id" => {
                    if let syn::Expr::Lit(expr_lit) = &field.expr {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            id = lit_str.value();
                        }
                    }
                },
                "description" => {
                    if let syn::Expr::Lit(expr_lit) = &field.expr {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            description = lit_str.value();
                        }
                    }
                },
                _ => {}
            }
        }
    }
    
    // Создаем идентификатор для структуры
    let struct_ident = format_ident!("{}", id);
    
    // Генерируем код для нового типа здания
    let expanded = quote! {
        pub struct #struct_ident {
            pub name: String,
            pub id: String,
            pub description: String,
        }
        
        impl #struct_ident {
            pub fn new() -> Self {
                Self {
                    name: #name.to_string(),
                    id: #id.to_string(),
                    description: #description.to_string(),
                }
            }
        }
        
        impl cityrade_types::building::BuildingType for #struct_ident {
            fn display_name(&self) -> &str {
                &self.name
            }
            
            fn description(&self) -> &str {
                &self.description
            }
            
            fn base_cost(&self) -> Vec<(cityrade_types::resources::ResourceType, u32)> {
                vec![]
            }
            
            fn production_effect(&self, level: u32) -> Vec<(cityrade_types::resources::ResourceType, i32)> {
                vec![]
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания нового здания в игровом мире
/// 
/// Этот макрос позволяет декларативно создать новое здание с указанными параметрами.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::building;
/// 
/// let market = building! {
///     name: "Городской рынок",
///     type: BuildingType::Market,
///     level: 2,
///     position: (10, 15)
/// };
/// ```
#[proc_macro]
pub fn building(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprStruct);
    
    // Извлекаем поля из макроса
    let mut name = String::new();
    let mut building_type = String::new();
    let mut level = 1u32;
    let mut position_x = 0i32;
    let mut position_y = 0i32;
    
    for field in &input.fields {
        if let syn::Member::Named(ident) = &field.member {
            let field_name = ident.to_string();
            
            match field_name.as_str() {
                "name" => {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &field.expr {
                        name = s.value();
                    }
                },
                "type" => {
                    // Здесь нужна логика для извлечения типа здания
                    // ...
                },
                "level" => {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i), .. }) = &field.expr {
                        if let Ok(l) = i.base10_parse::<u32>() {
                            level = l;
                        }
                    }
                },
                "position" => {
                    // Здесь нужна логика для извлечения позиции
                    // ...
                },
                _ => {}
            }
        }
    }
    
    // Генерируем код для создания нового здания
    let expanded = quote! {
        {
            let mut building = cityrade_types::building::Building::new(
                uuid::Uuid::new_v4().to_string(),
                #name.to_string(),
                #building_type,
                (#position_x, #position_y),
            );
            
            // Устанавливаем уровень, если он не 1
            if #level > 1 {
                for _ in 1..#level {
                    building.upgrade();
                }
            }
            
            building
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для определения параметров генерации мира
/// 
/// Этот макрос позволяет декларативно определить параметры для генерации игрового мира.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::world_generator;
/// 
/// let generator = world_generator! {
///     seed: 12345,
///     size: (2000, 2000),
///     biomes: {
///         water: 0.3,
///         plains: 0.4,
///         mountains: 0.2,
///         forest: 0.1
///     },
///     resources: {
///         iron: { density: 0.05, min_depth: 10, clusters: true },
///         gold: { density: 0.02, min_depth: 20, clusters: true },
///         wood: { density: 0.1, biomes: ["forest"], clusters: false }
///     },
///     structures: {
///         cities: { count: 5..10, min_distance: 200 },
///         villages: { count: 15..25, min_distance: 100 }
///     }
/// };
/// ```
#[proc_macro]
pub fn world_generator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprStruct);
    
    let mut seed = quote!(None);
    let mut size = quote!((1000, 1000));
    
    for field in &input.fields {
        if let syn::Member::Named(ident) = &field.member {
            let field_name = ident.to_string();
            
            match field_name.as_str() {
                "seed" => {
                    if let syn::Expr::Lit(expr_lit) = &field.expr {
                        if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                            let seed_value = lit_int.base10_parse::<u64>().unwrap_or(0);
                            seed = quote!(Some(#seed_value));
                        }
                    }
                },
                "size" => {
                    size = quote!(#field.expr);
                },
                _ => {}
            }
        }
    }
    
    let expanded = quote! {
        {
            let mut generator = cityrade_types::world::WorldGenerator::new(#seed);
            let (width, height) = #size;
            let world = generator.generate(width, height);
            world
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания типа здания через derive
/// 
/// Этот макрос упрощает создание типов зданий через атрибуты.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::BuildingType;
/// 
/// #[derive(BuildingType)]
/// #[building(
///     name = "Кузница",
///     id = "blacksmith",
///     description = "Производит железные изделия и оружие"
/// )]
/// #[cost(
///     Wood = 80,
///     Stone = 50,
///     Iron = 30,
///     Gold = 100
/// )]
/// #[production(
///     level_1(Iron = 5, Gold = -10),
///     level_2(Iron = 10, Gold = -15),
///     level_3(Iron = 18, Gold = -25, Weapons = 2)
/// )]
/// struct Blacksmith;
/// ```
#[proc_macro_derive(BuildingType, attributes(building, cost, production))]
pub fn derive_building_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Извлекаем атрибуты здания
    let mut building_name = String::new();
    let mut building_id = String::new();
    let mut building_description = String::new();
    let mut base_costs: Vec<(ResourceType, u32)> = Vec::new();
    let mut production_effects: HashMap<u32, Vec<(ResourceType, i32)>> = HashMap::new();
    
    for attr in &input.attrs {
        if attr.path().is_ident("building") {
            // Извлекаем основные характеристики здания
            // ...
        } else if attr.path().is_ident("cost") {
            // Извлекаем стоимость строительства
            // ...
        } else if attr.path().is_ident("production") {
            // Извлекаем эффекты производства
            // ...
        }
    }
    
    // Если атрибуты не указаны, используем имя структуры
    if building_name.is_empty() {
        building_name = name.to_string();
    }
    if building_id.is_empty() {
        building_id = name.to_string().to_lowercase();
    }
    if building_description.is_empty() {
        building_description = format!("Здание {}", name);
    }
    
    // Генерируем имплементацию трейта BuildingType
    let expanded = quote! {
        impl cityrade_types::building::BuildingType for #name {
            fn display_name(&self) -> &str {
                &#building_name
            }
            
            fn description(&self) -> &str {
                &#building_description
            }
            
            fn base_cost(&self) -> Vec<(cityrade_types::resources::ResourceType, u32)> {
                vec![
                    // Здесь будут ресурсы, указанные в cost
                    // ...
                ]
            }
            
            fn production_effect(&self, level: u32) -> Vec<(cityrade_types::resources::ResourceType, i32)> {
                match level {
                    // Здесь будут эффекты в зависимости от уровня
                    // ...
                    _ => vec![]
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания генератора мира через derive
/// 
/// Этот макрос упрощает создание генераторов мира через атрибуты.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::WorldGenerator;
/// 
/// #[derive(WorldGenerator)]
/// #[generator(
///     name = "ContinentalGenerator",
///     description = "Генератор мира с континентами и островами"
/// )]
/// #[biomes(
///     water = 0.3,
///     plains = 0.35,
///     mountains = 0.2,
///     forest = 0.15
/// )]
/// #[resources(
///     iron(density = 0.05, min_depth = 10, clusters = true),
///     gold(density = 0.02, min_depth = 20, clusters = true),
///     wood(density = 0.1, biomes = ["forest"], clusters = false)
/// )]
/// #[structures(
///     cities(count = "5..10", min_distance = 200),
///     villages(count = "15..25", min_distance = 100)
/// )]
/// struct ContinentalGenerator {
///     // Дополнительные поля генератора
/// }
/// ```
#[proc_macro_derive(WorldGenerator, attributes(generator, biomes, resources, structures))]
pub fn derive_world_generator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Извлекаем атрибуты генератора
    let mut generator_name = String::new();
    let mut generator_description = String::new();
    let mut biomes: HashMap<String, f32> = HashMap::new();
    let mut resources: HashMap<String, ResourceConfig> = HashMap::new();
    let mut structures: HashMap<String, StructureConfig> = HashMap::new();
    
    for attr in &input.attrs {
        if attr.path().is_ident("generator") {
            // Извлекаем основные характеристики генератора
            // ...
        } else if attr.path().is_ident("biomes") {
            // Извлекаем биомы
            // ...
        } else if attr.path().is_ident("resources") {
            // Извлекаем ресурсы
            // ...
        } else if attr.path().is_ident("structures") {
            // Извлекаем структуры
            // ...
        }
    }
    
    // Если атрибуты не указаны, используем имя структуры
    if generator_name.is_empty() {
        generator_name = name.to_string();
    }
    if generator_description.is_empty() {
        generator_description = format!("Генератор мира {}", name);
    }
    
    // Генерируем трейты и имплементации для генератора мира
    let expanded = quote! {
        impl #name {
            pub fn new(seed: Option<u64>) -> Self {
                Self {
                    // Инициализация полей генератора
                    // ...
                }
            }
            
            pub fn generate(&mut self, width: u64, height: u64) -> cityrade_types::world::WorldMap {
                let mut world = cityrade_types::world::WorldMap::new(width, height);
                
                // Логика генерации мира на основе атрибутов
                // ...
                
                world
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для определения обработчиков событий
/// 
/// Этот макрос позволяет декларативно определить обработчики различных игровых событий.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::event_handler;
/// 
/// event_handler! {
///     on_player_join(player_id: &str, world_id: &str) {
///         println!("Игрок {} присоединился к миру {}", player_id, world_id);
///         // Ваш код обработки события
///     },
///     
///     on_player_leave(player_id: &str) {
///         println!("Игрок {} покинул игру", player_id);
///         // Ваш код обработки события
///     },
///     
///     on_building_constructed(building_id: &str, position: (i32, i32)) {
///         println!("Построено здание {} на позиции {:?}", building_id, position);
///         // Ваш код обработки события
///     }
/// }
/// ```
#[proc_macro]
pub fn event_handler(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    // Простая демонстрационная реализация
    let expanded = quote! {
        struct EventHandlers {
            registered: bool,
        }
        
        impl EventHandlers {
            fn new() -> Self {
                Self {
                    registered: false,
                }
            }
            
            fn register(&mut self, event_system: &mut cityrade_types::events::EventSystem) {
                if self.registered {
                    return;
                }
                
                // Регистрируем обработчики событий
                event_system.register_handler::<cityrade_types::events::PlayerJoinEvent>(
                    "player_join_handler".to_string(),
                    cityrade_types::events::EventPriority::Normal,
                    false,
                    |event| {
                        println!("Игрок {} присоединился к миру {}", event.player_id, event.world_id);
                        cityrade_types::events::EventResult::Continue
                    }
                );
                
                self.registered = true;
            }
        }
        
        EventHandlers::new()
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания игровых команд
/// 
/// Этот макрос позволяет декларативно определить игровые команды с параметрами и обработчиками.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::command;
/// 
/// command! {
///     name: "teleport",
///     aliases: ["tp", "warp"],
///     permission: "admin.teleport",
///     usage: "/teleport <player> <x> <y>",
///     description: "Телепортирует игрока на указанную позицию",
///     min_args: 3,
///     handler: |sender, args| {
///         let player = args[0];
///         let x = args[1].parse::<i32>().unwrap_or(0);
///         let y = args[2].parse::<i32>().unwrap_or(0);
///         
///         println!("Телепортация игрока {} на позицию ({}, {})", player, x, y);
///         // Ваш код телепортации
///         
///         true // команда выполнена успешно
///     }
/// }
/// ```
#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprStruct);
    
    // Извлекаем поля из макроса
    let mut name = String::new();
    let mut aliases: Vec<String> = Vec::new();
    let mut permission = "default.permission".to_string();
    let mut usage = "".to_string();
    let mut description = "".to_string();
    let mut min_args = 0;
    let mut handler = None;
    
    for field in &input.fields {
        if let syn::Member::Named(ident) = &field.member {
            let field_name = ident.to_string();
            
            match field_name.as_str() {
                "name" => {
                    if let syn::Expr::Lit(expr_lit) = &field.expr {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            name = lit_str.value();
                        }
                    }
                },
                "aliases" => {
                    if let syn::Expr::Array(array) = &field.expr {
                        for elem in &array.elems {
                            if let syn::Expr::Lit(expr_lit) = elem {
                                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                    aliases.push(lit_str.value());
                                }
                            }
                        }
                    }
                },
                "handler" => {
                    handler = Some(&field.expr);
                },
                _ => {}
            }
        }
    }
    
    // Проверяем, что все необходимые поля указаны
    if name.is_empty() {
        return TokenStream::from(quote! {
            compile_error!("Command must have a name");
        });
    }
    
    if handler.is_none() {
        return TokenStream::from(quote! {
            compile_error!("Command must have a handler");
        });
    }
    
    let handler_expr = handler.unwrap();
    let expanded = quote! {
        {
            struct CommandHandler;
            
            impl cityrade_types::commands::Command for CommandHandler {
                fn name(&self) -> &str {
                    #name
                }
                
                fn aliases(&self) -> &[String] {
                    &[#(#aliases.to_string()),*]
                }
                
                fn permission(&self) -> &str {
                    #permission
                }
                
                fn usage(&self) -> &str {
                    #usage
                }
                
                fn description(&self) -> &str {
                    #description
                }
                
                fn min_args(&self) -> u32 {
                    #min_args
                }
                
                fn execute(&self, sender: &dyn cityrade_types::commands::CommandSender, args: Vec<String>) -> bool {
                    let handler = #handler_expr;
                    handler(sender, args)
                }
            }
            
            CommandHandler
        }
    };
    
    TokenStream::from(expanded)
}

/// Макрос для создания конфигурации плагина
/// 
/// Этот макрос позволяет декларативно определить конфигурацию плагина с значениями по умолчанию.
/// 
/// # Пример использования
/// 
/// ```
/// use cityrade_macros::config;
/// 
/// config! {
///     struct MyPluginConfig {
///         max_players: u32 = 100,
///         spawn_point: (f32, f32) = (0.0, 0.0),
///         welcome_message: String = "Добро пожаловать!",
///         pvp_enabled: bool = true,
///         allowed_commands: Vec<String> = vec!["help", "spawn", "stats"]
///     }
/// }
/// ```
#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &input.ident;
    
    let mut fields = Vec::new();
    let mut default_values = Vec::new();
    
    if let syn::Fields::Named(named_fields) = &input.fields {
        for field in &named_fields.named {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            
            fields.push(quote! {
                pub #field_name: #field_type
            });
            
            default_values.push(quote! {
                #field_name: Default::default()
            });
        }
    }
    
    // Генерируем код для создания структуры конфигурации
    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub struct #struct_name {
            #(#fields),*
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#default_values),*
                }
            }
        }
        
        impl #struct_name {
            pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
                if let Ok(file) = std::fs::File::open(path) {
                    Ok(serde_json::from_reader(file)?)
                } else {
                    let config = Self::default();
                    config.save(path)?;
                    Ok(config)
                }
            }
            
            pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
                let file = std::fs::File::create(path)?;
                serde_json::to_writer_pretty(file, self)?;
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}
