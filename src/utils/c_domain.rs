use std::net::{SocketAddr, IpAddr};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::{tcp::TcpClientConnection, udp::UdpClientConnection};
use url::{Url, Host, Position, ParseError};
use std::str::FromStr;


/// 获取ip
fn get_ip_from_url(url: &str) -> Option<IpAddr> {
    let url = match Url::parse(url) {
        Ok(url) => {url}
        Err(e) => {
            println!("地址错误：{}, e: {:?}", url,e);
            return None;
        }
    };
    // let port = url.port().unwrap_or(1935);
    let host = match url.host_str() {
        Some(host) => {host}
        None => {
            println!("地址错误：{}, host为空", url);
            return None;
        }
    };
    
    match IpAddr::from_str(host) {
        Ok(addr) => {
            return Some(addr);
        }
        _ => {}
    }
    
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = TcpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    
    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(format!("{}.", host).as_str()).unwrap();

// NOTE: see 'Setup a connection' example above
// Send the query and get a message response, see RecordType for all supported options
    let response: DnsResponse = match client.query(&name, DNSClass::IN, RecordType::A) {
        Ok(res) => {res}
        Err(e) => {
            println!("name错误：{}, e: {:?}", host,e);
            return None;
        }
    };

// Messages are the packets sent between client and server in DNS.
//  there are many fields to a Message, DnsResponse can be dereferenced into
//  a Message. It's beyond the scope of these examples
//  to explain all the details of a Message. See trust_dns_client::op::message::Message for more details.
//  generally we will be interested in the Message::answers
    let answers: &[Record] = response.answers();
    
    println!("answers[0]  {:#?}", answers);
//    if answers.len()>0 {
//        println!("answers[1]  {:#?}",answers[1]);
//    }


// Records are generic objects which can contain any data.
//  In order to access it we need to first check what type of record it is
//  In this case we are interested in A, IPv4 address
    for v in answers.iter() {
        match v.rdata() {
            &RData::A(addr) => {
                println!("ip  {:#?}", addr);
                return Some(IpAddr::from(addr));
            }
            &RData::AAAA(addr) => {
                println!("ip  {:#?}", addr);
                return Some(IpAddr::from(addr));
            }
            &RData::ANAME(_) => {}
            &RData::CAA(_) => {}
            &RData::CNAME(_) => {}
            &RData::MX(_) => {}
            &RData::NAPTR(_) => {}
            &RData::NULL(_) => {}
            &RData::NS(_) => {}
            &RData::OPENPGPKEY(_) => {}
            &RData::OPT(_) => {}
            &RData::PTR(_) => {}
            &RData::SOA(_) => {}
            &RData::SRV(_) => {}
            &RData::SSHFP(_) => {}
            &RData::TLSA(_) => {}
            &RData::TXT(_) => {}
            &RData::DNSSEC(_) => {}
            &RData::Unknown { code: _, rdata: _ } => {}
            &RData::ZERO => {}
        }
    }
    
    None
}

/// 通过url地址获取ips
fn get_ips_from_url(url: &str) -> Vec<IpAddr> {
    let mut ips =Vec::new();
    
    let url = match Url::parse(url) {
        Ok(url) => {url}
        Err(e) => {
            println!("地址错误：{}, e: {:?}", url,e);
            return ips;
        }
    };
    // let port = url.port().unwrap_or(1935);
    let host = match url.host_str() {
        Some(host) => {host}
        None => {
            println!("地址错误：{}, host为空", url);
            return ips;
        }
    };
    
    match IpAddr::from_str(host) {
        Ok(addr) => {
            ips.push(addr);
            return ips;
        }
        _ => {}
    }
    
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = TcpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    
    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(format!("{}.", host).as_str()).unwrap();

// NOTE: see 'Setup a connection' example above
// Send the query and get a message response, see RecordType for all supported options
    let response: DnsResponse = match client.query(&name, DNSClass::IN, RecordType::A) {
        Ok(res) => {res}
        Err(e) => {
            println!("name错误：{}, e: {:?}", host,e);
            return ips;
        }
    };

// Messages are the packets sent between client and server in DNS.
//  there are many fields to a Message, DnsResponse can be dereferenced into
//  a Message. It's beyond the scope of these examples
//  to explain all the details of a Message. See trust_dns_client::op::message::Message for more details.
//  generally we will be interested in the Message::answers
    let answers: &[Record] = response.answers();
    
    // println!("answers[0]  {:#?}", answers);
//    if answers.len()>0 {
//        println!("answers[1]  {:#?}",answers[1]);
//    }


// Records are generic objects which can contain any data.
//  In order to access it we need to first check what type of record it is
//  In this case we are interested in A, IPv4 address
    for v in answers.iter() {
        match v.rdata() {
            &RData::A(addr) => {
                println!("ip  {:#?}", addr);
                ips.push(IpAddr::from(addr));
                // return Some(IpAddr::from(addr));
            }
            &RData::AAAA(addr) => {
                println!("ip  {:#?}", addr);
                // return Some(IpAddr::from(addr));
            }
            &RData::ANAME(_) => {}
            &RData::CAA(_) => {}
            &RData::CNAME(_) => {}
            &RData::MX(_) => {}
            &RData::NAPTR(_) => {}
            &RData::NULL(_) => {}
            &RData::NS(_) => {}
            &RData::OPENPGPKEY(_) => {}
            &RData::OPT(_) => {}
            &RData::PTR(_) => {}
            &RData::SOA(_) => {}
            &RData::SRV(_) => {}
            &RData::SSHFP(_) => {}
            &RData::TLSA(_) => {}
            &RData::TXT(_) => {}
            &RData::DNSSEC(_) => {}
            &RData::Unknown { code: _, rdata: _ } => {}
            &RData::ZERO => {}
        }
    }
    
    ips
}