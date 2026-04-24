use fixedstr::str256;

pub trait AddressFromKey: Sized {
    fn from_key(key: str256) -> Self;
}