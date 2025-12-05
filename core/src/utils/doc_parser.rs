use godot::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct NebulaDocParser {
    base: Base<Object>,
}

#[godot_api]
impl IObject for NebulaDocParser {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl NebulaDocParser {
    #[func]
    fn parse(input_text: GString) -> GString {
        let input = input_text.to_string();
        let sectioned_tags = Self::get_sectioned_tag_configs();
        let inline_tags = Self::get_inline_tag_configs();
        
        let grouped = Self::extract_and_group_tags(&input, &sectioned_tags);
        let output_without_tags = Self::remove_tag_lines(&input, &sectioned_tags);
        let appended_sections = Self::build_tag_sections(&grouped, &sectioned_tags);
        
        let mut output = output_without_tags;
        if !output.is_empty() && !appended_sections.is_empty() {
            output.push_str("\n\n");
        }
        output.push_str(&appended_sections);

        output = Self::apply_inline_tags(&output, &inline_tags);

        GString::from(output.trim())
    }

    fn get_sectioned_tag_configs() -> Vec<SectionedTagConfig> {
        vec![
            SectionedTagConfig {
                tag_name: "prop",
                section_title: "Properties",
                formatter: Self::format_property,
            },
            SectionedTagConfig {
                tag_name: "event",
                section_title: "Events",
                formatter: Self::format_event,
            },
        ]
    }

