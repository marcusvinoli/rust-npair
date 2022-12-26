use std::str::FromStr;

use crate::data_object::DataObject;
use crate::packet::Packet;

pub struct SampledValue<T: FromStr + Default> {
    data_object: DataObject,
    value: T,
}

impl<T: FromStr + Default> SampledValue<T> {
    pub fn new(address: u16) -> SampledValue<T> {
        SampledValue {
            data_object: DataObject::new(address),
            value: Default::default(),
        }
    }

    pub fn parse(&mut self, pckt: &Packet) -> bool {
        if let Some(value) = pckt.get_data(&self.data_object.get_address()) {
            self.value = value;
            return true;
        }
        return false;
    }

    pub fn get_value(&self) -> &T {
        return &self.value;
    }
}
