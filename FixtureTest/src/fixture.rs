use std::collections::HashMap;
use crate::color::Color;

pub(crate) struct Channel{
    pub(crate) value: u16,
    is_real: bool,
    channel : u16,
    fine_channel: Option<u16>,
}

trait MultiChannel {
    fn get_last_channel(&self) -> u16;
}

struct Beam{
    zoom: Channel,
    focus: Channel,
    frost: Channel,
}

impl MultiChannel for Beam {
    fn get_last_channel(&self) -> u16 {
        let mut last_channel = 0;
        for channel in [&self.zoom, &self.focus, &self.frost] {
            if channel.is_real && channel.channel > last_channel {
                last_channel = channel.channel;
            }
            if let Some(fine) = channel.fine_channel {
                if fine > last_channel {
                    last_channel = fine;
                }
            }
        }
        last_channel
    }
}

struct Prism{
    prism: Channel,
    prism_rotation: Channel,
    prism_indexation: Channel,
}

impl MultiChannel for Prism {
    fn get_last_channel(&self) -> u16 {
        let mut last_channel = 0;
        for channel in [&self.prism, &self.prism_rotation, &self.prism_indexation] {
            if channel.is_real && channel.channel > last_channel {
                last_channel = channel.channel;
            }
            if let Some(fine) = channel.fine_channel {
                if fine > last_channel {
                    last_channel = fine;
                }
            }
        }
        last_channel
    }
}

/// Represents the various configurable properties of a lighting fixture.
///
/// This enum encapsulates all supported attribute types of a fixture,
/// allowing each property to carry its associated DMX channel(s).
/// It provides a unified way to describe colors, movement, beam effects,
/// gobos, atmospheric controls, and any custom or manufacturer-specific
/// attributes.
///
/// # Variants
///
/// * **Color(Color)**
///   A color-related property such as RGB, CMY, or other color-mixing systems.
///
/// * **Dimmer(Channel)**
///   Controls fixture brightness (0–255).
///
/// * **Strobe(Channel)**
///   Controls strobe rate or shutter pulse effects.
///
/// * **Beam { zoom, focus, frost }**
///   Beam-shaping properties:
///   - `zoom`: controls beam width
///   - `focus`: controls sharpness
///   - `frost`: applies diffusion/frost effect
///
/// * **Shutter(Channel)**
///   Mechanical shutter control (open/close).
///
/// * **Prism { prism, prism_rotation, prism_indexation }**
///   Prism and prism-effect controls:
///   - `prism`: enables/selects prism
///   - `prism_rotation`: rotation speed/direction
///   - `prism_indexation`: discrete index positioning
///
/// * **Gobo { gobo_rotation, gobo_rotation_speed, gobo_wheel_rotation, gobo_wheel_rotation_speed }**
///   Gobo selection and motion:
///   - `gobo_rotation`: absolute rotation
///   - `gobo_rotation_speed`: continuous rotation speed
///   - `gobo_wheel_rotation`: selects wheel slot rotation
///   - `gobo_wheel_rotation_speed`: rotation speed of the gobo wheel
///
/// * **Position { pan, tilt }**
///   Movement parameters for head-positioning.´
///
/// * **UV(Channel)**
///   UV-LED intensity control.
///
/// * **Speed(Channel)**
///   Global macro speed or effect speed.
///
/// * **Fog { fog_intensity, fog_fan_speed }**
///   Atmospheric effects:
///   - `fog_intensity`: fog output amount
///   - `fog_fan_speed`: fan speed for fog dispersion
///
/// * **Other(String, Channel)**
///   Any manufacturer-specific or unsupported property, given as a descriptive name and channel index.
enum Property {
    Color(Color),
    Dimmer(Channel),
    Strobe(Channel),
    Beam(Beam),
    Shutter(Channel),
    Prism{
        prism: Channel,
        prism_rotation: Channel,
        prism_indexation: Channel,
    },
    Gobo{
        gobo_rotation: Channel,
        gobo_rotation_speed: Channel,
        gobo_wheel_rotation: Channel,
        gobo_wheel_rotation_speed: Channel,
    },
    Position{
        pan: Channel,
        tilt: Channel,
    },
    UV(Channel),
    Speed(Channel),
    Fog{
        fog_intensity: Channel,
        fog_fan_speed: Channel,
    },

    Other(String, Channel),
}

struct FixtureType {
    name: String,
    manufacturer: String,
    total_channels: u16,
    properties: HashMap<String, Property>,
}

impl FixtureType {
    fn new(name: String, manufacturer: String, properties: HashMap<String, Property>) -> Self {
        let total_channels = properties.values().map(

        ).max();
        FixtureType {
            name,
            manufacturer,
            total_channels,
            properties,
        }
    }
}


struct Fixture {
    properties: HashMap<String, Property>,

}

impl Fixture {
    fn new(properties_list: HashMap<String, Property>) -> Self {
        Fixture {
            properties: properties_list
        }
    }

}