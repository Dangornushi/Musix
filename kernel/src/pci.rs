use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

const MAX_DEVICES: usize = 32;
const MAX_FUNCTIONS: usize = 8;

const INVALID_VENDOR_ID: u16 = 0xffff;

static PCI_CONFIG: spin::Mutex<PciConfig> = spin::Mutex::new(PciConfig::new());

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Full,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub header_type: u8,
}

const EMPTY_DEVICE: Device = Device {
    bus: 0xde,
    device: 0xad,
    function: 0xbe,
    header_type: 0xef,
};

//TODO should be fixed length vector or something linke that
pub struct PciDevices {
    devices: [Device; MAX_DEVICES],
    count: usize,
}

pub struct PciDevicesIter<'a> {
    devices: &'a [Device],
    index: usize,
}

impl PciDevices {
    const fn new() -> Self {
        Self {
            devices: [EMPTY_DEVICE; MAX_DEVICES],
            count: 0,
        }
    }

    pub fn add_device(&mut self, device: Device) -> Result<()> {
        if self.count > MAX_DEVICES {
            Err(Error::Full)
        } else {
            self.devices[self.count] = device;
            self.count += 1;
            Ok(())
        }
    }

    fn scan_function(&mut self, bus: u8, device: u8, function: u8) -> Result<()> {
        let header_type = read_header_type(bus, device, function);
        self.add_device(Device {
            bus,
            device,
            function,
            header_type,
        })?;
        let class_code = read_class_code(bus, device, function);
        let base = (class_code >> 24 & 0xff) as u8;
        let sub = (class_code >> 16 & 0xff) as u8;
        if base == 0x06 && sub == 0x04 {
            // PCI-PCI bridge
            let bus_numbers = read_bus_numbers(bus, device, function);
            let secondary_bus = ((bus_numbers >> 8) & 0xff) as u8;
            return self.scan_bus(secondary_bus);
        }
        Ok(())
    }

    fn scan_device(&mut self, bus: u8, device: u8) -> Result<()> {
        self.scan_function(bus, device, 0)?;
        if !is_single_function_device(read_header_type(bus, device, 0)) {
            for function in 1..MAX_FUNCTIONS {
                if read_vendor_id(bus, device, function as u8) != INVALID_VENDOR_ID {
                    self.scan_function(bus, device, function as u8)?;
                }
            }
        }
        Ok(())
    }

    fn scan_bus(&mut self, bus: u8) -> Result<()> {
        for device in 0..32 {
            if read_vendor_id(bus, device, 0) != INVALID_VENDOR_ID {
                self.scan_device(bus, device)?;
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> PciDevicesIter {
        PciDevicesIter {
            devices: &self.devices[..self.count],
            index: 0,
        }
    }
}

impl<'a> Iterator for PciDevicesIter<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.devices.len() {
            None
        } else {
            let r = self.devices[self.index];
            self.index += 1;
            Some(r)
        }
    }
}

struct PciConfig {
    address_port: PortWriteOnly<u32>,
    data_port: PortReadOnly<u32>,
}

impl PciConfig {
    const fn new() -> Self {
        Self {
            address_port: PortWriteOnly::new(0xcf8),
            data_port: PortReadOnly::new(0xcfc),
        }
    }

    fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
        1u32 << 31
            | u32::from(bus) << 16
            | u32::from(device) << 11
            | u32::from(function) << 8
            | u32::from(reg_addr & 0xfc)
    }

    pub fn read(&mut self, bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
        let addr = PciConfig::make_address(bus, device, function, reg_addr);
        unsafe {
            self.address_port.write(addr);
            self.data_port.read()
        }
    }
}

pub fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    (PCI_CONFIG.lock().read(bus, device, function, 0) & 0xffff) as u16
}

pub fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    (PCI_CONFIG.lock().read(bus, device, function, 0x0c) >> 16 & 0xff) as u8
}

pub fn read_class_code(bus: u8, device: u8, function: u8) -> u32 {
    PCI_CONFIG.lock().read(bus, device, function, 0x08)
}

pub fn read_bus_numbers(bus: u8, device: u8, function: u8) -> u32 {
    PCI_CONFIG.lock().read(bus, device, function, 0x18)
}

pub fn is_single_function_device(header_type: u8) -> bool {
    header_type & 0x80 == 0
}

pub fn scan_all_bus() -> Result<PciDevices> {
    let mut pci_devices = PciDevices::new();
    let header_type = read_header_type(0, 0, 0);
    if is_single_function_device(header_type) {
        pci_devices.scan_bus(0)?;
    }
    for function in 1..MAX_FUNCTIONS {
        let function = function as u8;
        if read_vendor_id(0, 0, function) != INVALID_VENDOR_ID {
            pci_devices.scan_bus(function)?;
        }
    }
    Ok(pci_devices)
}
