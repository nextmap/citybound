pub mod lane;
pub mod construction;
pub mod microtraffic;
pub mod rendering;

pub mod planning;
pub mod pathfinding;

use kay::ActorSystem;
use core::simulation::SimulationID;

pub fn setup(system: &mut ActorSystem, simulation: SimulationID) {
    self::lane::setup(system);
    self::construction::setup(system);
    self::microtraffic::setup(system);
    self::pathfinding::setup(system, simulation);
    self::rendering::setup(system);
    self::planning::setup(system);
}