    fn get_inline_tag_configs() -> Vec<InlineTagConfig> {
        vec![
            InlineTagConfig {
                pattern: r"\[h(?:\s+([^\]]+))?\]\s*(.*)",
                replacer: |captures| {
                    let description = captures.get(2).map_or("", |m| m.as_str()).trim();
                    format!("[font_size=32]{}[/font_size]", description)
                },
            },
            InlineTagConfig {
                pattern: r"\[hr\]",
                replacer: |_| {
                    "[center]——————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————————[/center]".to_string()
                },
            },
            InlineTagConfig {
                pattern: r"\[object\s+([^\]]+?)\]\s*(.*?)(?:\n|$)",
                replacer: |captures| {
                    let attrs_str = captures.get(1).map_or("", |m| m.as_str());
                    let description = captures.get(2).map_or("", |m| m.as_str()).trim();
                    
                    let re = regex::Regex::new(r#"(\w+)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^,\s]+))"#).unwrap();
                    let mut attrs = std::collections::HashMap::new();
                    
                    for cap in re.captures_iter(attrs_str) {
                        let key = cap.get(1).unwrap().as_str().trim();
                        let value = cap.get(2)
                            .or(cap.get(3))
                            .or(cap.get(4))
                            .map(|m| m.as_str().trim())
                            .unwrap_or("");
                        attrs.insert(key, value);
                    }
                    
                    let name = attrs.get("name").unwrap_or(&"Unknown");
                    let class = attrs.get("class").unwrap_or(&"");
                    
                    format!(
                        "[font_size=40]{}[/font_size][br][desc opacity=0.5][font_size=14][i]{}[/i][/font_size][/desc][br][br][font_size=16][desc opacity=0.5]{}[/desc][/font_size]\n",
                        name, class, description
                    )
                },
            }
        ]
    }

    fn remove_tag_lines(input: &str, configs: &[SectionedTagConfig]) -> String {
        let mut output = String::new();
        let mut skip_next_description = false;

        for line in input.lines() {
            let trimmed = line.trim();

            if skip_next_description {
                if trimmed.is_empty() || trimmed.starts_with('[') {
                    skip_next_description = false;
                    if !trimmed.is_empty() {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if Self::is_sectioned_tag_line(trimmed, configs) {
                skip_next_description = true;
                continue;
            }

            output.push_str(line);
            output.push('\n');
        }

        output.trim_end().to_string()
    }

    fn is_sectioned_tag_line(line: &str, configs: &[SectionedTagConfig]) -> bool {
        configs.iter().any(|config| {
            line.starts_with(&format!("[{} ", config.tag_name))
        })
    }

    fn extract_and_group_tags(input: &str, configs: &[SectionedTagConfig]) -> HashMap<String, Vec<TagData>> {
        let mut grouped: HashMap<String, Vec<TagData>> = HashMap::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let mut matched = false;

            for config in configs {
                if let Some((attributes, inline_desc)) = Self::parse_tag_line(line, config.tag_name) {
                    let description = if inline_desc.is_empty() {
                        Self::extract_multiline_description(&lines, &mut i, configs)
                    } else {
                        i += 1;
                        inline_desc
                    };

                    grouped
                        .entry(config.tag_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(TagData {
                            attributes,
                            description,
                        });
                    
                    matched = true;
                    break;
                }
            }

            if !matched {
                i += 1;
            }
        }

        grouped
    }

    fn extract_multiline_description(lines: &[&str], index: &mut usize, configs: &[SectionedTagConfig]) -> String {
        let mut description = String::new();
        *index += 1;

        while *index < lines.len() {
            let desc_line = lines[*index].trim();
            
            if Self::is_sectioned_tag_line(desc_line, configs) {
                break;
            }

            if !desc_line.is_empty() {
                if !description.is_empty() {
                    description.push(' ');
                }
                description.push_str(desc_line);
            }
            *index += 1;
        }

        description
    }

    fn parse_tag_line(line: &str, tag_type: &str) -> Option<(String, String)> {
        let pattern = format!(r"\[{}\s+([^\]]+)\]\s*(.*)", tag_type);
        let re = Regex::new(&pattern).ok()?;

        re.captures(line).map(|cap| {
            let attributes = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let inline_desc = cap.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            (attributes, inline_desc)
        })
    }

    fn build_tag_sections(grouped: &HashMap<String, Vec<TagData>>, configs: &[SectionedTagConfig]) -> String {
        let mut output = String::new();

        output.push_str("[color=#46474C][hr][/color]\n\n");

        for config in configs {
            if let Some(tags) = grouped.get(config.tag_name) {
                if !tags.is_empty() {
                    output.push_str(&format!("[h] {}\n\n", config.section_title));
                    
                    for tag in tags {
                        output.push_str(&(config.formatter)(tag));
                        output.push('\n');
                    }
                    output.push('\n');
                }
            }
        }

        output
    }

    fn apply_inline_tags(input: &str, configs: &[InlineTagConfig]) -> String {
        let mut result = input.to_string();

        for config in configs {
            let re = Regex::new(config.pattern).unwrap();
            let mut replacements = Vec::new();

            for cap in re.captures_iter(&result) {
                let full_match = cap.get(0).unwrap().as_str();
                let replacement = (config.replacer)(&cap);
                replacements.push((full_match.to_string(), replacement));
            }

            for (original, replacement) in replacements {
                result = result.replace(&original, &replacement);
            }
        }

        result
    }

    fn format_property(tag: &TagData) -> String {
        let attrs = Self::parse_attributes(&tag.attributes);

        format!(
            "\t\t{}: [code]{}[/code] = [code]{}[/code]\n\t\t\t[font_size=13][desc opacity=0.5] • {}[/desc][/font_size]",
            attrs.get("name").unwrap_or(&"unknown".to_string()),
            attrs.get("type").unwrap_or(&"var".to_string()),
            attrs.get("default").unwrap_or(&"null".to_string()),
            tag.description
        )
    }

    fn format_event(tag: &TagData) -> String {
        let attrs = Self::parse_attributes(&tag.attributes);

        format!(
            "\t\t{}[font_size=13][desc opacity=0.5]: {}[/desc][/font_size]",
            attrs.get("name").unwrap_or(&"unknown".to_string()),
            tag.description
        )
    }

    fn parse_attributes(attr_string: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let re = Regex::new(r#"(\w+)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^,\s]+))"#).unwrap();

        for cap in re.captures_iter(attr_string) {
            let key = cap.get(1).unwrap().as_str().trim().to_string();
            let value = cap.get(2)
                .or(cap.get(3))
                .or(cap.get(4))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            result.insert(key, value);
        }

        result
    }
}

struct SectionedTagConfig {
    tag_name: &'static str,
    section_title: &'static str,
    formatter: fn(&TagData) -> String,
}

struct InlineTagConfig {
    pattern: &'static str,
    replacer: fn(&regex::Captures) -> String,
}

struct TagData {
    attributes: String,
    description: String,
}
