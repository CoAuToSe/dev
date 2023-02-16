// use fast_io::prelude::*;
use super::super::Connection as C;

#[derive(Clone, Debug)]
pub struct Connection {
    pub weight: f64,
    pub prev_delta: f64,
}

impl PartialEq for Connection {
    fn eq(&self, rhs: &Connection) -> bool {
        self.weight == rhs.weight
    }
}

// impl CustomIO for Connection {
//     fn save<T: CopyIO>(&self, f: &mut T) -> Result<()> {
//         f.write_copy(&self.weight)
//     }
//     fn load<T: CopyIO>(f: &mut T) -> Result<Self> {
//         Ok(Connection {
//             weight: f.read_copy()?,
//             prev_delta: 0.0,
//         })
//     }
// }

impl C for Connection {
    fn new(weight: f64) -> Connection {
        Connection {
            weight,
            prev_delta: 0.0,
        }
    }

    fn weight(&self) -> f64 {
        self.weight
    }
}
