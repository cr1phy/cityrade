use crate::types::Map;

#[test]
fn test_new_map() {
    let map = Map::new(42, 10, 10);
    assert_eq!(map.seed, 42);
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 10);
    assert_eq!(map.chunks.len(), 100);
}

#[test]
fn test_generate_chunks() {
    let mut map = Map::new(42, 10, 10);
    map.generate_chunks(0, 0, 5, 5);
    assert_eq!(map.chunks.len(), 125);
}

#[test]
fn test_get_chunk() {
    let map = Map::new(42, 10, 10);
    let chunk = map.get_chunk(0, 0);
    assert!(chunk.is_some());
    assert_eq!(chunk.unwrap().x, 0);
    assert_eq!(chunk.unwrap().y, 0);
}