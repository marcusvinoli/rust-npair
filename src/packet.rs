use crate::data_object::DataObject;

const STARTER_CHAR: char = '$';
const DELIMITER_LEFT_CHAR: char = '{';
const DELIMITER_RIGHT_CHAR: char = '}';
const ATTRIBUTION_CHAR: char = ':';
const SEPARATOR_CHAR: char = ',';
const VERSION_MAJOR: char = '0';
const VERSION_MINOR: char = '0';
const VERSION_BUILD: char = '1';
const END_OF_PACKET: char = '\n';

pub struct Packet {
    payload: Vec<DataObject>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            payload: Vec::new(),
        }
    }

    pub fn parse(&mut self, data: Vec<char>) -> bool {
        self.payload.clear();

        let mut raw_data = data.into_iter();
        let mut return_packet: bool = false;

        if let Some(starter_char) = raw_data.next() {
            if starter_char != STARTER_CHAR {
                return return_packet;
            }
        }

        if let Some(version_major) = raw_data.next() {
            if version_major != VERSION_MAJOR {
                return return_packet;
            }
        }

        if let Some(version_minor) = raw_data.next() {
            if version_minor != VERSION_MINOR {
                return return_packet;
            }
        }

        if let Some(version_build) = raw_data.next() {
            if version_build != VERSION_BUILD {
                return return_packet;
            }
        }

        if let Some(packet_size) = raw_data.next() {
            if packet_size == '0' {
                return return_packet;
            }
        }

        if let Some(delimiter) = raw_data.next() {
            if delimiter != DELIMITER_LEFT_CHAR {
                return return_packet;
            }
        }

        let mut address_vec: Vec<char> = Vec::new();
        let mut data_vec: Vec<char> = Vec::new();

        while let Some(x) = raw_data.next() {
            if x == DELIMITER_RIGHT_CHAR {
                break;
            }
            if x == ATTRIBUTION_CHAR {
                let addr: String = address_vec.iter().collect();
                let addr: u16 = addr.parse().unwrap_or(0);
                let mut data_object: DataObject = DataObject::new(addr);
                while let Some(x) = raw_data.next() {
                    if x == SEPARATOR_CHAR || x == DELIMITER_RIGHT_CHAR {
                        let data: String = data_vec.iter().collect();
                        data_object.set_data(data);
                        self.payload.push(data_object);
                        return_packet = true;
                        break;
                    }
                    data_vec.push(x);
                }
                address_vec.clear();
                data_vec.clear();

                if x == DELIMITER_RIGHT_CHAR {
                    return return_packet;
                } else {
                    continue;
                }
            }
            address_vec.push(x);
        }

        return return_packet;
    }

    pub fn remove_data(&mut self, address: u16) {
        self.payload
            .retain(|data_object| data_object.get_address() != address);
    }

    pub fn insert_data<T: ToString>(&mut self, addr: u16, data: T) {
        let mut data_object = DataObject::new(addr);
        data_object.set_data(data);
        self.insert_data_object(data_object);
    }

    pub fn insert_data_object(&mut self, data_object: DataObject) {
        self.payload.push(data_object);
    }

    pub fn get_data<T: std::str::FromStr>(&self, address: &u16) -> Option<T> {
        for data_object in &self.payload {
            if data_object.get_address() == *address {
                if let Ok(x) = data_object.get_data().parse::<T>() {
                    return Some(x);
                }
            }
        }
        return None;
    }

    pub fn to_char_vector(&mut self) -> Vec<char> {
        let mut char_vect: Vec<char> = Vec::new();
        let mut char_payload: Vec<char> = self.payload_assembly();
        let char_payload_size: u8 = char_payload.len() as u8;
        char_vect.push(STARTER_CHAR);
        char_vect.push(VERSION_MAJOR);
        char_vect.push(VERSION_MINOR);
        char_vect.push(VERSION_BUILD);
        char_vect.push(char_payload_size as char);
        char_vect.push(DELIMITER_LEFT_CHAR);
        char_vect.append(&mut char_payload);
        char_vect.push(DELIMITER_RIGHT_CHAR);
        char_vect.push(END_OF_PACKET);
        return char_vect;
    }

    pub fn from_char_vector(&mut self, data: Vec<char>) -> bool {
        return self.parse(data);
    }

    pub fn clear(&mut self) {
        self.payload.clear();
    }

    pub fn update(&mut self, data_object: DataObject) {
        self.remove_data(data_object.get_address());
        self.insert_data_object(data_object);
    }

    fn payload_assembly(&self) -> Vec<char> {
        let mut payload: Vec<char> = Vec::new();
        let mut payload_iter = self.payload.iter();
        while let Some(data_object) = payload_iter.next() {
            let mut data_object_as_vec = Packet::data_object_to_vec(data_object);
            payload.append(&mut data_object_as_vec);
            payload.push(SEPARATOR_CHAR);
        }
        if payload.len() > 0 {
            payload.pop();
        }
        return payload;
    }

    fn data_object_to_vec(data_object: &DataObject) -> Vec<char> {
        let mut ret_vec: Vec<char> = Vec::new();
        ret_vec = data_object.get_address().to_string().chars().collect();
        ret_vec.push(ATTRIBUTION_CHAR);
        let mut value: Vec<char> = data_object.get_data().chars().collect();
        ret_vec.append(&mut value);
        return ret_vec;
    }
}
