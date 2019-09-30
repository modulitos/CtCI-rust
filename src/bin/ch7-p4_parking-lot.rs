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

struct Spot {
    size: VehicleSize,
    is_available: bool,
    row: u8,
    level: u8,
}

struct Level {
    floor: u8,
    spots: Vec<Spot>,
}

const ROWS_IN_LEVEL: u8 = 4;
const SPOTS_IN_ROW: u8 = 12;
impl Level {
    fn new(floor: u8) -> Self {
        Level {
            floor,
            spots: (0..ROWS_IN_LEVEL * SPOTS_IN_ROW)
                .map(|i| Spot {
                    size: VehicleSize::Compact,
                    is_available: true,
                    level: i / ROWS_IN_LEVEL,
                    row: i % SPOTS_IN_ROW,
                })
                .collect(),
        }
    }

    fn available_spots(&self) -> usize {
        self.spots.iter().filter(|spot| spot.is_available).count()
    }
}

struct ParkingLot {
    levels: Vec<Level>,
}

struct Vehicle {
    size: VehicleSize,
}

#[test]
fn test_level() {
    let level = Level::new(1);
    assert_eq!(level.available_spots(), 48);
}
