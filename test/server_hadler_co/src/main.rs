use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    sync::{
        Arc,
        Mutex, 
        mpsc::{
            self,
            *
        }
    },
    thread::{
        self,
        *
    },
    time::Duration,
    str,
    io::prelude::*,
    fs,
};

static mut COST: bool = false;

#[derive(Debug)]
struct Connection {
    state: Possible_state,

    listener: Arc<Mutex<TcpStream>>, 
    thread_listening: JoinHandle<()>, 
    datas_listening : Arc<Mutex<Vec<Vec<u8>>>>,
    listerning_sender: Sender<Vec<u8>>,
    listerning_receiver: Arc<Mutex<Receiver<Vec<u8>>>>,

    talker: Arc<Mutex<TcpStream>>,
    thread_talking: JoinHandle<()>,
    datas_talking: Arc<Mutex<Vec<Vec<u8>>>>,
    talking_sender: Sender<Vec<u8>>,
    talking_receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
}

#[derive(Debug, PartialEq)]
enum Possible_state{
    Initializing,
    Actif,
    Waiting,
    Dead,
}

#[derive(Debug)]
enum Typeses {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64, f128,
}

impl Connection {
    fn new(my_stream: Option<TcpStream>, stream_to_connect: Option<TcpStream>) -> Connection {

        let temp = TcpStream::connect("127.0.0.1:17172");
        println!("{:?}", temp);
        let mut own_stream = match my_stream {
            Some(stream) => Arc::new(Mutex::new(stream)),
            None => Arc::new(Mutex::new(temp.unwrap())),
        };
        let mut temp_own_stream = own_stream.clone();
        let mut data_listener = Arc::new(Mutex::new(vec![]));
        let temp_data_listener = data_listener.clone();
        let (listerner_sender, listerner_receiver) = mpsc::channel::<Vec<u8>>();
        let listerner_receiver = Arc::new(Mutex::new(listerner_receiver));
        let temp_listerner_reicever = listerner_receiver.clone();
        let thread_listerner = thread::spawn(move || loop {
            // let message = temp_talker_reicever.lock().unwrap().recv().unwrap();
            let mut buffer: [u8; 1024]= [0; 1024];
            // // println!("stream :\n{:#?}", stream);
            let sometimes = temp_own_stream.lock().unwrap().read(&mut buffer).unwrap();
            let mut threaded_thing = temp_data_listener.lock().unwrap();
            threaded_thing.push({
                let mut temp = vec![];
                for i in 0..sometimes { temp.push(buffer[i]) };
                temp
            });
            println!("{:?}", threaded_thing);
            if unsafe{COST} {break}
            // let message = temp_listerner_reicever.lock().unwrap().recv().unwrap();
            // own_stream.read(&message);
        });


        let mut other_stream = match stream_to_connect {
            Some(stream) => Arc::new(Mutex::new(stream)),
            None => Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:8080").unwrap())),
        };
        let mut temp_other_stream = other_stream.clone();
        let mut data_talker = Arc::new(Mutex::new(vec![]));
        let (talker_sender, talker_receiver) = mpsc::channel::<Vec<u8>>();
        let talker_receiver = Arc::new(Mutex::new(talker_receiver));
        let temp_talker_reicever = talker_receiver.clone();
        let thread_talker = thread::spawn(move || loop {
            let message = temp_talker_reicever.lock().unwrap().recv().unwrap();
            temp_other_stream.lock().unwrap().write(&message);
            if unsafe{COST} {break}
            // match &self.state {
            //     Possible_state::Dead => {break},
            //     _ => ()
            // }
            // if let something = self.state {
            //     if something == Possible_state::Dead {
            //         break
            //     }
            // };
        });

        Connection {
            state: Possible_state::Initializing,

            listener: own_stream, 
            thread_listening: thread_listerner, 
            datas_listening : data_listener,
            listerning_sender: listerner_sender,
            listerning_receiver: listerner_receiver,

            talker: other_stream,
            thread_talking: thread_talker,
            datas_talking: data_talker,
            talking_sender: talker_sender,
            talking_receiver: talker_receiver,
        }
    }
}

fn main() {
    println!("Hello, world!");
        let temp = TcpStream::connect("127.0.0.1:8080")?;
        println!("{:?}", temp);
    let the_co = Connection::new(None, None);
    println!("{:?}", the_co);
}




fn connect_to(addrty : Option<&str>, addrty2 : Option<String>) -> std::io::Result<()> {
    let mut stream = match addrty {
        Some(art) => TcpStream::connect(art)?,
        None => TcpStream::connect("127.0.0.1:8080")?,
    };
    // let mut my_adret = match addrty2 {
    //         Some(art) => art,
    //         None => {
    //             let my_ip =  stream.local_addr().unwrap().ip();
    //             let my_port =  stream.local_addr().unwrap().port();
    //             format!("{:?}:{:?}", my_ip, my_port)
    //         },
    //     };
    // println!("my_addr: {:?}",my_adret);
    // let as_uadfaz = {
    //     let mut temp = vec![];
    //     for e in my_adret.bytes() {temp.push(e)}
    //     temp
    // };

    stream.write(&[1u8,8,9] as &[u8])?;
    stream.read(&mut [0; 128])?;
    // println!("{:?}:{:?}", stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());
    Ok(())
} // the stream is closed here