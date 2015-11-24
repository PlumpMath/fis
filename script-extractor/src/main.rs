extern crate xml;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug, Clone, Default)]
struct LineAttributes {
    top: i32,
    left: i32,
    height: i32,
}

fn read_attributes(attr_list: &Vec<OwnedAttribute>) -> LineAttributes {
    let mut attributes : LineAttributes = Default::default();

    for attr in attr_list {
        match attr.name.local_name.as_ref() {
            "top" => attributes.top = attr.value.parse().unwrap(),
            "left" => attributes.left = attr.value.parse().unwrap(),
            "height" => attributes.height = attr.value.parse().unwrap(),
            _ => {}
        }
    }

    attributes
}

#[derive(Debug, Clone, Default)]
struct ScriptProperties {
    direction_position: i32,
    dialog_position: i32,
    speaker_direction_position: i32,
    speaker_position: i32,
    intra_paragraph_line_height: i32,
}

fn read_and_analyze_script(reader: Box<Read>) -> (ScriptProperties, Vec<(LineAttributes, String)>) {
    let mut script_properties: ScriptProperties = Default::default();
    let mut lines: Vec<(LineAttributes, String)> = Vec::new();

    // these maps are used to heuristically determine the ScriptProperties
    let mut position_uses: HashMap<i32, u32> = HashMap::new();
    let mut line_height_uses: HashMap<i32, u32> = HashMap::new();

    // states for the streaming xml parsing
    let mut current_line_attributes: LineAttributes = Default::default();
    let mut current_text_buffer = String::new();
    let mut last_line_height = 0;

    let parser = EventReader::new(reader);
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_ref() {
                    "text" => {
                        current_line_attributes = read_attributes(&attributes);

                        current_text_buffer.clear();

                        // increase the count for the current left margin
                        *position_uses.entry(current_line_attributes.left)
                                      .or_insert(0) += 1;

                        // increase the count for the current top position diff
                        *line_height_uses.entry(current_line_attributes.top - last_line_height)
                                         .or_insert(0) += 1;
                        last_line_height = current_line_attributes.top;
                    }
                    "page" => {
                        last_line_height = 0;
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                if current_text_buffer.len() > 0 {
                    current_text_buffer.push(' ');
                }
                current_text_buffer.push_str(text.trim());
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name == "text" {
                    lines.push((current_line_attributes.clone(), current_text_buffer.clone()));
                }
            }
            Ok(_) => {},
            Err(e) => panic!("Error parsing xml: {}", e),
        }
    }

    // the position_uses map should at least have 3 different entries
    if position_uses.len() < 3 {
        panic!("Script uses strange layout");
    }

    // copy the position uses map into a vector and sort by value desc
    let mut position_uses_vec = position_uses.into_iter().collect::<Vec<_>>();
    position_uses_vec.sort_by(|&a, &b| b.1.cmp(&a.1));

    // sort the top 3 entries by position again (right-exclusive range)
    position_uses_vec[0..3].sort_by(|&a, &b| b.0.cmp(&a.0));

    script_properties.speaker_position = position_uses_vec[0].0;
    script_properties.dialog_position = position_uses_vec[1].0;
    script_properties.direction_position = position_uses_vec[2].0;

    // take the next value between speaker and dialog as speaker_direction
    for &value in position_uses_vec.iter().skip(3) {
        let (position, _) = value;
        if position > script_properties.dialog_position &&
           position < script_properties.speaker_position {
            script_properties.speaker_direction_position = position;
            break;
        }
    }

    // use to most used line diff value to determine the specific sections
    let mut last_uses: u32 = 0;
    for (&line_height, &uses) in line_height_uses.iter() {
        if uses > last_uses {
            script_properties.intra_paragraph_line_height = line_height;
            last_uses = uses;
        }
    }

    (script_properties, lines)
}

#[derive(Debug, Clone)]
enum ScenePart {
    Direction {
        direction: String,
    },
    Dialog {
        speaker: String,
        direction: String,
        dialog: String,
    }
}

#[derive(Debug, Clone)]
enum ScriptPart {
    Separator,
    ScenePart(ScenePart),
    LocationChange(String),
    SceneChange,
    PageNumber(i32),
}

fn is_location_change(line: &str) -> bool {
    line.starts_with("INT.") || line.starts_with("EXT.")
}

fn is_scene_change(line: &str) -> bool {
    line.starts_with("CUT TO")
}

