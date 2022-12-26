use rust_npair::*;

#[cfg(test)]
mod data_object_test {
    #[test]
    fn test_creation() {
        let test_data_object = super::data_object::DataObject::new(15);
        assert_eq!(test_data_object.get_data(), String::from(""));
        assert_eq!(test_data_object.get_address(), 15);
    }

    #[test]
    fn test_string_value() {
        let mut test_data_object = super::data_object::DataObject::new(10);
        assert_eq!(test_data_object.get_data(), String::from(""));
        test_data_object.set_data(String::from("Hello"));
        assert_eq!(test_data_object.get_data(), String::from("Hello"));
    }

    #[test]
    fn test_string_integer() {
        let mut test_data_object = super::data_object::DataObject::new(5);
        assert_eq!(test_data_object.get_data(), String::from(""));
        test_data_object.set_data(100);
        assert_eq!(test_data_object.get_data(), String::from("100"));
    }

    #[test]
    fn test_string_boolean() {
        let mut test_data_object = super::data_object::DataObject::new(5);
        assert_eq!(test_data_object.get_data(), String::from(""));
        test_data_object.set_data(false);
        assert_eq!(test_data_object.get_data(), String::from("0"));
    }
}

#[cfg(test)]
mod packet_test {
    #[test]
    fn test_packet_from_vec() {
        let ok_packet: Vec<char> = vec![
            '$', '0', '0', '1', 'X', '{', '1', ':', '2', '6', ',', '9', ':', '1', '0', '}', '\n',
        ];

        let error_packet: Vec<char> =
            vec!['%', '0', '0', '1', 'X', '{', '1', ':', '2', '6', '}', '\n'];

        let mut in_packet = super::packet::Packet::new();

        assert_eq!(in_packet.parse(error_packet), false);
        assert_eq!(in_packet.parse(ok_packet), true);

        assert_eq!(in_packet.get_data::<i16>(&1), Some(26));
        assert_eq!(in_packet.get_data::<i16>(&9), Some(10));
        assert_eq!(in_packet.get_data::<i16>(&10), None);
        assert_eq!(in_packet.get_data::<i16>(&5), None);
    }

    #[test]
    fn test_vec_from_packet() {
        let ok_packet: Vec<char> = vec![
            '$', '0', '0', '1', 'X', '{', '1', ':', '2', '6', ',', '9', ':', '1', '0', '}', '\n',
        ];
        let mut in_packet = super::packet::Packet::new();
        assert_eq!(in_packet.parse(ok_packet), true);

        let out_packet: Vec<char> = in_packet.to_char_vector();

        let ok_packet: Vec<char> = vec![
            '$', '0', '0', '1', 9 as char, '{', '1', ':', '2', '6', ',', '9', ':', '1', '0', '}',
            '\n',
        ];
        for i in 0..out_packet.len() {
            println!("{:?}", out_packet[i]);
            //assert_eq!(out_packet[i], ok_packet[i]);
        }
        assert_eq!(out_packet, ok_packet);
    }
}

#[cfg(test)]
mod sampled_value_test {
    #[test]
    fn test_sv_constructor() {
        let mut my_sv: crate::sampled_value::SampledValue<isize> =
            crate::sampled_value::SampledValue::new(9);

        assert_eq!(my_sv.get_value(), &0);

        let ok_packet: Vec<char> = vec![
            '$', '0', '0', '1', 'X', '{', '1', ':', '2', '6', ',', '9', ':', '1', '0', '}', '\n',
        ];
        let mut in_packet = super::packet::Packet::new();
        assert!(in_packet.parse(ok_packet));
        assert!(my_sv.parse(&in_packet));
        assert_eq!(my_sv.get_value(), &10);
    }
}
