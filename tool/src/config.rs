use std::str::FromStr;

use crate::others::keys::Key;
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    // commands: Vec<CfgCommand>,
}

impl Config {
    pub(crate) fn parse(cfg: &str) -> Result<Self, String> {
        toml::from_str::<Config>(cfg).map_err(|e| format!("TOML configuration parse error: {}", e))
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: "0".parse().unwrap(),
                hide: "rshift+0".parse().ok(),
                show_console: false,
                indicators: Indicator::default_set(),
            },
            // commands: Vec::new(),
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: Key,
    pub(crate) hide: Option<Key>,
    #[serde(default)]
    pub(crate) show_console: bool,
    #[serde(default = "Indicator::default_set")]
    pub(crate) indicators: Vec<Indicator>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "IndicatorConfig")]
pub(crate) struct Indicator {
    pub(crate) indicator: IndicatorType,
    pub(crate) enabled: bool,
}

impl Indicator {
    fn default_set() -> Vec<Indicator> {
        vec![
            Indicator { indicator: IndicatorType::GameVersion, enabled: true },
            Indicator { indicator: IndicatorType::Igt, enabled: true },
            Indicator { indicator: IndicatorType::Position, enabled: false },
            Indicator { indicator: IndicatorType::PositionChange, enabled: false },
            Indicator { indicator: IndicatorType::Animation, enabled: false },
            Indicator { indicator: IndicatorType::Fps, enabled: false },
            Indicator { indicator: IndicatorType::FrameCount, enabled: false },
            Indicator { indicator: IndicatorType::ImguiDebug, enabled: false },
        ]
    }
}


#[derive(Debug, Deserialize, Clone)]
struct IndicatorConfig {
    indicator: String,
    enabled: bool,
}


impl TryFrom<IndicatorConfig> for Indicator {
    type Error = String;

    fn try_from(indicator: IndicatorConfig) -> Result<Self, Self::Error> {
        match indicator.indicator.as_str() {
            "igt" => Ok(Indicator { indicator: IndicatorType::Igt, enabled: indicator.enabled }),
            "position" => {
                Ok(Indicator { indicator: IndicatorType::Position, enabled: indicator.enabled })
            },
            "position_change" => Ok(Indicator {
                indicator: IndicatorType::PositionChange,
                enabled: indicator.enabled,
            }),
            "game_version" => {
                Ok(Indicator { indicator: IndicatorType::GameVersion, enabled: indicator.enabled })
            },
            "imgui_debug" => {
                Ok(Indicator { indicator: IndicatorType::ImguiDebug, enabled: indicator.enabled })
            },
            "fps" => Ok(Indicator { indicator: IndicatorType::Fps, enabled: indicator.enabled }),
            "framecount" => {
                Ok(Indicator { indicator: IndicatorType::FrameCount, enabled: indicator.enabled })
            },
            "animation" => {
                Ok(Indicator { indicator: IndicatorType::Animation, enabled: indicator.enabled })
            },
            value => Err(format!("Unrecognized indicator: {value}")),
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub(crate) enum IndicatorType {
    Igt,
    Position,
    PositionChange,
    GameVersion,
    ImguiDebug,
    Fps,
    FrameCount,
    Animation,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
pub(crate) struct LevelFilterSerde(LevelFilter);

impl LevelFilterSerde {
    pub(crate) fn inner(&self) -> LevelFilter {
        self.0
    }
}

impl TryFrom<String> for LevelFilterSerde {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LevelFilterSerde(LevelFilter::from_str(&value).map_err(
            |e| format!("Couldn't parse log level filter: {}", e),
        )?))
    }
}
