use std::fmt::Debug;

/// Трейт для отправителя команды
pub trait CommandSender: Debug {
    /// Получить имя отправителя
    fn name(&self) -> &str;
    
    /// Проверить, имеет ли отправитель указанное разрешение
    fn has_permission(&self, permission: &str) -> bool;
    
    /// Отправить сообщение отправителю
    fn send_message(&self, message: &str);
    
    /// Проверить, является ли отправитель игроком
    fn is_player(&self) -> bool;
    
    /// Проверить, является ли отправитель консолью
    fn is_console(&self) -> bool {
        !self.is_player()
    }
}

/// Трейт для игровой команды
pub trait Command: Send + Sync {
    /// Получить имя команды
    fn name(&self) -> &str;
    
    /// Получить список псевдонимов команды
    fn aliases(&self) -> &[String];
    
    /// Получить разрешение, необходимое для выполнения команды
    fn permission(&self) -> &str;
    
    /// Получить описание использования команды
    fn usage(&self) -> &str;
    
    /// Получить описание команды
    fn description(&self) -> &str;
    
    /// Получить минимальное количество аргументов для команды
    fn min_args(&self) -> u32;
    
    /// Выполнить команду
    fn execute(&self, sender: &dyn CommandSender, args: Vec<String>) -> bool;
    
    /// Проверить, может ли отправитель выполнить команду
    fn can_execute(&self, sender: &dyn CommandSender) -> bool {
        sender.has_permission(self.permission())
    }
}

/// Менеджер команд
pub struct CommandManager {
    commands: Vec<Box<dyn Command>>,
}

impl CommandManager {
    /// Создать новый менеджер команд
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    
    /// Зарегистрировать команду
    pub fn register_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    
    /// Получить команду по имени или псевдониму
    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        for command in &self.commands {
            if command.name() == name || command.aliases().contains(&name.to_string()) {
                return Some(command);
            }
        }
        None
    }
    
    /// Выполнить команду
    pub fn execute_command(&self, sender: &dyn CommandSender, command_line: &str) -> bool {
        let parts: Vec<&str> = command_line.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        
        let command_name = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        if let Some(command) = self.get_command(command_name) {
            if !command.can_execute(sender) {
                sender.send_message("У вас нет разрешения на выполнение этой команды");
                return false;
            }
            
            if args.len() < command.min_args() as usize {
                sender.send_message(&format!("Недостаточно аргументов. Использование: {}", command.usage()));
                return false;
            }
            
            return command.execute(sender, args);
        } else {
            sender.send_message(&format!("Команда '{}' не найдена", command_name));
            return false;
        }
    }
    
    /// Получить список всех команд
    pub fn get_all_commands(&self) -> Vec<&Box<dyn Command>> {
        self.commands.iter().collect()
    }
    
    /// Получить список команд, доступных отправителю
    pub fn get_available_commands(&self, sender: &dyn CommandSender) -> Vec<&Box<dyn Command>> {
        self.commands.iter()
            .filter(|cmd| cmd.can_execute(sender))
            .collect()
    }
} 