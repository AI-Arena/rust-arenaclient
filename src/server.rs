use crossbeam::channel::{self, TryRecvError};
// use log::{error, info, warn};
// use std::env::var;
// use std::fs::File;
// use std::io::prelude::*;
use crate::controller::Controller;
use crate::proxy;
use bincode::{deserialize, serialize};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::ToPyObject;
use serde::{Deserialize, Serialize};
use std::thread;
use std::thread::JoinHandle;

pub enum ClientType {
    Bot,
    Controller,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RustServer {
    ip_addr: String,
}

impl RustServer {
    pub fn new(ip_addr: &str) -> Self {
        RustServer {
            ip_addr: String::from(ip_addr),
        }
    }

    pub fn run(&self) -> JoinHandle<()> {
        let (proxy_sender, proxy_receiver) = channel::unbounded();

        let addr = self.ip_addr.clone();
        thread::spawn(move || {
            proxy::run(&addr, proxy_sender);
        });
        let mut controller = Controller::new();
        thread::spawn(move || loop {
            match proxy_receiver.try_recv() {
                Ok((c_type, client)) => match c_type {
                    ClientType::Bot => {
                        controller.add_client(client);
                        controller.send_message("{\"Bot\": \"Connected\"}")
                    }
                    ClientType::Controller => {
                        controller.add_supervisor(client);
                        controller.get_config_from_supervisor();
                    }
                },
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            }
            controller.update_clients();
            controller.update_games();
            thread::sleep(::std::time::Duration::from_millis(100));
        })
    }
}
#[pyclass(module = "rust_ac")]
pub(crate) struct PServer {
    server: Option<RustServer>,
}

#[pymethods]
impl PServer {
    #[new]
    #[args(args = "*")]
    fn new(args: &PyTuple) -> Self {
        match args.len() {
            0 => Self { server: None },
            1 => {
                if let Ok(f) = args.get_item(0).extract::<&str>() {
                    Self {
                        server: Some(RustServer::new(f)),
                    }
                } else {
                    Self { server: None }
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn run(&self) -> bool {
        match &self.server {
            Some(server) => server.run().join().is_ok(),
            None => false,
        }
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyBytes>(py) {
            Ok(s) => {
                self.server = deserialize(s.as_bytes()).unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyBytes::new(py, &serialize(&self.server).unwrap()).to_object(py))
    }
}
