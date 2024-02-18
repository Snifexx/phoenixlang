use std::{rc::Rc, collections::{HashMap, HashSet}, hash::BuildHasherDefault, path::PathBuf, str::FromStr, fs, sync::{Arc, Mutex, mpsc::{self, Sender}}, borrow::BorrowMut, thread::{self, JoinHandle}};
use ahash::AHasher;
use clap::error::ErrorKind;
use toml::Table;

use crate::{error::{PhoenixError, CompErrID}, compiler, strings::InternStrSync};

use self::{module::Module, scanner::Scanner, chunk::Chunk};

pub mod chunk;
pub mod scanner;
mod token;
pub mod module;

type AHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Compiler {
    modules: AHashMap<Arc<str>, Module>,
    strings: InternStrSync,
    transmitter: Option<Sender<JoinHandle<Result<(), Vec<PhoenixError>>>>>
}

impl Compiler {
    pub fn new(intern_str: InternStrSync) -> Self { Self { modules: AHashMap::default(), strings: intern_str, transmitter: None }}

    pub fn compile(path: PathBuf) -> Result<Chunk, Vec<PhoenixError>> {// Todo Temp chunk    
        macro_rules! config_err { ($($arg:tt)*) => { vec![PhoenixError::Config(format!($($arg)*))] }; }

        let feather_toml = path.join("Feather.toml");
        if !path.is_dir() || !feather_toml.is_file() { 
            return Err(vec![PhoenixError::Cli(ErrorKind::InvalidValue, format!("Given project must be a directory containing a Feather.toml"))]) }

        let confs = fs::read_to_string(feather_toml).unwrap();
        let confs = Table::from_str(&confs).map_err(|err| vec![PhoenixError::Cli(ErrorKind::Io, format!("{err}"))])?;
        if let Some(app_map) = confs.get("main") {
            if !app_map.is_table() { return Err(config_err!("'main' field in 'Feather.toml' isn't a table")) }

            let name = app_map.get("project-id").ok_or_else(|| config_err!("The 'main' table must specify a project-id"))?
                .as_str().ok_or_else(|| config_err!("The project-id in 'main' must be a string"))?.to_owned();
            let version = app_map.get("version").ok_or_else(|| config_err!("The 'main' table must specify a version"))?;

            let main = path.join("main.phx"); if !main.is_file() { return Err(config_err!("Missing main.phx in project directory")); }

            let mut intern_str = InternStrSync::new();

            let id = intern_str.intern_str(&name);
            let mut compiler = Arc::new(Mutex::new(Compiler::new(intern_str)));

            let (tx, rx) = mpsc::channel();
            
            let txx = tx.clone();
            let compiler_two = compiler.clone();
            let idd = id.clone();
            let main_thread = thread::spawn(move || {
                let mut module = Module::new(
                    Scanner::new(fs::read_to_string(main).map_err(|err| config_err!("{err}"))?).scan().map_err(|err| vec![err])?,
                    idd.clone(),
                    compiler_two.clone());
                module.compile(txx)?;

                let mut compiler_two = compiler_two.lock().unwrap();
                compiler_two.modules.insert(idd, module);
                Ok(())
            });
            
            tx.send(main_thread);
            drop(tx);
            
            for thread in rx {
                thread.join().unwrap()?;
            }
            
            let mut compiler = Arc::into_inner(compiler).unwrap().into_inner().unwrap();
            let chunk = compiler.modules.get_mut(&id).unwrap().chunk.take().unwrap();
            return Ok(chunk.build());
        }

        Ok(Chunk::new())
    }
}

