use super::sdt::SdtHeader;

#[derive(Debug)]
pub struct Rsdt(&'static SdtHeader);

impl Rsdt {
    pub fn new(sdt: &'static SdtHeader) -> Option<Self> {
        match &sdt.signature {
            b"RSDT" => Some(Rsdt(sdt)),
            _ => None,
        }
    }
}
