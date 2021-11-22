use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let sub_command = std::env::args().nth(1).expect("No sub command given");
    let sub_sub_command = std::env::args().nth(2).expect("No sub sub command given");

    match SubCommand::from_str(&sub_command) {
        Ok(SubCommand::Vehicle) => {
            match VehicleSubCommand::from_str(&sub_sub_command) {
                Ok(VehicleSubCommand::Add) => { println!("Add vehicle"); }
                Ok(VehicleSubCommand::Remove) => { println!("Remove vehicle"); }
                Ok(VehicleSubCommand::List) => { println!("List vehicles"); }
                Ok(VehicleSubCommand::Get) => { println!("Get vehicle"); }
                Ok(VehicleSubCommand::Update) => { println!("Update vehicle"); }
                Err(()) => { println!("Invalid vehicle subcommand \"{}\"", sub_sub_command)}
            }
        }
        Err(()) => { println!("Invalid subcommand \"{}\"", sub_command)}
    }
    
    

    // let data = r#"
    //     {
    //         "name": "Toyota Sienna",
    //         "total_mileage": 1000,
    //         "maintenance_items": {
    //             "Oil Change": {
    //                 "months_interval": 50,
    //                 "miles_interval": 3,
    //                 "name": "Oil Change",
    //                 "description": "Synthetic 5w20",
    //                 "last_performed_maintenance": {
    //                     "miles_performed": 50,
    //                     "date_performed": 0
    //                 }
    //             },
    //             "Transmission Fluid Change": {
    //                 "months_interval": 100,
    //                 "miles_interval": 140,
    //                 "name": "Transmission Fluid Change",
    //                 "description": "Da Blue Stuff!",
    //                 "last_performed_maintenance": {
    //                     "miles_performed": 22,
    //                     "date_performed": 0
    //                 }
    //             },
    //             "Change all the bolts": {
    //                 "months_interval": 900,
    //                 "miles_interval": 140,
    //                 "name": "Change all the bolts",
    //                 "description": "Except spoiler bolts",
    //                 "last_performed_maintenance": {
    //                     "miles_performed": 999,
    //                     "date_performed": 0
    //                 }
    //             }
    //         }
    //     }"#;

    // let mut sienna = serde_json::from_str(data);

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

    // dbg!(&sienna);
    // determine_needed_maintenance(sienna.unwrap())
}

enum SubCommand {
    Vehicle
}

impl FromStr for SubCommand {

    type Err = ();

    fn from_str(input: &str) -> Result<SubCommand, Self::Err> {
        match input {
            "vehicle"  => Ok(SubCommand::Vehicle),
            _      => Err(()),
        }
    }
}

enum VehicleSubCommand {
    Add,
    Remove,
    List,
    Get,
    Update,
}

impl FromStr for VehicleSubCommand {

    type Err = ();

    fn from_str(input: &str) -> Result<VehicleSubCommand, Self::Err> {
        match input {
            "add"  => Ok(VehicleSubCommand::Add),
            "remove"  => Ok(VehicleSubCommand::Remove),
            "list"  => Ok(VehicleSubCommand::List),
            "get"  => Ok(VehicleSubCommand::Get),
            "update"  => Ok(VehicleSubCommand::Update),
            _      => Err(()),
        }
    }
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