use masonry::widget::prelude::*;
use masonry::widget::Flex;
use masonry::widget::Label;
use roc_std::RocList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Elem {
    FlexCol(Vec<Vec<Elem>>),
    FlexRow(Vec<Vec<Elem>>),
    Label(Vec<LabelConfig>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelConfig {
    text: String,
}

#[allow(dead_code)]
pub fn from_json_to_masonry(bytes: RocList<u8>) -> impl Widget {
    // println!("{}", String::from_utf8_lossy(&bytes[..]));

    let decoded: Elem = serde_json::from_slice(&bytes[..]).expect("JSON was not well-formatted");

    from_json_to_masonry_help(decoded)
}

pub fn from_json_to_masonry_help<'a>(elem: Elem) -> Box<dyn Widget> {
    match elem {
        Elem::FlexCol(children) => {
            let mut col = Flex::column();

            for roc_child in flatten(children) {
                col = col.with_child(from_json_to_masonry_help(roc_child));
            }

            Box::new(col)
        }
        Elem::FlexRow(children) => {
            let mut col = Flex::row();

            for roc_child in flatten(children) {
                col = col.with_child(from_json_to_masonry_help(roc_child));
            }

            Box::new(col)
        }
        Elem::Label(tag_payloads) => {
            let config = tag_payloads.first().unwrap();

            Box::new(Label::new(config.text.to_owned()).with_text_size(32.0))
        }
    }
}

fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}
