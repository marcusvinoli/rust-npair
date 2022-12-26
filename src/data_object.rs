#[derive(Clone, Debug)]
pub struct DataObject {
    address: u16,
    data: Vec<char>,
}

impl DataObject {
    pub fn new(address: u16) -> DataObject {
        DataObject {
            address: address,
            data: Vec::new(),
        }
    }

    pub fn get_data(&self) -> String {
        let mut return_string: String = self.data.iter().collect();
        if return_string == String::from("true") {
            return_string = String::from("1");
        } else if return_string == String::from("false") {
            return_string = String::from("0");
        }
        return return_string.to_owned();
    }

    pub fn set_data<T: ToString>(&mut self, data: T) {
        let data_char_vector = data.to_string().chars().collect();
        self.data = data_char_vector;
    }

    pub fn get_address(&self) -> u16 {
        return self.address;
    }
}
