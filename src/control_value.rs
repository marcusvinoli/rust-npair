use std::str::FromStr;

use crate::data_object::DataObject;
use crate::packet::Packet;

pub struct ControlValue<T: FromStr + ToString + Default> {
    data_object: DataObject,
    value: T,
}

impl<T: FromStr + ToString + Default + Copy> ControlValue<T> {
    pub fn new(address: u16) -> ControlValue<T> {
        ControlValue {
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

    pub fn set_value(&mut self, value: &T) {
        self.value = value.clone();
        self.data_object.set_data(self.value);
    }

    pub fn get_data_object(&self) -> DataObject {
        return self.data_object.clone();
    }
}
