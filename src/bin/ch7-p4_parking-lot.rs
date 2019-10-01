// Parking Lot: Design a parking lot using object-oriented principles.

// - The parking lot has multiple levels.
// - Each level has multiple rows of spots.
// - The parking lot can park motorcycles, cars, and buses.
// - The parking lot has motorcycle spots, compact spots, and large spots.
// - A motorcycle can park in any spot.
// - A car can park in either a single compact spot or a single large spot.
// - A bus can park in five large spots that are consecutive and within the same row. It cannot park in small spots

// This solution largely inspired by:
// https://github.com/careercup/CtCI-6th-Edition/tree/59018cfcb90292209275db1c4b3ed306d4b07d7f/Java/Ch%2007.%20Object-Oriented%20Design/Q7_04_Parking_Lot

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VehicleSize {
    Motorcycle,
    Compact,
    Large,
}

#[derive(Debug)]
struct Spot {
    size: VehicleSize,
    is_available: bool,
    row: u8,
}

impl Spot {
    fn set_availability(&mut self, is_available: bool) {
        self.is_available = is_available;
    }
}

struct Level {
    floor: u8,
    spots: Vec<Spot>,
}

const ROWS_IN_LEVEL: u8 = 4;
const SPOTS_IN_ROW: u8 = 12;
impl Level {
    fn new(floor: u8) -> Self {
        let motos_per_row = SPOTS_IN_ROW / 4;
        let compacts_per_row = SPOTS_IN_ROW / 4;
        Level {
            floor,
            spots: (0..ROWS_IN_LEVEL * SPOTS_IN_ROW)
                .map(|i| {
                    let size = if (i % SPOTS_IN_ROW) < motos_per_row {
                        VehicleSize::Motorcycle
                    } else if (i % SPOTS_IN_ROW) < compacts_per_row + motos_per_row {
                        VehicleSize::Compact
                    } else {
                        VehicleSize::Large
                    };
                    Spot {
                        size,
                        is_available: true,
                        row: i / SPOTS_IN_ROW,
                    }
                })
                .collect(),
        }
    }

    fn available_spots(&self) -> usize {
        println!("spots: {:?}", self.spots);
        self.spots.iter().filter(|spot| spot.is_available).count()
    }

    fn park_vehicle(&mut self, vehicle: &impl Parkable) -> Result<(), String> {
        let mut prev_row = 0;
        let mut prev_i = 0;
        let mut spots: Vec<&mut Spot> = vec![];
        for (i, spot) in self.spots.iter_mut().enumerate() {
            if prev_row != spot.row || i != prev_i + 1 {
                spots.clear();
            }
            prev_row = spot.row;
            prev_i = i;

            if spot.is_available && vehicle.can_fit_in_spot(spot) {
                spots.push(spot);
                if spots.len() == vehicle.slots_needed() {
                    break;
                }
            }
        }

        if spots.len() == vehicle.slots_needed() {
            for spot in spots.iter_mut() {
                spot.is_available = false;
            }
            Ok(())
        } else {
            Err(String::from("No parking available on this level!"))
        }
    }
}

struct ParkingLot {
    levels: Vec<Level>,
}
impl ParkingLot {
    fn new(levels: u8) -> Self {
        ParkingLot {
            levels: (0..levels).map(|level| Level::new(level)).collect(),
        }
    }
    fn park_vehicle(&mut self, vehicle: &impl Parkable) -> Result<(), String> {
        self.levels
            .iter_mut()
            .find_map(|level| level.park_vehicle(vehicle).ok())
            .ok_or(String::from("No parking in this lot!"))
    }
}

trait Parkable {
    fn slots_needed(&self) -> usize;
    fn can_fit_in_spot(&self, spot: &Spot) -> bool;
}

struct Motorcycle {}
impl Parkable for Motorcycle {
    fn slots_needed(&self) -> usize {
        1
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Motorcycle
    }
}

struct Car {}
impl Parkable for Car {
    fn slots_needed(&self) -> usize {
        1
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Compact
    }
}

struct Bus {}
impl Parkable for Bus {
    fn slots_needed(&self) -> usize {
        5
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Large
    }
}

#[test]
fn test_level() {
    let mut level = Level::new(1);
    assert_eq!(level.available_spots(), 48);
    assert!(level.park_vehicle(&Motorcycle {}).is_ok());
    assert_eq!(level.available_spots(), 47);
    assert!(level.park_vehicle(&Bus {}).is_ok());
    assert_eq!(level.available_spots(), 42);
    assert!(level.park_vehicle(&Car {}).is_ok());
    assert_eq!(level.available_spots(), 41);

    assert!(level.park_vehicle(&Bus {}).is_ok());
    assert_eq!(level.available_spots(), 36);
    assert!(level.park_vehicle(&Bus {}).is_ok());
    assert_eq!(level.available_spots(), 31);
    assert!(level.park_vehicle(&Bus {}).is_ok());
    assert_eq!(level.available_spots(), 26);
    assert!(level.park_vehicle(&Bus {}).is_err());
    assert!(level.park_vehicle(&Car {}).is_ok());
    assert!(level.park_vehicle(&Car {}).is_ok());
    assert!(level.park_vehicle(&Car {}).is_ok());
    assert!(level.park_vehicle(&Car {}).is_ok());
}

#[test]
fn test_parking_lot() {
    let mut lot = ParkingLot::new(2);

    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_ok());
    assert!(lot.park_vehicle(&Bus {}).is_err());
}
