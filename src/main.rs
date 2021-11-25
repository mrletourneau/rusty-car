use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;

fn main() {
    let sub_command = retrieve_argument(1, None);
    let sub_command: SubCommand = convert_argument(&sub_command);

    match sub_command {
        SubCommand::Vehicle => {
            let vehicle_command = retrieve_argument(
                2,
                Some(String::from("No vehicle sub command found, exiting...")),
            );
            let vehicle_command: VehicleSubCommand = convert_argument(&vehicle_command);

            match vehicle_command {
                VehicleSubCommand::Add => {
                    println!("Add vehicle");
                }
                VehicleSubCommand::Remove => {
                    println!("Remove vehicle");
                }
                VehicleSubCommand::List => {
                    println!("List vehicles");
                }
                VehicleSubCommand::Get => {
                    let vehicle_name = retrieve_argument(
                        3,
                        Some(String::from("No vehicle name specified, exiting...")),
                    );

                    let cars = load_cars();

                    dbg!(cars.get(&vehicle_name));
                }
                VehicleSubCommand::Update => {
                    println!("Update vehicle");
                }
            }
        }
    }

    fn retrieve_argument(argument_position: usize, error_message: Option<String>) -> String {
        match std::env::args().nth(argument_position) {
            Some(i) => i,
            None => {
                println!(
                    "{}",
                    error_message.unwrap_or(String::from("No command passed, exiting..."))
                );
                exit(1);
            }
        }
    }

    fn convert_argument<T: FromStr<Err = ()>>(argument: &String) -> T {
        match T::from_str(&argument) {
            Ok(i) => i,
            Err(()) => {
                println!("Invalid subcommand \"{}\", exiting...", argument);
                exit(1);
            }
        }
    }

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

fn load_cars() -> HashMap<String, Car> {
    let data = r#"
    {
        "sienna": {
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
        }
    }"#;

    serde_json::from_str(data).unwrap()
}

enum SubCommand {
    Vehicle,
}

impl FromStr for SubCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<SubCommand, Self::Err> {
        match input {
            "vehicle" => Ok(SubCommand::Vehicle),
            _ => Err(()),
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
            "add" => Ok(VehicleSubCommand::Add),
            "remove" => Ok(VehicleSubCommand::Remove),
            "list" => Ok(VehicleSubCommand::List),
            "get" => Ok(VehicleSubCommand::Get),
            "update" => Ok(VehicleSubCommand::Update),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Car {
    name: String,
    total_mileage: i32,
    maintenance_items: HashMap<String, MaintenanceItem>,
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
struct MaintenanceItem {
    months_interval: u16,
    miles_interval: i32,
    name: String,
    description: String,
    last_performed_maintenance: Option<PerformedMaintenance>,
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
struct PerformedMaintenance {
    miles_performed: i32,
    date_performed: i32,
}

#[allow(dead_code)]
fn determine_needed_maintenance(car: Car) {
    println!("Car has {} miles\n", car.total_mileage);
    for (k, v) in car.maintenance_items {
        match v.last_performed_maintenance {
            None => println!("{}", k),
            Some(performed_maintenance) => {
                if (performed_maintenance.miles_performed + v.miles_interval) <= car.total_mileage {
                    println!(
                        "* {} last performed at {}, should be performed every {} miles",
                        k, performed_maintenance.miles_performed, v.miles_interval
                    );
                }
            }
        }
    }
}
