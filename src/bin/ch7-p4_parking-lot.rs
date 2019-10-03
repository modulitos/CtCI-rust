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

// but uses ECS instead of traditional OOP:
// https://en.wikipedia.org/wiki/Entity_component_system

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VehicleSize {
    Motorcycle,
    Compact,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SpotId(usize);

#[derive(Debug)]
struct Spot {
    size: VehicleSize,
    is_available: bool,
    row: u8,
    id: SpotId,
}

struct Level {
    floor: usize,
    spots: Vec<Spot>,
}

const ROWS_IN_LEVEL: u8 = 4;
const SPOTS_IN_ROW: u8 = 12;
impl Level {
    fn new(floor: usize) -> Self {
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
                        id: SpotId(usize::from(i)),
                    }
                })
                .collect(),
        }
    }

    fn available_spots(&self) -> usize {
        println!("spots: {:?}", self.spots);
        self.spots.iter().filter(|spot| spot.is_available).count()
    }

    fn park_vehicle(&mut self, vehicle: &mut impl Parkable) -> Result<(), String> {
        let mut prev_row = 0;
        let mut prev_i = 0;
        let mut spots: Vec<&mut Spot> = vec![];
        // HELP: would it be clearer to write this in a functional manner?
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
            vehicle.park(self.floor, spots.into_iter().map(|s| s.id).collect());
            Ok(())
        } else {
            Err(String::from("No parking available on this level!"))
        }
    }

    fn clear_vehicle(&mut self, parked_vehicle: &ParkedVehicle) {
        let spots: Vec<&mut Spot> = self
            .spots
            .iter_mut()
            .filter(|spot| parked_vehicle.spots.contains(&spot.id))
            .collect();
        for spot in spots {
            spot.is_available = true;
        }
    }
}

struct ParkingLot {
    levels: Vec<Level>,
}
impl ParkingLot {
    fn new(levels: usize) -> Self {
        ParkingLot {
            levels: (0..levels).map(|level| Level::new(level)).collect(),
        }
    }
    fn park_vehicle(&mut self, vehicle: &mut impl Parkable) -> Result<(), String> {
        self.levels
            .iter_mut()
            .find_map(|level| level.park_vehicle(vehicle).ok())
            .ok_or(String::from("No parking in this lot!"))
    }

    fn clear_vehicle(&mut self, vehicle: &mut impl Parkable) -> Result<ParkedVehicle, String> {
        match vehicle.leave() {
            Ok(parked_vehicle) => {
                if let Some(level) = self
                    .levels
                    .iter_mut()
                    .find(|level| level.floor == parked_vehicle.level)
                {
                    level.clear_vehicle(&parked_vehicle);
                    Ok(parked_vehicle)
                } else {
                    Err(String::from("no parking level found!"))
                }
            }
            Err(e) => Err(e),
        }
    }
}

trait Parkable {
    fn slots_needed(&self) -> usize;
    fn can_fit_in_spot(&self, spot: &Spot) -> bool;
    // HELP: These methods all have the same implementation. How to
    // leverage code re-use?
    fn park(&mut self, level: usize, spots: Vec<SpotId>);
    fn leave(&mut self) -> Result<ParkedVehicle, String>;
}

#[derive(Debug, PartialEq)]
struct ParkedVehicle {
    level: usize,
    spots: Vec<SpotId>,
}

struct Motorcycle {
    parked: Option<ParkedVehicle>,
}
impl Motorcycle {
    fn new() -> Self {
        Motorcycle { parked: None }
    }
}

impl Parkable for Motorcycle {
    fn slots_needed(&self) -> usize {
        1
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Motorcycle
    }
    fn park(&mut self, level: usize, spots: Vec<SpotId>) {
        self.parked = Some(ParkedVehicle { level, spots });
    }
    fn leave(&mut self) -> Result<ParkedVehicle, String> {
        if let Some(parked) = self.parked.take() {
            self.parked = None;
            Ok(parked)
        } else {
            Err(String::from("Wasn't parked!"))
        }
    }
}

