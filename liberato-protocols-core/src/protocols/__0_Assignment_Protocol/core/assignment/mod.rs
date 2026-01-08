use fixedstr::str256;
use fixedstr::str64;

pub trait Assign {
    fn sign_assignment_using_keypair();
}


pub struct AssignmentLabel {
    assigned: str256,
    protocol: str64,
}