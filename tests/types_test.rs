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
    assert!(bf.get_bit(&Flag::Carry));
    assert!(bf.get_bit(&Flag::Zero));
    assert!(bf.get_bit(&Flag::Interrupt));
    assert!(!bf.get_bit(&Flag::Decimal));
    assert!(!bf.get_bit(&Flag::Break));
    assert!(bf.get_bit(&Flag::Overflow));
    assert!(!bf.get_bit(&Flag::Negative));
}

#[test]
fn test_set() {
    let mut bf = BitFields::default();
    for flag in FLAGS {
        bf.set_bit(flag, true);
        assert!(bf.get_bit(flag));
        bf.set_bit(flag, true);
        assert!(bf.get_bit(flag));

        bf.set_bit(flag, false);
        assert!(!bf.get_bit(flag));
        bf.set_bit(flag, true);
        assert!(bf.get_bit(flag));

        bf.set_bit(flag, false);
        assert!(!bf.get_bit(flag));
        bf.set_bit(flag, false);
        assert!(!bf.get_bit(flag));

        bf.set_bit(flag, true);
        assert!(bf.get_bit(flag));
        bf.set_bit(flag, false);
        assert!(!bf.get_bit(flag));
    }
}

#[test]
fn test_invert() {
    let mut bf = BitFields::default();
    for flag in FLAGS {
        for _ in 0..2 {
            bf.invert_bit(flag);
            assert!(bf.get_bit(flag));
            bf.invert_bit(flag);
            assert!(!bf.get_bit(flag));
        }
    }
}