fn extract_script_parts(properties: ScriptProperties, lines: &Vec<(LineAttributes, String)>)
    -> Vec<ScriptPart> {
    let mut script_parts: Vec<ScriptPart> = Vec::new();
    let mut last_top_position = 0;
    let is_page_number = regex::Regex::new(r"^[0-9]+\s*\.*$").unwrap();

    for line in lines.iter() {
        let &(ref attributes, ref line) = line;

        if line.len() == 0 {
            continue;
        }

        // check if a new section starts
        if attributes.top - last_top_position > 18 ||
           attributes.top - last_top_position < 0 {
            // used to separate two consecutive script parts of the
            // same type. this is just a implementation detail of
            // the parsing. the Separator can be ignored later.
            script_parts.push(ScriptPart::Separator);
        }

        if attributes.left == properties.direction_position {
            if is_location_change(line) {
                script_parts.push(ScriptPart::LocationChange(line.clone()));
            } else {
                // ensure the last script part is a direction
                if let Some(&ScriptPart::ScenePart(ScenePart::Direction{..})) = script_parts.last() {
                } else {
                    script_parts.push(ScriptPart::ScenePart(
                        ScenePart::Direction{
                            direction: String::new()}));
                }

                if let Some(&mut ScriptPart::ScenePart(ScenePart::Direction{ref mut direction})) = script_parts.last_mut() {
                    if direction.len() > 0 {
                        direction.push(' ');
                    }
                    direction.push_str(line);
                }
            }
        } else if attributes.left == properties.speaker_position ||
                  attributes.left == properties.speaker_direction_position ||
                  attributes.left == properties.dialog_position {
            // Ensure the last script part is a dialog
            if let Some(&ScriptPart::ScenePart(ScenePart::Dialog{..})) = script_parts.last() {
            } else {
                script_parts.push(ScriptPart::ScenePart(
                    ScenePart::Dialog{
                        speaker: String::new(),
                        direction: String::new(),
                        dialog: String::new()}));
            }

            // get the dialog, should never fail (see above)
            if let Some(&mut ScriptPart::ScenePart(ScenePart::Dialog{ref mut speaker, ref mut direction, ref mut dialog})) = script_parts.last_mut() {
                if attributes.left == properties.speaker_position {
                    // there is only one speaker per dialog
                    speaker.push_str(line);
                } else if attributes.left == properties.speaker_direction_position {
                    if direction.len() > 0 {
                        direction.push(' ');
                    }
                    direction.push_str(line);
                } else if attributes.left == properties.dialog_position {
                    if dialog.len() > 0 {
                        dialog.push(' ');
                    }
                    dialog.push_str(line);
                }
            }
        } else {
            if is_scene_change(line) {
                script_parts.push(ScriptPart::SceneChange);
            } else if is_page_number.is_match(line) {
                // TODO: parse actual page number
                script_parts.push(ScriptPart::PageNumber(0));
            }
        }

        last_top_position = attributes.top;
    }

    script_parts
}

fn condense_script_demo(properties: ScriptProperties, lines: &Vec<(LineAttributes, String)>) {
    let mut last_left_position = 0;
    let mut last_top_position = 0;

    for line in lines.iter() {
        let &(ref attributes, ref line) = line;

        if line.len() == 0 {
            continue;
        }

        // start new section
        if last_left_position != attributes.left ||
           attributes.top - last_top_position > 18 ||
           attributes.top - last_top_position < 0 {
            if ! (last_left_position == properties.speaker_direction_position ||
                  last_left_position == properties.speaker_position) {
                // empty line after last section
                println!("");
            }
        }

        match attributes.left {
            k if (k == properties.direction_position)
                => println!("{}", line),
            k if (k == properties.dialog_position)
                => println!("          {}", line),
            k if (k == properties.speaker_direction_position)
                => println!("               {}", line),
            k if (k == properties.speaker_position)
                => println!("                    {}", line),
            _   => println!("------- {}", line),
        }

        last_left_position = attributes.left;
        last_top_position = attributes.top;
    }
}

fn main() {
    let xml_file_name = env::args().skip(1).next().expect("No input file given");

    let file_reader = File::open(xml_file_name).expect("Could not open file");
    let buffered_file_reader = Box::new(BufReader::new(file_reader));

    let (properties, lines) = read_and_analyze_script(buffered_file_reader);

    condense_script_demo(properties, &lines);
    //println!("{:?}", extract_script_parts(properties, &lines));
}
