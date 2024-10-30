use dioxus::prelude::*;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::error::Error;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub peripheral: Peripheral,
    pub name: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub enum BluetoothState {
    Scanning,
    Connected(BluetoothDevice),
    Disconnected,
    Error(String),
}

pub fn use_bluetooth() -> Signal<BluetoothState> {
    let state = use_signal(|| BluetoothState::Disconnected);
    
    use_effect(move || {
        spawn(async move {
            to_owned![state];
            let manager = Manager::new().await.unwrap();
            let adapters = manager.adapters().await.unwrap();
            let central = adapters.into_iter().next().unwrap();
            
            central.start_scan(ScanFilter::default()).await.unwrap();
            
            state.set(BluetoothState::Scanning);
            
            tokio::time::sleep(Duration::from_secs(10)).await;
            
            let peripherals = central.peripherals().await.unwrap();
            
            for peripheral in peripherals.iter() {
                if let Ok(Some(props)) = peripheral.properties().await {
                    if let Some(name) = props.local_name {
                        if name.contains("ELK-BLEDOM") {
                            let device = BluetoothDevice {
                                peripheral: peripheral.clone(),
                                name,
                                is_connected: false,
                            };
                            state.set(BluetoothState::Connected(device));
                            break;
                        }
                    }
                }
            }
        });
        
        ()
    });

    state
}

pub async fn send_command(device: &BluetoothDevice, command: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let peripheral = &device.peripheral;
    
    if !device.is_connected {
        peripheral.connect().await?;
    }
    
    let chars = peripheral.characteristics();
    let cmd_char = chars.iter()
        .find(|c| c.uuid.to_string() == "FFD9")
        .ok_or("Command characteristic not found")?;

    peripheral.write(&cmd_char, &command, btleplug::api::WriteType::WithoutResponse).await?;
    
    Ok(())
}
