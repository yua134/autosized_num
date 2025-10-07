use autosized_num::*;
use static_assertions::assert_type_eq_all;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsigned() {
        assert_type_eq_all!(auto_sized_unsigned!(10), u8);
        assert_type_eq_all!(auto_sized_unsigned!(257), u16);
        assert_eq!(auto_sized_unsigned_val!(10), 10u8);
    }

    #[test]
    fn singed() {
        assert_type_eq_all!(auto_sized_signed!(-10), i8);
        assert_type_eq_all!(auto_sized_signed!(129), i16);
        assert_eq!(auto_sized_signed_val!(-10), -10i8);
    }

    #[test]
    fn int() {
        assert_type_eq_all!(auto_sized_int!(-10), i8);
        assert_type_eq_all!(auto_sized_int!(1_000_000_000), u32);
        assert_eq!(auto_sized_int_val!(-100_000_000), -100_000_000i32);
    }
}
