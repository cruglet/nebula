use godot::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct DocParser {
    base: Base<Object>,
}

#[godot_api]
impl IObject for DocParser {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl DocParser {
    /// Transform a string with custom tags to BBCode formatted string
    /// 
    /// Supported tags:
    /// - [prop name=x, type=y, default=z] description
    /// - [event name=x] description
    /// 
    /// Example:
    /// ```gdscript
    /// var formatted = DocParser.parse("[prop name=ex_prop, type=bool, default=false] This is an example property")
    /// ```
    #[func]
    fn parse(input_text: GString) -> GString {
        let input = input_text.to_string();
        
        let grouped = Self::extract_and_group_tags(&input);
        
        let mut result = String::new();
        
        if !grouped.properties.is_empty() {
            result.push_str("[h] Properties\n\n");
            for prop in &grouped.properties {
                result.push_str(&Self::parse_prop_tag(prop));
                result.push('\n');
            }
        }
        
        if !grouped.events.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("[h] Events\n\n");
            for event in &grouped.events {
                result.push_str(&Self::parse_event_tag(event));
                result.push('\n');
            }
        }
        
        result = Self::parse_header_tags(&result);
        result = Self::parse_line_tags(&result);
        
        GString::from(result.trim())
    }
    
    fn extract_and_group_tags(input: &str) -> GroupedTags {
        let mut grouped = GroupedTags {
            properties: Vec::new(),
            events: Vec::new(),
        };
        
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i];
            
            if let Some((attributes, inline_desc)) = Self::parse_tag_line(line, "prop") {
                let mut description = inline_desc;
                
                if description.is_empty() {
                    i += 1;
                    while i < lines.len() {
                        let desc_line = lines[i].trim();
                        if desc_line.starts_with('[') && 
                           (desc_line.starts_with("[prop") || 
                            desc_line.starts_with("[event") || 
                            desc_line.starts_with("[h")) {
                            break;
                        }
                        if !desc_line.is_empty() {
                            if !description.is_empty() {
                                description.push(' ');
                            }
                            description.push_str(desc_line);
                        }
                        i += 1;
                    }
                } else {
                    i += 1;
                }
                
                grouped.properties.push(TagData {
                    attributes,
                    description,
                });
                continue;
            }
            
            if let Some((attributes, inline_desc)) = Self::parse_tag_line(line, "event") {
                let mut description = inline_desc;
                
                if description.is_empty() {
                    i += 1;
                    while i < lines.len() {
                        let desc_line = lines[i].trim();
                        if desc_line.starts_with('[') && 
                           (desc_line.starts_with("[prop") || 
                            desc_line.starts_with("[event") || 
                            desc_line.starts_with("[h")) {
                            break;
                        }
                        if !desc_line.is_empty() {
                            if !description.is_empty() {
                                description.push(' ');
                            }
                            description.push_str(desc_line);
                        }
                        i += 1;
                    }
                } else {
                    i += 1;
                }
                
                grouped.events.push(TagData {
                    attributes,
                    description,
                });
                continue;
            }
            
            i += 1;
        }
        
        grouped
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
    
    fn parse_prop_tag(tag: &TagData) -> String {
        let prop_data = Self::parse_attributes(&tag.attributes);
        
        format!(
            "\t\t{}: [code]{}[/code] = [code]{}[/code]\n\t\t\t[font_size=13][desc opacity=0.5] • {}[/desc][/font_size]",
            prop_data.get("name").unwrap_or(&"unknown".to_string()),
            prop_data.get("type").unwrap_or(&"var".to_string()),
            prop_data.get("default").unwrap_or(&"null".to_string()),
            tag.description
        )
    }
    
    fn parse_event_tag(tag: &TagData) -> String {
        let event_data = Self::parse_attributes(&tag.attributes);
        
        format!(
            "\t\t{}[font_size=13][desc opacity=0.5]: {}[/desc][/font_size]\n",
            event_data.get("name").unwrap_or(&"unknown".to_string()),
            tag.description
        )
    }
    
    fn parse_header_tags(input: &str) -> String {
        let re = Regex::new(r"\[h(?:\s+([^\]]+))?\]\s*(.*)").unwrap();
        let mut result = input.to_string();
        
        for cap in re.captures_iter(input) {
            let full_match = cap.get(0).unwrap().as_str();
            let description = cap.get(2).map_or("", |m| m.as_str()).trim();
            
            let formatted = format!(
                "[font_size=32]{}[/font_size]",
                description
            );
            
            result = result.replace(full_match, &formatted);
        }
        
        result
    }
    
    fn parse_line_tags(input: &str) -> String {
        let hr_re = Regex::new(r"\[hr\]").unwrap();
        let hr_line = "[center]—————————————————————————————————————————————————————————————————————————————————————————————————————————————[/center]";
        hr_re.replace_all(&input, hr_line).to_string()
    }
    
    fn parse_attributes(attr_string: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let re = Regex::new(r"(\w+)\s*=\s*([^,]+)").unwrap();
        
        for cap in re.captures_iter(attr_string) {
            let key = cap.get(1).unwrap().as_str().trim().to_string();
            let value = cap.get(2).unwrap().as_str().trim().to_string();
            result.insert(key, value);
        }
        
        result
    }
}

struct GroupedTags {
    properties: Vec<TagData>,
    events: Vec<TagData>,
}

struct TagData {
    attributes: String,
    description: String,
}