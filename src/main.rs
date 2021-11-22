use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let data = r#"
        {
            "name": "Toyota Sienna",
            "total_mileage": 1000,
            "maintenance_items": {
                "Oil Change": {
                    "months_interval": 50,
                    "miles_interval": 3,
                    "name": "Oil Change",
                    "description": "Synthetic 5w20",
                    "last_performed_maintenance": {
                        "miles_performed": 50,
                        "date_performed": 0
                    }
                },
                "Transmission Fluid Change": {
                    "months_interval": 100,
                    "miles_interval": 140,
                    "name": "Transmission Fluid Change",
                    "description": "Da Blue Stuff!",
                    "last_performed_maintenance": {
                        "miles_performed": 22,
                        "date_performed": 0
                    }
                },
                "Change all the bolts": {
                    "months_interval": 900,
                    "miles_interval": 140,
                    "name": "Change all the bolts",
                    "description": "Except spoiler bolts",
                    "last_performed_maintenance": {
                        "miles_performed": 999,
                        "date_performed": 0
                    }
                }
            }
        }"#;

    let mut sienna = serde_json::from_str(data);

    // let mut sienna = Car {
    //     name: String::from("Toyota Sienna"),
    //     total_mileage: 1000,
    //     maintenance_items: HashMap::new()
    // };

    // sienna.add_maintenance_item(
    //     3,
    //     50,
    //     "Oil Change".to_string(),
    //     "Synthetic 5w20".to_string(),
    //     Some(
    //         PerformedMaintenance {
    //             miles_performed: 949,
    //             date_performed: Instant::now()
    //         }
    //     )
    // );

    // sienna.add_maintenance_item(
    //     3,
    //     500,
    //     "Transmission fluid Change".to_string(),
    //     "Blue stuff".to_string(),
    //     Some(
    //         PerformedMaintenance {
    //             miles_performed: 600,
    //             date_performed: Instant::now()
    //         }
    //     )
    // );

    dbg!(&sienna);
    determine_needed_maintenance(sienna.unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct Car {
    name: String,
    total_mileage: i32,
    maintenance_items: HashMap<String, MaintenanceItem>
}

impl Car {
    fn add_maintenance_item(
        &mut self,
        months_interval: u16, 
        miles_interval: i32, 
        name: String, 
        description: String,
        last_performed_maintenance: Option<PerformedMaintenance>
    ) {
        self.maintenance_items.insert(
            name.to_string(), 
            build_maintenance_item(
                months_interval, 
                miles_interval, 
                name.to_string(),
                description,
                last_performed_maintenance
            )
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MaintenanceItem {
    months_interval: u16,
    miles_interval: i32,
    name: String,
    description: String,
    last_performed_maintenance: Option<PerformedMaintenance>,
}

fn build_maintenance_item(
    months_interval: u16, 
    miles_interval: i32, 
    name: String, 
    description: String,
    last_performed_maintenance: Option<PerformedMaintenance>
) -> MaintenanceItem {
    MaintenanceItem {
        months_interval,
        miles_interval,
        name,
        description,
        last_performed_maintenance
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PerformedMaintenance {
    miles_performed: i32,
    date_performed: i32
}

fn determine_needed_maintenance(car: Car) {
    println!("Car has {} miles\n", car.total_mileage);
    for (k, v) in car.maintenance_items {
        match v.last_performed_maintenance {
            None => println!("{}",k),
            Some(performed_maintenance) => {
                if (performed_maintenance.miles_performed + v.miles_interval) <= car.total_mileage {
                    let total = (performed_maintenance.miles_performed + v.miles_interval);
                    println!(
                        "* {} last performed at {}, should be performed every {} miles", 
                        k,
                        performed_maintenance.miles_performed,
                        v.miles_interval
                    );
                }
            }
        }
    }
}