use super::sdt::SdtHeader;

#[derive(Debug)]
pub struct Xsdt(&'static SdtHeader);

impl Xsdt {
    pub fn new(sdt: &'static SdtHeader) -> Option<Xsdt> {
        match &sdt.signature {
            b"XSDT" => Some(Xsdt(sdt)),
            _ => None,
        }
    }
}