struct Car {
    parked: Option<ParkedVehicle>,
}
impl Car {
    fn new() -> Self {
        Car { parked: None }
    }
}
impl Parkable for Car {
    fn slots_needed(&self) -> usize {
        1
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Compact
    }
    fn park(&mut self, level: usize, spots: Vec<SpotId>) {
        self.parked = Some(ParkedVehicle { level, spots });
    }
    fn leave(&mut self) -> Result<ParkedVehicle, String> {
        if let Some(parked) = self.parked.take() {
            self.parked = None;
            Ok(parked)
        } else {
            Err(String::from("Wasn't parked!"))
        }
    }
}

struct Bus {
    parked: Option<ParkedVehicle>,
}
impl Bus {
    fn new() -> Self {
        Bus { parked: None }
    }
}
impl Parkable for Bus {
    fn slots_needed(&self) -> usize {
        5
    }
    fn can_fit_in_spot(&self, spot: &Spot) -> bool {
        spot.size >= VehicleSize::Large
    }
    fn park(&mut self, level: usize, spots: Vec<SpotId>) {
        self.parked = Some(ParkedVehicle { level, spots });
    }
    fn leave(&mut self) -> Result<ParkedVehicle, String> {
        if let Some(parked) = self.parked.take() {
            self.parked = None;
            Ok(parked)
        } else {
            Err(String::from("Wasn't parked!"))
        }
    }
}

#[test]
fn test_level_park() {
    let mut level: Level = Level::new(1);
    assert_eq!(level.available_spots(), 48);
    assert!(level.park_vehicle(&mut Motorcycle::new()).is_ok());
    assert_eq!(level.available_spots(), 47);
    assert!(level.park_vehicle(&mut Bus::new()).is_ok());
    assert_eq!(level.available_spots(), 42);
    assert!(level.park_vehicle(&mut Car::new()).is_ok());
    assert_eq!(level.available_spots(), 41);

    assert!(level.park_vehicle(&mut Bus::new()).is_ok());
    assert_eq!(level.available_spots(), 36);
    assert!(level.park_vehicle(&mut Bus::new()).is_ok());
    assert_eq!(level.available_spots(), 31);
    assert!(level.park_vehicle(&mut Bus::new()).is_ok());
    assert_eq!(level.available_spots(), 26);
    assert!(level.park_vehicle(&mut Bus::new()).is_err());
    assert!(level.park_vehicle(&mut Car::new()).is_ok());
    assert!(level.park_vehicle(&mut Car::new()).is_ok());
    assert!(level.park_vehicle(&mut Car::new()).is_ok());
    assert!(level.park_vehicle(&mut Car::new()).is_ok());
}

#[test]
fn test_level_unpark() {
    let mut level: Level = Level::new(1);
    assert_eq!(level.available_spots(), 48);
    let mut motorcycle = Motorcycle::new();
    assert!(level.park_vehicle(&mut motorcycle).is_ok());
    assert_eq!(level.available_spots(), 47);

    let mut bus = Bus::new();
    assert!(level.park_vehicle(&mut bus).is_ok());
    assert_eq!(level.available_spots(), 42);

    assert_eq!(
        &motorcycle.parked,
        &Some(ParkedVehicle {
            level: 1,
            spots: vec![SpotId(0)]
        })
    );
    level.clear_vehicle(&motorcycle.parked.unwrap());
    assert_eq!(level.available_spots(), 43);
    assert_eq!(
        &bus.parked,
        &Some(ParkedVehicle {
            level: 1,
            spots: vec![SpotId(6), SpotId(7), SpotId(8), SpotId(9), SpotId(10)]
        })
    );
    level.clear_vehicle(&bus.parked.unwrap());
    assert_eq!(level.available_spots(), 48);
}

#[test]
fn test_parking_lot() {
    let mut lot = ParkingLot::new(2);

    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_err());
}

#[test]
fn test_parking_lot_unpark() {
    let mut lot = ParkingLot::new(3);

    // level 0:
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    // level 1:
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    // level 2:
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());
    assert!(lot.park_vehicle(&mut Bus::new()).is_ok());

    let mut bus = Bus::new();
    assert!(lot.park_vehicle(&mut bus).is_ok());
    assert_eq!(
        &bus.parked,
        &Some(ParkedVehicle {
            level: 2,
            spots: vec![SpotId(30), SpotId(31), SpotId(32), SpotId(33), SpotId(34)]
        })
    );
    assert!(lot.clear_vehicle(&mut bus).is_ok());
    assert_eq!(bus.parked, None);
}
