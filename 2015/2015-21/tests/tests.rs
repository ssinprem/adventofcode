use rpg_simulator_20xx::*;

#[test]
fn test_fight() {
    let p1 = Fighter {
        hitpoint: 8,
        damage: 5,
        armor: 5,
    };
    let p2 = Fighter {
        hitpoint: 12,
        damage: 7,
        armor: 2,
    };
    assert!(is_fight_win(&p1, &p2));
}
