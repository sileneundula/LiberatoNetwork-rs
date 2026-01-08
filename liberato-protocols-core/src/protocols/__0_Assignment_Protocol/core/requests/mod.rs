use fixedstr::str256;

pub struct RequestAssignment {
    pub addr: str256,
    pub public_key: str256,
    pub ecies_key: str256,
    
    pub source: u16, // 
    
    pub request_id: u32,
    pub request_name: str256,
}