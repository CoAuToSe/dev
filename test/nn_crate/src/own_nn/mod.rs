//! regroups every types and their unique implementations
pub mod connection;
pub mod connection_old;
pub mod layer;
pub mod neural_net;
pub mod neural_network;
pub mod neural_ops;
pub mod neuron;
pub mod neuron_old;

pub mod back_prop;
// pub mod evolution; ***Coming soon!***

use self::connection_old::Connection;
use self::layer::Layer;
use self::neural_net::*;
use self::neural_network::*;
use self::neuron_old::Neuron;
// use fast_io::prelude::*;
