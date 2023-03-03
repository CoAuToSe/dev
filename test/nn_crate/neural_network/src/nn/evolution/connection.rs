use fast_io::prelude::*;
use super::super::Connection as C;

#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    pub weight: f64
}

impl CustomIO for Connection {
    fn save<T: CopyIO>(&self, f: &mut T) -> Result<()> {
        f.write_copy(&self.weight)
    }
    fn load<T: CopyIO>(f: &mut T) -> Result<Self> {
        Ok(Connection {
            weight: f.read_copy()?
        })
    }
}

impl C for Connection {
    fn new(weight: f64) -> Connection {
        Connection {
            weight
        }
    }

    fn weight(&self) -> f64 {
        self.weight
    }
}