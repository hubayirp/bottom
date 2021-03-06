use battery::{
    units::{power::watt, ratio::percent, time::second},
    Battery, Manager,
};

#[derive(Debug, Clone)]
pub struct BatteryHarvest {
    pub charge_percent: f64,
    pub secs_until_full: Option<i64>,
    pub secs_until_empty: Option<i64>,
    pub power_consumption_rate_watts: f64,
    pub health_percent: f64,
}

pub fn refresh_batteries(manager: &Manager, batteries: &mut [Battery]) -> Vec<BatteryHarvest> {
    batteries
        .iter_mut()
        .filter_map(|battery| {
            if manager.refresh(battery).is_ok() {
                Some(BatteryHarvest {
                    secs_until_full: {
                        let optional_time = battery.time_to_full();
                        if let Some(time) = optional_time {
                            Some(f64::from(time.get::<second>()) as i64)
                        } else {
                            None
                        }
                    },
                    secs_until_empty: {
                        let optional_time = battery.time_to_empty();
                        if let Some(time) = optional_time {
                            Some(f64::from(time.get::<second>()) as i64)
                        } else {
                            None
                        }
                    },
                    charge_percent: f64::from(battery.state_of_charge().get::<percent>()),
                    power_consumption_rate_watts: f64::from(battery.energy_rate().get::<watt>()),
                    health_percent: f64::from(battery.state_of_health().get::<percent>()),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
