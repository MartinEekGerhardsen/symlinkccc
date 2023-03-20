use quick_xml::events::Event;

pub fn package_name_parser(input: &str) -> Option<String> {
    let mut reader = quick_xml::reader::Reader::from_str(input);

    reader.trim_text(true);

    let mut within_name_tag = false;
    let mut package_name: Option<String> = None;

    loop {
        match reader.read_event() {
            Err(e) => {
                log::warn!("Error at position {}: {:?}", reader.buffer_position(), e);
            }
            Ok(Event::Eof) => {
                log::debug!("Finished parsing file");
                break;
            }
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"name" {
                    within_name_tag = true;
                }
            }
            Ok(Event::Text(e)) => {
                if within_name_tag {
                    match e.unescape() {
                        Ok(text) => {
                            package_name = Some(text.to_string());
                        }
                        Err(_) => {
                            package_name = None;
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"name" {
                    break;
                }
            }
            Ok(_) => {}
        }
    }

    package_name
}

#[cfg(test)]
mod tests {
    use quick_xml::{events::Event, Reader};

    use crate::parsers::package::package_name_parser;

    #[test]
    fn test_package() {
        let pack = "
<?xml version=\"1.0\"?>
<package format=\"3\">
  <name>ouster_ros</name>
  <version>0.7.2</version>
  <description>Ouster ROS driver</description>
  <maintainer email=\"oss@ouster.io\">ouster developers</maintainer>
  <license file=\"LICENSE\">BSD</license>
  <buildtool_depend>catkin</buildtool_depend>
  <depend>roscpp</depend>
  <depend>std_msgs</depend>
  <depend>sensor_msgs</depend>
  <depend>geometry_msgs</depend>
  <depend>tf2_ros</depend>
  <depend>pcl_ros</depend>
  <depend>pcl_conversions</depend>
  <build_depend>boost</build_depend>
  <build_depend>nodelet</build_depend>
  <build_depend>libjsoncpp-dev</build_depend>
  <build_depend>eigen</build_depend>
  <build_depend>message_generation</build_depend>
  <build_depend>tf2_eigen</build_depend>
  <build_depend>libpcl-all-dev</build_depend>
  <build_depend>curl</build_depend>
  <build_depend>spdlog</build_depend>
  <exec_depend>nodelet</exec_depend>
  <exec_depend>libjsoncpp</exec_depend>
  <exec_depend>message_runtime</exec_depend>
  <exec_depend>topic_tools</exec_depend>
  <exec_depend>curl</exec_depend>
  <exec_depend>spdlog</exec_depend>
  <test_depend>gtest</test_depend>
  <export>
    <nodelet plugin=\"${prefix}/nodelets_os.xml\"/>
  </export>
</package>
            ";
        let package_name = package_name_parser(pack);
        assert_eq!(package_name, Some("ouster_ros".to_string()));
    }

    #[test]
    fn test_quickxml() {
        let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test</tag2>
                <tag2>Test 2</tag2>
             </tag1>"#;
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut count = 0;
        let mut txt = Vec::new();
        let mut buf = Vec::new();
        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"tag1" => println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    ),
                    b"tag2" => count += 1,
                    _ => (),
                },
                Ok(Event::Text(e)) => {
                    txt.push(e.unescape().unwrap().into_owned());
                }

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        println!("{count}");
    }
}
