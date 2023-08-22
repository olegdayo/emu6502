use emu6502::types::{BitFields, Flag};

const FLAGS: &[Flag] = &[
    Flag::Carry,
    Flag::Zero,
    Flag::Interrupt,
    Flag::Decimal,
    Flag::Break,
    Flag::Overflow,
    Flag::Negative,
];

#[test]
fn test_get() {
    let bf = BitFields::new(0b10100111);
    assert!(bf.get_flag(&Flag::Carry));
    assert!(bf.get_flag(&Flag::Zero));
    assert!(bf.get_flag(&Flag::Interrupt));
    assert!(!bf.get_flag(&Flag::Decimal));
    assert!(!bf.get_flag(&Flag::Break));
    assert!(bf.get_flag(&Flag::Overflow));
    assert!(!bf.get_flag(&Flag::Negative));
}

#[test]
fn test_set() {
    let mut bf = BitFields::default();
    for flag in FLAGS {
        bf.set_flag(flag, true);
        assert!(bf.get_flag(flag));
        bf.set_flag(flag, true);
        assert!(bf.get_flag(flag));

        bf.set_flag(flag, false);
        assert!(!bf.get_flag(flag));
        bf.set_flag(flag, true);
        assert!(bf.get_flag(flag));

        bf.set_flag(flag, false);
        assert!(!bf.get_flag(flag));
        bf.set_flag(flag, false);
        assert!(!bf.get_flag(flag));

        bf.set_flag(flag, true);
        assert!(bf.get_flag(flag));
        bf.set_flag(flag, false);
        assert!(!bf.get_flag(flag));
    }
}

#[test]
fn test_invert() {
    let mut bf = BitFields::default();
    for flag in FLAGS {
        for _ in 0..2 {
            bf.invert_flag(flag);
            assert!(bf.get_flag(flag));
            bf.invert_flag(flag);
            assert!(!bf.get_flag(flag));
        }
    }
}
