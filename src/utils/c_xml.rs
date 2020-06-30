use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::string::ToString;
use xml::writer::events;

/// 将`xml`数据解析成`BTreeMap`
pub fn from_xml_str(data: &str) -> BTreeMap<String, String> {
    let mut pairs = BTreeMap::new();

    let reader = xml::reader::EventReader::from_str(data);
    let mut tag: String = "".to_owned();
    for event in reader {
        match event {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                tag = name.local_name;
            }
            Ok(xml::reader::XmlEvent::CData(value)) => {
                pairs.insert(tag.clone(), value);
            }
            Err(e) => {
                println!("Parse xml error: {:?}", e);
                break;
            }
            _ => {}
        }
    }
    //    dbg!(&pairs);  //todo
    pairs
}

/// 使用`BTreeMap`生成`xml`数据
pub fn to_xml_str(pairs: &BTreeMap<String, String>) -> String {
    let mut target: Vec<u8> = Vec::new();
    {
        let mut writer = xml::writer::EmitterConfig::new()
            .write_document_declaration(false)
            .create_writer(&mut target);
        let _ = writer.write::<events::XmlEvent>(events::XmlEvent::start_element("xml").into());
        for (key, value) in pairs {
            let _ = writer
                .write::<events::XmlEvent>(events::XmlEvent::start_element(key.as_ref()).into());
            let _ = writer
                .write::<events::XmlEvent>(events::XmlEvent::characters(value.as_ref()).into());
            let _ = writer.write::<events::XmlEvent>(events::XmlEvent::end_element().into());
        }
        let _ = writer.write::<events::XmlEvent>(events::XmlEvent::end_element().into());
    }
    String::from_utf8(target).unwrap()
}
