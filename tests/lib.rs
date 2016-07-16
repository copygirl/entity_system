extern crate entity_system;

use entity_system::*;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Location(f32, f32, f32);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Health(i32);


#[test]
fn test() {
    let mut ec_map = storage::Map::<Entity>::new();
    
    let entity   = Entity(0);
    let location = Location(1.0, 2.0, 3.0);
    
    ec_map.insert(entity, location);
    assert_eq!(ec_map.borrow::<Location>(entity), Some(&Location(1.0, 2.0, 3.0)));
    assert_eq!(ec_map.get::<Location>(entity), Some(Location(1.0, 2.0, 3.0)));
    assert_eq!(ec_map.get::<Health>(entity), None);
    
    ec_map.remove::<Location>(entity);
    assert_eq!(ec_map.get::<Location>(entity), None);
}
