use crate::utils::units::Energy;
use crate::config::data_reader::{LCLEnergyConsumptionRecord, UkPvSolarGenerationRecord, load_dataset};
use chrono::NaiveDateTime;

use std::path::PathBuf;


pub struct HouseData {
    pub consumption_data: Vec<LCLEnergyConsumptionRecord>,
    generation_data: Vec<UkPvSolarGenerationRecord>,
    pub consumption_schedule: Vec<Option<Energy>>,
    pub generation_schedule: Vec<Option<Energy>>,
}

impl HouseData {
    /// Build a period-indexed schedule from the records, using the start date and period count.
    pub fn build_consumption_schedule(&mut self, start_date: &NaiveDateTime, period_count: usize) {
        let mut schedule = vec![None; period_count];
        for record in &self.consumption_data {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&record._date_time, "%Y-%m-%d %H:%M:%S") {
                let period = ((dt.signed_duration_since(*start_date).num_minutes()) / 30) as usize;
                if period < period_count {
                    schedule[period] = Some(Energy::new(record.consumption_wh.round() as u32));
                }
            }
        }
        self.consumption_schedule = schedule;
    }

    pub fn build_generation_schedule(&mut self, start_date: &NaiveDateTime, period_count: usize) {
        let mut schedule = vec![None; period_count];
        use chrono::DateTime;
        for record in &self.generation_data {
            if let Ok(dt) = DateTime::parse_from_str(&record._datetime_gmt, "%Y-%m-%d %H:%M:%S%z") {
                let naive_dt = dt.naive_local();
                let period = ((naive_dt.signed_duration_since(*start_date).num_minutes()) / 30) as usize;
                if period < period_count {
                    schedule[period] = Some(Energy::new(record.generation_wh.round() as u32));
                }
            }
        }
        self.generation_schedule = schedule;
    }

    pub fn consumption_energy(&self) -> Vec<Energy> {
        self.consumption_schedule.iter().map(|e| e.unwrap_or(Energy::new(0))).collect()
    }
    pub fn generation_energy(&self) -> Vec<Energy> {
        self.generation_schedule.iter().map(|e| e.unwrap_or(Energy::new(0))).collect()
    }
}

impl From<PathBuf> for HouseData {
    fn from(consumption_dataset: PathBuf) -> Self {
        HouseData {
            consumption_data: load_dataset::<LCLEnergyConsumptionRecord>(&consumption_dataset),
            generation_data: Vec::new(),
            consumption_schedule: Vec::new(),
            generation_schedule: Vec::new(),
        }
    }
}

impl From<(PathBuf, PathBuf)> for HouseData {
    fn from((consumption_dataset, generation_dataset): (PathBuf, PathBuf)) -> Self {
        HouseData {
            consumption_data: load_dataset::<LCLEnergyConsumptionRecord>(&consumption_dataset),
            generation_data: load_dataset::<UkPvSolarGenerationRecord>(&generation_dataset),
            consumption_schedule: Vec::new(),
            generation_schedule: Vec::new(),
        }
    }
}