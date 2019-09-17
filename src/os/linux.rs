use ::netstat::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, SocketInfo};
use ::pnet::datalink::Channel::Ethernet;
use ::pnet::datalink::DataLinkReceiver;
use ::pnet::datalink::{self, NetworkInterface};
use ::std::fs::File;
use ::std::io::prelude::*;
use ::std::io::stdin;
use ::termion::event::Event;
use ::termion::input::TermRead;

pub struct KeyboardEvents;

impl Iterator for KeyboardEvents {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        match stdin().events().next() {
            Some(Ok(ev)) => Some(ev),
            _ => None,
        }
    }
}

pub fn get_datalink_channel(interface: &NetworkInterface) -> Box<DataLinkReceiver> {
    match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    }
}

pub fn get_interface(interface_name: &str) -> Option<NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == interface_name)
}

fn read_proc_comm(id: i32) -> std::io::Result<String> {
    let filename = format!("/proc/{}/comm", id);
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_process_name(id: i32) -> Option<String> {
    match read_proc_comm(id) {
        Ok(contents) => Some(contents),
        Err(_) => None,
    }
}

pub fn get_open_sockets() -> Vec<SocketInfo> {
    let af_flags = AddressFamilyFlags::IPV4;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    get_sockets_info(af_flags, proto_flags).unwrap_or_default()
}
