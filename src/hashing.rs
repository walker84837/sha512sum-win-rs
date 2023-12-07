use sha2::{Sha512, Digest};

pub struct Sha512Sum;

impl Sha512Sum {
    pub fn get_checksum(data: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data);
        let checksum = format!("{:x}", hasher.finalize());
        println!("{}", checksum);
        checksum
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sha512sum() {
        let data = b"i use arch btw\n";

        // echo 'i use arch btw' | sha512sum -b
        let expected_checksum = "2ddbe9f9af5a630d3734ce469fac19088e8d0242541768630777de5c56dc4053d346a67527cb95de3ab094d6862f393392ba26bed459d9ad149b423aeae552a2"
            .to_owned();
        let actual_checksum = Sha512Sum::get_checksum(data);
        assert_eq!(actual_checksum, expected_checksum);
    }
}
