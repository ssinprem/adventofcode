use restroon_redoubt::*;

#[test]
fn test_robot() {
    let mut robot = Robot::new(
        "p=0,4 v=3,-3".to_string(),
        (11,7)
    );
    assert_eq!(robot.pos, (0,4));
    assert_eq!(robot.vec, (3,-3));
    assert_eq!(robot.limit, (11,7));
    robot.moved();
    assert_eq!(robot.pos, (3,1));
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    assert_eq!(robot.pos, (6,5));
    assert_eq!(robot.get_quadrant(), 2);
    robot.moved();
    assert_eq!(robot.pos, (9,2));
    assert_eq!(robot.get_quadrant(), 1);
    robot.moved();
    assert_eq!(robot.pos, (1,6));
    assert_eq!(robot.get_quadrant(), 3);
    robot.moved();
    assert_eq!(robot.pos, (4,3));
    assert_eq!(robot.get_quadrant(), 0);
    robot.moved();
    assert_eq!(robot.pos, (7,0));
    assert_eq!(robot.get_quadrant(), 1);
}

#[test]
fn test_robot2() {
    let limit = (11,7);
    let mut robot = Robot::new(
        "p=0,0 v=-1,-1".to_string(),
        limit.clone()
    );
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (0,0));
    assert_eq!(robot.vec, (-1,-1));
    assert_eq!(robot.limit, limit.clone());
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (10,6));
    assert_eq!(robot.get_quadrant(), 2);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (9,5));
    assert_eq!(robot.get_quadrant(), 2);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (8,4));
    assert_eq!(robot.get_quadrant(), 2);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (7,3));
    assert_eq!(robot.get_quadrant(), 0);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (6,2));
    assert_eq!(robot.get_quadrant(), 1);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (5,1));
    assert_eq!(robot.get_quadrant(), 0);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (4,0));
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (3,6));
    assert_eq!(robot.get_quadrant(), 3);
}

#[test]
fn test_robot3() {
    let limit = (11,7);
    let mut robot = Robot::new(
        "p=0,0 v=1,1".to_string(),
        limit.clone()
    );
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (0,0));
    assert_eq!(robot.vec, (1,1));
    assert_eq!(robot.limit, limit.clone());
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (1,1));
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (2,2));
    assert_eq!(robot.get_quadrant(), 4);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (3,3));
    assert_eq!(robot.get_quadrant(), 0);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (4,4));
    assert_eq!(robot.get_quadrant(), 3);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (5,5));
    assert_eq!(robot.get_quadrant(), 0);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (6,6));
    assert_eq!(robot.get_quadrant(), 2);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (7,0));
    assert_eq!(robot.get_quadrant(), 1);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (8,1));
    assert_eq!(robot.get_quadrant(), 1);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (9,2));
    assert_eq!(robot.get_quadrant(), 1);
    robot.moved();
    println!("{}",print_map(&[robot],limit.clone()));
    assert_eq!(robot.pos, (10,3));
    assert_eq!(robot.get_quadrant(), 0);
}