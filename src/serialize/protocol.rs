#[allow(non_camel_case_types)]
pub enum ProtocolVersion {
    V_1_12_2 = 340, // 1.12.2
}

pub trait ProtocolToID {
    fn resolve_id(&self, ver: &ProtocolVersion) -> i32;
}
