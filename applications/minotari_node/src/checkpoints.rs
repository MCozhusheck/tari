use std::str::FromStr;

use hickory_client::{
    client::{Client, SyncClient},
    op::DnsResponse,
    rr::{DNSClass, Name, RData, Record, RecordType},
    udp::UdpClientConnection,
};

pub fn fetch_checkpoints() -> Result<Vec<(u64, String)>, anyhow::Error> {
    let address = "8.8.8.8:53".parse()?;
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);
    let name = Name::from_str("checkpoints-nextnet.tari.com")?;
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::TXT)?;

    let answers: &[Record] = response.answers();
    let checkpoints: Vec<(u64, String)> = answers
        .iter()
        .filter_map(|record| {
            if let RData::TXT(txt) = record.data() {
                let ascii_txt = txt.txt_data().iter().fold(String::new(), |mut acc, bytes| {
                    acc.push_str(&String::from_utf8_lossy(bytes));
                    acc
                });
                let (height, hash) = ascii_txt.split_once(':')?;
                return Some((height.parse().unwrap(), hash.to_string()));
            }
            None
        })
        .collect();

    Ok(checkpoints)
}

#[cfg(test)]
mod tests {
    use crate::checkpoints::fetch_checkpoints;

    #[test]
    fn test_fetch_checkpoints() {
        let res = fetch_checkpoints();
        assert!(res.is_ok());
        let checkpoints = res.unwrap();
        checkpoints.iter().for_each(|(height, hash)| {
            println!("Height: {}, Hash: {}", height, hash);
        });
    }
}
