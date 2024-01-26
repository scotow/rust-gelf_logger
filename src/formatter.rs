// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
// Copyright 2009 The gelf_logger Authors. All rights reserved.

use serde_gelf::{GelfRecord};
use serde_json::{Map, Value};

use crate::config::Config;
use crate::result::Result;

#[derive(Clone, Debug)]
pub struct GelfFormatter {
    additional_fields: Map<String, Value>,
    null_character: bool,
}

impl GelfFormatter {
    pub fn new(null_character: bool, additional_fields: Map<String, Value>) -> GelfFormatter {
        GelfFormatter {
            null_character,
            additional_fields,
            // additional_fields: match to_flatten_maptree("_",Some("_"), &additional_fields) {
            //     Err(_) => BTreeMap::new(),
            //     Ok(values) => values
            // },
        }
    }
    fn default_additional_fields(&self) -> &Map<String, Value> {
        &self.additional_fields
    }

    fn null_character(&self) -> &bool {
        &self.null_character
    }

    pub fn format(&self, _record: &GelfRecord) -> Result<String> {
        dbg!(self.default_additional_fields(), self.null_character());
        // let rec = record.clone()
        //     .extend_additional_fields(self.default_additional_fields().clone());
        // Ok(match self.null_character() {
        //     &true => format!("{}\n\0", serde_json::to_string(&rec)?),
        //     &false => format!("{}\n", serde_json::to_string(&rec)?)
        // })
        todo!()
    }
}

impl From<&Config> for GelfFormatter {
    fn from(cfg: &Config) -> GelfFormatter {
        GelfFormatter::new(cfg.null_character().clone(), cfg.additional_fields().clone())
    }
}