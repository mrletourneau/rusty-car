use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub name: String,
    pub total_mileage: i32,
    pub maintenance_items: HashMap<String, MaintenanceItem>,
}

impl Car {
    #[allow(dead_code)]
    fn add_maintenance_item(
        &mut self,
        months_interval: u16,
        miles_interval: i32,
        name: String,
        description: String,
        last_performed_maintenance: Option<PerformedMaintenance>,
    ) {
        self.maintenance_items.insert(
            name.to_string(),
            build_maintenance_item(
                months_interval,
                miles_interval,
                name.to_string(),
                description,
                last_performed_maintenance,
            ),
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceItem {
    pub months_interval: u16,
    pub miles_interval: i32,
    pub name: String,
    pub description: String,
    pub last_performed_maintenance: Option<PerformedMaintenance>,
}

#[allow(dead_code)]
fn build_maintenance_item(
    months_interval: u16,
    miles_interval: i32,
    name: String,
    description: String,
    last_performed_maintenance: Option<PerformedMaintenance>,
) -> MaintenanceItem {
    MaintenanceItem {
        months_interval,
        miles_interval,
        name,
        description,
        last_performed_maintenance,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformedMaintenance {
    pub miles_performed: i32,
    pub date_performed: i32,
}