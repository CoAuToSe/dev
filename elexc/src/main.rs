use core::default::*;

#[derive(Default, Clone, Copy)]
enum SignalType {
    Analog,
    Digital,
    #[default]
    Undefined,
}

#[derive(Default, Clone, Copy)]
enum PinState {
    Reading(SignalType),
    Output(SignalType),
    Interupt,
    Multiple(&'static PinState, &'static PinState),
    Other,
    #[default]
    Undefined,
}

const NUM_PIN: usize = 10;
const PINS: [(usize, PinState); NUM_PIN] = {
    let mut default_list: [(usize, PinState); NUM_PIN] = [(0, PinState::Undefined); NUM_PIN];
    let mut index = 0;
    'a: loop {
        default_list[index] = (index, PinState::Undefined);
        index += 1;
        if index == NUM_PIN {
            break 'a;
        }
    }
    default_list
};

fn main() {
    println!("Hello, world!");
}
