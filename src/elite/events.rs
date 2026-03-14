use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JournalEntry {
    pub timestamp: DateTime<Local>,
    #[serde(flatten)]
    pub event: Event,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Event {
    ApproachBody {
        body: String,
        star_system: String,
    },
    ApproachSettlement {
        #[serde(alias = "BodyName")]
        body: String,
        name: String,
        #[serde(alias = "Name_Localised")]
        name_localised: Option<String>,
    },
    CarrierJump {
        body: String,
        on_foot: bool,
    },
    CodexEntry {
        system: String,
    },
    Disembark {
        body: String,
        #[serde(alias = "SRV")]
        on_srv: bool,
        star_system: String,
        station_name: Option<String>,
    },
    Docked {
        star_system: String,
        station_name: String,
    },
    FSDJump {
        body: String,
        star_system: String,
    },
    FSSAllBodiesFound {
        #[serde(alias = "system_name")]
        star_system: String,
    },
    FSSDiscoveryScan {
        #[serde(alias = "system_name")]
        star_system: String,
    },
    LeaveBody {
        body: String,
        star_system: String,
    },
    Liftoff {
        body: Option<String>,
        star_system: Option<String>,
    },
    LoadGame {
        ship: Option<String>,
        ship_ident: Option<String>,
        ship_name: Option<String>,
        start_dead: Option<bool>,
        start_landed: Option<bool>,
    },
    Loadout {
        ship: String,
        ship_ident: String,
        ship_name: String,
    },
    Location {
        body: String,
        body_type: BodyType,
        docked: bool,
        on_foot: bool,
        star_system: String,
        station_name: Option<String>,
    },
    Shutdown,
    StartJump {
        jump_type: JumpType,
        star_system: Option<String>,
    },
    SupercruiseDestinationDrop {
        #[serde(alias = "Type")]
        destination: String,
        #[serde(alias = "Type_Localised")]
        destination_localized: Option<String>,
    },
    SupercruiseEntry {
        star_system: String,
    },
    SupercruiseExit {
        body: String,
        star_system: String,
    },
    Touchdown {
        body: Option<String>,
        #[serde(alias = "NearestDestination")]
        nearest_destination: Option<String>,
        #[serde(alias = "NearestDestination_Localised")]
        nearest_localised: Option<String>,
        star_system: Option<String>,
    },
    USSDrop {
        #[serde(alias = "USSType_Localised")]
        name: String,
    },
    Undocked {
        station_name: String,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum BodyType {
    Null,
    Planet,
    PlanetaryRing,
    Star,
    Station,
    StellarRing,
}

#[derive(Deserialize, Debug)]
pub enum JumpType {
    Hyperspace,
    Supercruise,
}
