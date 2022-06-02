use std::path::PathBuf;
use std::env;
use std::fs;
use serde::{Deserialize};

const CONFIG_FILE: &str = "Settings.toml";


#[derive(Deserialize, Debug)]
pub struct Config{ 
    pub token_list: Vec<String>,  // 喜马拉雅token列表
}

// 从toml文件加载配置项
macro_rules! read_config { 
    ($struct: ident) => ({ 
        let current_dir = match env::current_dir(){
            Ok(path) => path,
            Err(_err) => PathBuf::from("."),
        };
        let config_path = current_dir.join(CONFIG_FILE).into_os_string();
        let config_str = match fs::read_to_string(&config_path) {
            Ok(str) => str,
            Err(err) => panic!("Fail to read config file(:{:?}), error:{}", config_path, err),
        };
        match toml::from_str::<$struct>(&config_str.as_str()){
            Ok(result) => result,
            Err(err) => panic!("Fail to parse config, error:{}", err),
        }
    })
}


lazy_static! { 
    pub static ref CONFIG:Config = read_config!(Config);
}

