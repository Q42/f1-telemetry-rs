use serde::Deserialize;
use std::io::BufRead;

use crate::{
    packet::{
        header::PacketHeader,
        tyre_sets::{PacketTyreSetsData, TyreSetData},
        UnpackError,
    },
    utils::assert_packet_size,
};

use super::consts::TYRE_SETS_PACKET_SIZE;

#[derive(Deserialize)]
struct RawPacketData {
    car_idx: u8,
    tyre_sets_data: [RawTyreSetData; 20],
    fitted_idx: u8,
}

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
#[derive(Deserialize)]
pub struct RawTyreSetData {
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

impl TryFrom<&RawTyreSetData> for TyreSetData {
    type Error = UnpackError;

    fn try_from(ts: &RawTyreSetData) -> Result<Self, Self::Error> {
        Ok(Self {
            actual_tyre_compound: ts.actual_tyre_compound,
            visual_tyre_compound: ts.visual_tyre_compound,
            wear: ts.wear,
            available: ts.available,
            reccomended_session: ts.reccomended_session,
            lifespan: ts.lifespan,
            usable_life: ts.usable_life,
            lap_delta_time: ts.lap_delta_time,
            fitted: ts.fitted,
        })
    }
}

pub(crate) fn parse_tyre_sets_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketTyreSetsData, UnpackError> {
    assert_packet_size(size, TYRE_SETS_PACKET_SIZE)?;
    let packet_data: RawPacketData = bincode::deserialize_from(reader)?;
    let tyre_sets_data = packet_data
        .tyre_sets_data
        .iter()
        .map(|ts| ts.try_into())
        .collect::<Result<Vec<TyreSetData>, UnpackError>>()?;
    Ok(PacketTyreSetsData {
        header,
        car_idx: packet_data.car_idx,
        tyre_sets_data,
        fitted_idx: packet_data.fitted_idx,
    })
}
