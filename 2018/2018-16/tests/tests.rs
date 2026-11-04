use chronal_classification::*;

#[test]
fn test_operate() {
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Addr, &[0, 1, 3]),
        [0, 1, 2, 1]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Addi, &[0, 4, 3]),
        [0, 1, 2, 4]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Mulr, &[1, 2, 3]),
        [0, 1, 2, 2]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Muli, &[2, 4, 3]),
        [0, 1, 2, 8]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Banr, &[2, 2, 3]),
        [0, 1, 2, 2]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Bani, &[2, 1, 3]),
        [0, 1, 2, 0]
    );
    assert_eq!(
        operate(&[0, 1, 2, 0], &Opcode::Borr, &[2, 1, 3]),
        [0, 1, 2, 3]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Bori, &[2, 4, 3]),
        [0, 1, 2, 6]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Setr, &[2, 4, 3]),
        [0, 1, 2, 2]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Seti, &[6, 4, 3]),
        [0, 1, 2, 6]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Gtir, &[0, 1, 3]),
        [0, 1, 2, 0]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Gtri, &[2, 1, 3]),
        [0, 1, 2, 1]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Gtrr, &[0, 1, 3]),
        [0, 1, 2, 0]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Eqir, &[0, 0, 3]),
        [0, 1, 2, 1]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Eqri, &[2, 2, 3]),
        [0, 1, 2, 1]
    );
    assert_eq!(
        operate(&[0, 1, 2, 3], &Opcode::Eqrr, &[0, 1, 3]),
        [0, 1, 2, 0]
    );
}
