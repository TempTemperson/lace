use std::collections::HashMap;

use log::debug;
use mdbook::{preprocess::{Preprocessor, PreprocessorContext}, book::Book, BookItem};
use pulldown_cmark::{Parser, Event, CodeBlockKind, Tag};
use regex::Regex;
use anyhow::anyhow;

use serde::Deserialize;

type GammaMap = HashMap<String, lace_stats::rv::dist::Gamma>;

fn check_deserialize<T>(input: &str) -> anyhow::Result<()>
where
    T: for<'a> Deserialize<'a>,
{
    debug!("attempting to deserialize to {}", core::any::type_name::<T>());
    serde_yaml::from_str::<T>(input)?;
    Ok(())
}

macro_rules! check_deserialize_arm {
    ($input:expr, $name:expr, [$($types:ty),*]) => {
        match $name {
            $(stringify!($types) => check_deserialize::<$types>($input),)*
            t =>  Err(anyhow!("unrecognized type to deserialize to: {t}")),
        }
    }
}
    
fn check_deserialize_dyn(input: &str, type_name: &str) -> anyhow::Result<()> {
    check_deserialize_arm!(input, type_name, [GammaMap, lace_codebook::ColMetadataList])
}

/// A Preprocessor for testing YAML code blocks
pub struct YamlTester;

impl YamlTester {
    pub fn new() -> YamlTester {
        YamlTester
    }

    fn examine_chapter_content(&self, content: &str, re: &Regex) -> anyhow::Result<()> {
        let parser = Parser::new(content);
        let mut code_block = Some(String::new());
        
        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref code_block_string))) => {
                    if re.is_match(&code_block_string) {
                        debug!("YAML Block Start, identifier string={}", code_block_string);
                        code_block=Some(String::new());    
                    }
                },
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref code_block_string))) => {
                    if let Some(captures) = re.captures(&code_block_string) {
                        debug!("Code Block End, identifier string={}", code_block_string);

                        let target_type = captures.get(1).ok_or(anyhow!("No deserialize type found"))?.as_str();
                        debug!("Target deserialization type is {}", target_type);

                        let final_block = code_block.take();
                        let final_block = final_block.ok_or(anyhow!("No YAML text found"))?;
                        debug!("Code block ended up as\n{}", final_block);
                        
                        check_deserialize_dyn(&final_block, target_type)?;                            
                    }
                },
                Event::Text(ref text) => {
                    if let Some(existing) = code_block.as_mut() {
                        existing.push_str(text);
                    }
                },
                _ => ()
            }
        }
        
        Ok(())
    }
}

impl Preprocessor for YamlTester {
    fn name(&self) -> &str {
        "lace-yaml-tester"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> anyhow::Result<Book> {
        debug!("Starting the run");
        let re = Regex::new(r"^yaml.*,deserializeTo=([^,]+)").unwrap();
        for book_item in book.iter() {
            if let BookItem::Chapter(chapter) = book_item {
                debug!("Examining Chapter {}", chapter.name);
                self.examine_chapter_content(&chapter.content, &re)?;
            }
        }

        Ok(book)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dummy() {
        assert!(True);
    }
}
