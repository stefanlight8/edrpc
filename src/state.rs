pub enum State {
    InGame,
    DeepSpace {
        star_system: String,
    },
    Supercruise {
        star_system: String,
    },
    UssSignal {
        uss_type: String,
        uss_threat: String,
    },
    Body {
        body: String,
    },
    Station {
        station: String,
    },
}
