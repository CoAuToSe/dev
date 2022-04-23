pub mod debug {
    use std::env::{Args, ArgsOs};
    pub fn test_debug(args: &Args, args_os: &ArgsOs) -> bool {
        if format!("{:?}", args).split_off(5) != format!("{:?}", args_os).split_off(7) {
            // println!("{:?}", stringify!(args.inner)[5..]);
            // println!("{:?}", stringify!(args_os.inner)[7..]);
            println!("(WIP) don't know what is the difference between args and args_os");
            // return false;
            println!("{:?}", format!("{:?}", args).split_off(5));
            println!("{:?}", format!("{:?}", args_os).split_off(7));
            return false;
        }
        return true;
    }
}
