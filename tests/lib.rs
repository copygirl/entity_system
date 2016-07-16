extern crate entity_system;

use entity_system::*;


#[test]
fn test() {
    let entity   = Entity(0);
    let location = [ 0f32; 3 ];
    
    let mut ec_map = storage::Map::<Entity>::new();
    ec_map.insert(entity, location);
    
    assert_eq!(ec_map.borrow::<[f32;3]>(entity), Some(&[ 0f32; 3 ]));
    assert_eq!(ec_map.get::<[f32;3]>(entity), Some([ 0f32; 3 ]));
    assert_eq!(ec_map.get::<[f32;2]>(entity), None);
    
    ec_map.remove::<[f32;3]>(entity);
    
    assert_eq!(ec_map.get::<[f32;3]>(entity), None);
}
