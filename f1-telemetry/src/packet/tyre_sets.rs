use serde::Serialize;

use super::header::PacketHeader;

/// This type is used for the `tyre_set_data` array of the [`PacketTyreSetData`] type.
///
/// ```text
/// ## Specification
/// actual_tyre_compound:  Actual tyre compound
/// visual_tyre_compound:  Visual tyre compound
/// wear:                  Tyre wear percentage
/// available:             Whether this set is currently available
/// reccomended_session:   Reccomended session for tyre set
/// lifespan:              Laps left in this tyre set
/// usable_life:           Number of laps reccomended for this compound
/// lap_delta_time:        Lap delta time in milliseconds compared to fitted
/// fitted:                Whether the set is fitted or not
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct TyreSetData {
    pub actual_tyre_compound: u8,
    pub visual_tyre_compound: u8,
    pub wear: u8,
    pub available: u8,
    pub reccomended_session: u8,
    pub lifespan: u8,
    pub usable_life: u8,
    pub lap_delta_time: u16,
    pub fitted: u8,
}

/// This packets gives a more in-depth details about tyre sets assigned to a vehicle during the session.
/// Frequency: 20 per second but cycling through cars
/// Size: 231 bytes
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketTyreSetsData {
    pub header: PacketHeader,
    pub car_idx: u8,
    pub tyre_sets_data: Vec<TyreSetData>,
    pub fitted_idx: u8,
}
