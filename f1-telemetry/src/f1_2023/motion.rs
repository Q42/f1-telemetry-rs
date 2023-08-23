use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::motion::{CarMotionData, PacketMotionData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// ## Specification
/// ```text
/// world_position_x:     World space X position
/// world_position_y:     World space Y position
/// world_position_z:     World space Z position
/// world_velocity_x:     Velocity in world space X
/// world_velocity_y:     Velocity in world space Y
/// world_velocity_z:     Velocity in world space Z
/// world_forward_dir_x:  World space forward X direction (normalised)
/// world_forward_dir_y:  World space forward Y direction (normalised)
/// world_forward_dir_z:  World space forward Z direction (normalised)
/// world_right_dir_x:    World space right X direction (normalised)
/// world_right_dir_y:    World space right Y direction (normalised)
/// world_right_dir_z:    World space right Z direction (normalised)
/// g_force_lateral:      Lateral G-Force component
/// g_force_longitudinal: Longitudinal G-Force component
/// g_force_vertical:     Vertical G-Force component
/// yaw:                  Yaw angle in radians
/// pitch:                Pitch angle in radians
/// roll:                 Roll angle in radians
/// ```
#[derive(Deserialize)]
struct RawCarMotion {
    world_position_x: f32,
    world_position_y: f32,
    world_position_z: f32,
    world_velocity_x: f32,
    world_velocity_y: f32,
    world_velocity_z: f32,
    world_forward_dir_x: i16,
    world_forward_dir_y: i16,
    world_forward_dir_z: i16,
    world_right_dir_x: i16,
    world_right_dir_y: i16,
    world_right_dir_z: i16,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl From<&RawCarMotion> for CarMotionData {
    fn from(car_motion: &RawCarMotion) -> Self {
        Self {
            world_position_x: car_motion.world_position_x,
            world_position_y: car_motion.world_position_y,
            world_position_z: car_motion.world_position_z,
            world_velocity_x: car_motion.world_velocity_x,
            world_velocity_y: car_motion.world_velocity_y,
            world_velocity_z: car_motion.world_velocity_z,
            world_forward_dir_x: car_motion.world_forward_dir_x,
            world_forward_dir_y: car_motion.world_forward_dir_y,
            world_forward_dir_z: car_motion.world_forward_dir_z,
            world_right_dir_x: car_motion.world_right_dir_x,
            world_right_dir_y: car_motion.world_right_dir_y,
            world_right_dir_z: car_motion.world_right_dir_z,
            g_force_lateral: car_motion.g_force_lateral,
            g_force_longitudinal: car_motion.g_force_longitudinal,
            g_force_vertical: car_motion.g_force_vertical,
            yaw: car_motion.yaw,
            pitch: car_motion.pitch,
            roll: car_motion.roll,
        }
    }
}

pub(crate) fn parse_motion_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionData, UnpackError> {
    assert_packet_size(size, MOTION_PACKET_SIZE)?;

    let motion_data: [RawCarMotion; NUMBER_CARS] = bincode::deserialize_from(reader)?;

    let car_motion = motion_data.iter().map(|cm| cm.into()).collect();

    Ok(PacketMotionData {
        header,
        motion_data: car_motion,
        player_car_data: None,
    })
}
