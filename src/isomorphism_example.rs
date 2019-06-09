#[derive(PartialEq, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

#[test]
fn isomorphism_example() {
    let pair_to_coords = | pair: (i32, i32) | Coords { x: pair.0, y: pair.1 };
    let coords_to_pair = | coords: Coords | (coords.x, coords.y);
    assert_eq!(
        pair_to_coords((1, 2)),
        Coords { x: 1, y: 2 },
    );
    assert_eq!(
        coords_to_pair(Coords { x: 1, y: 2 }),
        (1, 2),
    );
}
