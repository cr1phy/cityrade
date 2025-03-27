// In plugin.rs
use async_trait::async_trait;
use std::any::Any;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn description(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    enabled_plugins: Vec<String>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            enabled_plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub async fn initialize_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &mut self.plugins {
            plugin.initialize().await?;
        }
        Ok(())
    }

    pub async fn enable_plugin(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &mut self.plugins {
            if plugin.name() == name {
                plugin.on_enable().await?;
                self.enabled_plugins.push(name.to_string());
                return Ok(());
            }
        }
        Err(format!("Plugin not found: {}", name).into())
    }

    pub async fn disable_plugin(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &mut self.plugins {
            if plugin.name() == name {
                plugin.on_disable().await?;
                self.enabled_plugins.retain(|n| n != name);
                return Ok(());
            }
        }
        Err(format!("Plugin not found: {}", name).into())
    }

    pub fn get_plugin<T: 'static>(&self, name: &str) -> Option<&T> {
        for plugin in &self.plugins {
            if plugin.name() == name {
                return plugin.as_any().downcast_ref::<T>();
            }
        }
        None
    }

    pub fn get_plugin_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        for plugin in &mut self.plugins {
            if plugin.name() == name {
                return plugin.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }
    
    pub fn get_plugin_names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
}
