#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;
extern crate grpcio;
extern crate protobuf;

mod grpc;

use futures::prelude::*;
use grpc::audros as audros_types;
use grpc::audros_grpc as audros_srv;
use std::sync::Arc;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    loop {
        send_temperature(303.0);
        thread::sleep(time::Duration::from_secs(10));
    }
}

#[async]
fn send_temperature(temp_kelvin: f32) -> Result<audros_types::TemperatureResponse, grpcio::Error> {
    let env = Arc::new(grpcio::EnvBuilder::new().build());
    let ch = grpcio::ChannelBuilder::new(env)
        .connect(&format!("127.0.0.1:{}", 22222));
    let sensor_srv_client = audros_srv::SensorServiceClient::new(ch);

    let mut temperature = audros_types::Temperature::new();
    temperature.set_kelvin(temp_kelvin);
    await!(sensor_srv_client.put_temperature_async(&temperature).unwrap())
}
