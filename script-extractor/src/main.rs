#[macro_use]
extern crate clap;
extern crate regex;
extern crate xml;

pub mod parse;
pub mod serialize;

use clap::{App, Arg};
use parse::Script;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args = App::new("script-extractor")
                   .version(&crate_version!())
                   .about("Parse movie scripts into a structured format")
                   .after_help("Input has to be in the format generated by \
                                poppler's 'pdftohtml -xml'.\n\
                                Reads from stdin if no file or '-' is specified.")
                   .arg(Arg::with_name("input-file")
                            .help("input file in poppler's xml format")
                            .index(1)
                            .validator(check_file_exists))
                   .arg(Arg::with_name("pages")
                            .help("Specify a single page or range of pages to extract")
                            .short("p")
                            .long("pages")
                            .takes_value(true)
                            .validator(|v| if extract_range(&v).is_some() {
                                               Ok(())
                                           } else {
                                               Err(format!("Invalid page range '{}'", v))
                                           }))
                   .get_matches();

    let mut input: Box<Read> = if let Some(input_file) = args.value_of("input-file") {
        if input_file == "-" {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            let file_reader = File::open(input_file).expect("Cannot open input-file");
            Box::new(BufReader::new(file_reader))
        }
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    let mut script = parse::parse_script(&mut input);

    // filter by pages if requested
    if let Some(range_string) = args.value_of("pages") {
        let range = extract_range(range_string).unwrap_or((0,u32::max_value()));
        script = filter_script(script, range);
    }

    serialize::xml::format_script(&script, &mut std::io::stdout()).unwrap();
}

/// Check if the given file exists. Also considers "-" as a valid file.
fn check_file_exists(file_name: String) -> Result<(), String> {
    if file_name == "-" {
        Ok(())
    } else if let Ok(metadata) = std::fs::metadata(&file_name) {
        if metadata.is_file() && !metadata.permissions().readonly() {
            Ok(())
        } else {
            Err(format!("Cannot read file '{}'", file_name))
        }
    } else {
        Err(format!("File '{}' not found", file_name))
    }
}

/// Parses the given string into a range.
///
/// This is used when parsing the "--pages" cli argument.
///
/// # Examples
///
/// ```
/// assert_eq!(extract_range("42"), (42, 42));
/// assert_eq!(extract_range("3-15"), (3, 15));
/// assert_eq!(extract_range("foo"), (0, u32::max_value()));
/// ```
pub fn extract_range(range_string: &str) -> Option<(u32, u32)> {
    let range_regex = Regex::new(r"^(?P<lower>\d+)-(?P<upper>\d+)$").unwrap();

    if let Ok(page) = range_string.parse() {
        return Some((page, page));
    } else if let Some(captures) = range_regex.captures(range_string) {
        if let (Ok(lower), Ok(upper)) = (captures.name("lower").unwrap().parse(),
                                         captures.name("upper").unwrap().parse()) {
            return Some((lower, upper));
        }
    }

    None
}

/// Filter the script using a range of pages.
///
/// The range is inclusive and empty scenes and locations are removed.
pub fn filter_script(script: Script, page_range: (u32, u32)) -> Script {
    use parse::{Scene, ScenePart};

    let (lower, upper) = page_range;

    script.into_iter().filter_map(|scene| {
        let filtered_scene: Scene = scene.into_iter().filter_map(|mut location| {
            let filtered_scene_parts: Vec<ScenePart> = location.parts.into_iter().filter_map(|scene_part| {
                let page = match scene_part {
                    ScenePart::Direction { page, .. } => page,
                    ScenePart::Dialog { page, .. } => page
                };

                // filter using the given range
                if lower <= page && page <= upper {
                    Some(scene_part)
                } else {
                    None
                }
            }).collect();

            // filter out locations with no scene parts
            if filtered_scene_parts.len() > 0 {
                location.parts = filtered_scene_parts;
                Some(location)
            } else {
                None
            }
        }).collect();

        // filter out scenes with no locations
        if filtered_scene.len() > 0 {
            Some(filtered_scene)
        } else {
            None
        }
    }).collect()
}
