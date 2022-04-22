mod threadpool;

use threadpool::ThreadPool;

use std::{
    error::Error,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

pub struct Transport {
    address: String,
    port: u16,
}

type F = Arc<dyn Fn(TcpStream) + Send + Sync>;
// type F = Box<dyn FnOnce(TcpStream) + Send + 'static>;

pub trait TransportTrait {
    fn new(port: u16) -> Transport;
    fn listen(
        &self,
        fn_app_level: F, // fn_app_level: Arc<dyn Fn(TcpStream) + Send + Sync>,
    ) -> Result<(), Box<dyn Error + 'static>>;
}

impl TransportTrait for Transport {
    fn new(port: u16) -> Transport {
        Transport {
            address: "127.0.0.1".to_string(),
            port,
        }
    }

    fn listen(
        &self,
        fn_app_level: F, // fn_app_level: Arc<dyn Fn(TcpStream) + Send + Sync>,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port))?;

        println!(
            "Server listening on {}",
            format!("{}:{}", self.address, self.port)
        );

        let threads = ThreadPool::new(8);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let func = Arc::clone(&fn_app_level);
                    threads.execute(move || {
                        func(stream);
                    })?
                }
                Err(error) => println!("Connection failed. Error: {}", error),
            }
        }

        drop(listener);

        Ok(())
    }
}
