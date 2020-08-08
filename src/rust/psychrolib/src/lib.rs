/*
 * PsychroLib (version 2.5.0) (https://github.com/psychrometrics/psychrolib).
 * Copyright (c) 2018-2020 The PsychroLib Contributors for the current library implementation.
 * Copyright (c) 2017 ASHRAE Handbook — Fundamentals for ASHRAE equations and coefficients.
 * Licensed under the MIT License.
*/

/******************************************************************************************************
 * Global constants
 *****************************************************************************************************/

const ZERO_FARENHEIT_AS_RANKINE: f64 = 459.67; // Zero degree Fahrenheit (°F) expressed as degree Rankine (°R).
                                               // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 39.

const ZERO_CELCIUS_AS_KELVIN: f64 = 273.15; // Zero degree Celsius (°C) expressed as Kelvin (K).
                                            // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 39.

const R_DA_IP: f64 = 53.350; // Universal gas constant for dry air (IP version) in ft∙lbf/lb_da/R.
                             // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1.

const R_DA_SI: f64 = 287.042; // Universal gas constant for dry air (SI version) in J/kg_da/K.
                              // Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1.

const MAX_ITER_COUNT: usize = 100; // Max number of iterations before exiting while loop
const MIN_HUM_RATIO: f64 = 1e-7; // Minimum acceptable humidity ratio used/returned by any functions.
                                 // Any value above 0 or below the MIN_HUM_RATIO will be reset to this value.

const FREEZING_POINT_WATER_IP: f64 = 32.0; // Freezing point of water in Fahrenheit.

const FREEZING_POINT_WATER_SI: f64 = 0.0; // Freezing point of water in Celsius.

const TRIPLE_POINT_WATER_IP: f64 = 32.018; // Triple point of water in Fahrenheit.

const TRIPLE_POINT_WATER_SI: f64 = 0.01; // Triple point of water in Celsius.

const TOLERANCE_IP: f64 = 0.001 * 9.0 * 5.0; // Tolerance of temperature calculations in IP

const TOLERANCE_SI: f64 = 0.001; //Tolerance of temperature calculations in SI

/// UnitSystem describes the unit system (SI or IP) in use by psychrolib
#[derive(PartialEq, Debug)]
pub enum UnitSystem {
    IP,
    SI,
}

/// Psychrolib is the struct that represents the unit system and tolerance of an instance of the library
pub struct Psychrolib {
    units: UnitSystem,
    tolerance: f64,
}

impl Psychrolib {
    /// Instantiates Psychrolib struct with unit system to use (SI or IP) and associated tolerance
    ///
    /// # Example
    ///     use crate::psychrolib::UnitSystem;
    ///     use crate::psychrolib::Psychrolib;
    ///
    ///     let unit_system = psychrolib::UnitSystem::SI;
    ///     let psych = Psychrolib::new(unit_system);
    pub fn new(units: UnitSystem) -> Psychrolib {
        let tolerance;

        if units == UnitSystem::IP {
            tolerance = TOLERANCE_IP;
        } else {
            tolerance = TOLERANCE_SI;
        }

        Psychrolib { units, tolerance }
    }

    /// Returns the unit system in use by the Psychrolib
    ///
    /// # Example
    ///     use crate::psychrolib::UnitSystem;
    ///     use crate::psychrolib::Psychrolib;
    ///
    ///     let unit_system = psychrolib::UnitSystem::IP;
    ///     let psych = Psychrolib::new(unit_system);
    ///
    ///     assert_eq!(psych.get_units(), &psychrolib::UnitSystem::IP)
    ///
    pub fn GetUnitSystem(&self) -> &UnitSystem {
        &self.units
    }

    /// Sets the unit system to a Psychrolib already in use
    ///
    /// # Example:
    ///     use crate::psychrolib::UnitSystem;
    ///     use crate::psychrolib::Psychrolib;
    ///
    ///     let mut psych = Psychrolib::new(psychrolib::UnitSystem::IP);
    /// 
    ///     assert_eq!(psych.get_units(), &psychrolib::UnitSystem::IP);
    ///
    ///     psych.set_units(psychrolib::UnitSystem::SI);
    /// 
    ///     assert_eq!(psych.get_units(), &psychrolib::UnitSystem::SI);
    pub fn set_units(&mut self, unit_system: UnitSystem) {
        self.units = unit_system;
    }
}

/******************************************************************************************************
 * Helper functions
 *****************************************************************************************************/

 fn get_t_rankine_from_t_fahrenheit(t_fahrenheit: f64) -> f64 {

    t_fahrenheit
 }





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_units_ip() {
        let psych = Psychrolib::new(UnitSystem::IP);

        assert_eq!(psych.units, UnitSystem::IP);
        assert_eq!(psych.tolerance, TOLERANCE_IP);
    }

    #[test]
    fn get_unit_test() {
        let psych = Psychrolib::new(UnitSystem::IP);

        assert_eq!(psych.get_units(), &UnitSystem::IP);
    }

    #[test]
    fn get_t_rankine_from_t_fahrenheit() {
        let psych = Psychrolib::new(UnitSystem::IP);

        assert_eq!(psych.get_t_rankine_from_t_fahrenheit());

    }
}
