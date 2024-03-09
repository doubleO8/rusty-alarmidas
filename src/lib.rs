use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod machine_state;
use machine_state::lookup_by_name;
use machine_state::MachineState;
use machine_state::MACHINE_STATE_KEYS;

static LP: &str = "[RUSTY]";

#[wasm_bindgen]
pub struct NodeData {
    rpi_serial: String,
    cls: HashSet<String>,
    machine_state: String,
}

impl NodeData {
    fn new(rpi_serial: &str, cls: HashSet<String>) -> NodeData {
        NodeData {
            rpi_serial: rpi_serial.to_string(),
            cls,
            machine_state: String::from("unknown"),
        }
    }
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.rpi_serial, self.machine_state);

        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct BirdsEyeViewStats {
    data: HashMap<String, HashSet<String>>,
}

impl BirdsEyeViewStats {
    pub fn new(data: HashMap<String, HashSet<String>>) -> BirdsEyeViewStats {
        BirdsEyeViewStats { data }
    }

    pub fn json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.data).unwrap()
    }
}

#[wasm_bindgen]
pub struct BirdsEyeView {
    rows: u32,
    width: u32,
    height: u32,
    items_per_row: u32,
    items_dimension: u32,
    items_margin: u32,
    padding: u32,
    data: HashMap<String, NodeData>,
    stats: BirdsEyeViewStats,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct BirdsEyeViewNode {
    x: u32,
    y: u32,
    cls: HashSet<String>,
    rpi_serial: String,
    machine_state: String,
}

impl BirdsEyeViewNode {
    pub fn set_coordinates(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

#[wasm_bindgen]
impl BirdsEyeView {
    pub fn new() -> BirdsEyeView {
        console::log_1(&format!("{} Creating bird's eye view", LP).into());
        let items = 0;
        let items_per_row = 25;
        let padding = 5;
        let items_dimension = 20;
        let items_margin = 5;
        let data = HashMap::new();
        let stats = BirdsEyeViewStats::new(HashMap::from([
            ("macon_short_uptime".to_string(), HashSet::new()),
            ("no_identity".to_string(), HashSet::new()),
        ]));

        let width =
            padding + items_per_row * (items_margin + items_dimension + items_margin) + padding;

        let mut rows = items / items_per_row;
        if items % items_per_row > 0 {
            rows += 1;
        }

        let height = padding + rows * (items_margin + items_dimension + items_margin) + padding;

        BirdsEyeView {
            rows,
            width,
            height,
            items_per_row,
            items_dimension,
            items_margin,
            padding,
            data,
            stats,
        }
    }

    pub fn register(&mut self, rpi_serial: &str) -> bool {
        if self.data.contains_key(rpi_serial) {
            return false;
        }

        self.data.insert(
            rpi_serial.to_string(),
            NodeData::new(rpi_serial, HashSet::new()),
        );

        let mut rows = self.items() as u32 / self.items_per_row;
        if self.items() as u32 % self.items_per_row > 0 {
            rows += 1;
        }
        let height = self.padding
            + rows * (self.items_margin + self.items_dimension + self.items_margin)
            + self.padding;

        /*
        console::log_1(
            &format!(
                "{} register: rpi_serial={} rows={} height={}",
                LP, rpi_serial, rows, height
            )
            .into(),
        );
         */

        self.rows = rows;
        self.height = height;

        true
    }

    pub fn clear_cls(&mut self, rpi_serial: &str) {
        if self.data.contains_key(rpi_serial) {
            if let Some(x) = self.data.get_mut(rpi_serial) {
                x.cls.clear();
            }
        }
    }

    pub fn add_cls(&mut self, rpi_serial: &str, cls: &str) -> bool {
        let mut result = false;

        /*
                console::log_1(
                    &format!(
                        "{} add_class: .. WOULD LIKE TO .. rpi_serial={} cls={}",
                        LP,
                        rpi_serial.to_string(),
                        cls.to_string()
                    )
                    .into(),
                );
        */
        if self.data.contains_key(rpi_serial) {
            if let Some(x) = self.data.get_mut(rpi_serial) {
                x.cls.insert(cls.to_string());
                result = true;
            }

            match self.stats.data.get_mut(cls) {
                Some(x) => {
                    x.insert(rpi_serial.to_string());
                    /*
                    console::log_1(
                        &format!(
                            "{} STATS rpi_serial={} cls={} --> {}",
                            LP,
                            rpi_serial.to_string(),
                            cls.to_string(),
                            x.len()
                        )
                        .into(),
                    );
                    */
                }
                _ => (),
            }
        }

        result
    }

    pub fn set_machine_state(&mut self, rpi_serial: &str, state_value: &str) -> bool {
        /*
        console::log_1(
            &format!(
                "{} set_machine_state rpi_serial={} state_value={}",
                LP,
                rpi_serial.to_string(),
                state_value.to_string()
            )
            .into(),
        );
        */

        if self.data.contains_key(rpi_serial) {
            if lookup_by_name(state_value) == MachineState::UNKNOWN {
                /*
                console::log_1(
                    &format!(
                        "{} rpi_serial={} lookup failed!",
                        LP,
                        rpi_serial.to_string()
                    )
                    .into(),
                );
                 */
                return false;
            }

            if let Some(x) = self.data.get_mut(rpi_serial) {
                if x.machine_state == state_value.to_string() {
                    return false;
                } else {
                    x.machine_state = state_value.to_string();
                    return true;
                }
            }
        }
        /*
        else {
            console::log_1(
                &format!("{} rpi_serial={} unknown!", LP, rpi_serial.to_string()).into(),
            );
        }
         */
        false
    }

    pub fn machine_state_stats(&self) -> JsValue {
        let mut stats = HashMap::new();
        let mut result: Vec<(String, i32)> = Vec::new();

        for item in self.data.values() {
            let ms: MachineState = lookup_by_name(&item.machine_state.clone());

            if ms != MachineState::UNKNOWN {
                let stat = stats.entry(item.machine_state.clone()).or_insert(0);
                *stat += 1;
                // console::log_1(&format!("{item}: -> {ms}").into());
            }
        }

        for ms in MACHINE_STATE_KEYS {
            let stat = stats.entry(ms.to_string()).or_insert(0);
            result.push((ms.to_string(), *stat))
        }

        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    pub fn nodes(&self) -> JsValue {
        let mut node_items: Vec<BirdsEyeViewNode> = Vec::new();
        let mut node_keys = Vec::from_iter(self.data.keys());

        node_keys.sort();

        for (idx_u, key) in node_keys.iter().enumerate() {
            let idx = idx_u as u32;
            let item = &self.data[*key];
            let row = idx / self.items_per_row;
            let col = idx % self.items_per_row;
            let col_margin = self.items_margin;
            let row_margin = self.items_margin;

            let node = BirdsEyeViewNode {
                x: self.padding
                    + col_margin
                    + col * (self.items_margin + self.items_dimension + self.items_margin),
                y: self.padding
                    + row_margin
                    + row * (self.items_margin + self.items_dimension + self.items_margin),
                cls: item.cls.clone(),
                rpi_serial: item.rpi_serial.clone(),
                machine_state: item.machine_state.clone(),
            };
            node_items.push(node);
        }

        serde_wasm_bindgen::to_value(&node_items).unwrap()
    }

    pub fn change_items_per_row(&mut self, ipr: u32) {
        self.items_per_row = ipr;
        let width = self.padding
            + self.items_per_row * (self.items_margin + self.items_dimension + self.items_margin)
            + self.padding;
        self.width = width;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn items_per_row(&self) -> u32 {
        self.items_per_row
    }

    pub fn items_dimension(&self) -> u32 {
        self.items_dimension
    }

    pub fn items_margin(&self) -> u32 {
        self.items_margin
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn items(&self) -> usize {
        self.data.len()
    }

    pub fn stats(&self) -> JsValue {
        self.stats.json()
    }
}
