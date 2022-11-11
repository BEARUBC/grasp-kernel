#[cfg(feature = "tcp_edge")]
use std::ops::RangeInclusive;

use anyhow::Result;
use crossbeam::channel::Sender;

use crate::components::kernel;
#[cfg(feature = "tcp_edge")]
use crate::components::utils::run_tcp;
use crate::components::Component;
#[cfg(feature = "tcp_edge")]
use crate::config;
use crate::config::Config;
#[cfg(feature = "tcp_edge")]
use crate::config::TcpEdge;

#[cfg(feature = "tcp_edge")]
const MAX_BATTERY: f64 = 100.0;

#[cfg(feature = "tcp_edge")]
const HIGH_BATTERY_CUTOFF: f64 = 70.0;

#[cfg(feature = "tcp_edge")]
const MEDIUM_BATTERY_CUTOFF: f64 = 20.0;

#[cfg(feature = "tcp_edge")]
const HIGH_BATTERY_RANGE: RangeInclusive<f64> = HIGH_BATTERY_CUTOFF..=MAX_BATTERY;

#[cfg(feature = "tcp_edge")]
const MEDIUM_BATTERY_RANGE: RangeInclusive<f64> = MEDIUM_BATTERY_CUTOFF..=HIGH_BATTERY_CUTOFF;

pub(super) enum BatteryReport {
    High,
    Medium,
    Low,
}

pub(super) struct Bms {
    pub(super) tx: Sender<kernel::Message>,
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Bms {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Bms {
    fn run(
        self,
        Config {
            tcp_edge:
                TcpEdge {
                    bms: config::Bms { host, port },
                    ..
                },
            ..
        }: &Config,
    ) -> Result<()> {
        run_tcp(
            host,
            *port,
            move |data| {
                let battery_report = match data {
                    _ if HIGH_BATTERY_RANGE.contains(&data) => BatteryReport::High,
                    _ if MEDIUM_BATTERY_RANGE.contains(&data) => BatteryReport::Medium,
                    _ => BatteryReport::Low,
                };
                let message = kernel::Message::Bms(battery_report);
                self.tx.send(message)?;
                Ok(())
            },
            None,
        )?;
        Ok(())
    }
}
