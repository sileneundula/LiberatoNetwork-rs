use std::{path::{self, Path}, str::FromStr};

use fixedstr::{str64, str256};


pub struct OpenAddress {
    pub address: str256,
    pub network: str64,
    
    pub payload: Option<str256>,
}

impl OpenAddress {
    pub fn from_str<T: AsRef<str>>(s: T, network: Option<T>, payload: Option<T>) -> Result<Self,crate::errors::OpenAddressErrors> {
        let address = str256::from_str(s.as_ref());

        if address.is_err() {
            return Err(crate::errors::OpenAddressErrors::ParsingError)
        }

        if network.is_some() {
            let x = network.unwrap();
            let y = x.as_ref();
            let z = str64::from_str(y);

            if payload.is_some() {

                let x: &str = payload.unwrap().as_ref();
                let ya: Result<fixedstr::tstr, &str> = str256::from_str(x);

                if ya.is_ok() {
                    if z.is_ok() {
                        return Ok(Self {
                            address: address.unwrap(),
                            network: z.unwrap(),
                            payload: Some(ya.unwrap()),
                    })
                    else {
                        return Ok(Self {
                            address: address.unwrap(),
                            network: 
                        })
                    }
                }
                else {
                    return Ok(Self {
                        address: address.unwrap(),
                        network: 
                    })
                }


            }
        }
        else {
            
        }

            
        }
        else {
            return Err(crate::errors::OpenAddressErrors::ParsingError)
        }
        

    }
    pub fn from_file_path<T: AsRef<Path>>(path: T) {

    }
}