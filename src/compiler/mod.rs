use std::{rc::Rc, collections::{HashMap, HashSet}, hash::BuildHasherDefault, path::PathBuf, str::FromStr, fs, sync::{Arc, Mutex}, borrow::BorrowMut};
use ahash::AHasher;
use clap::error::ErrorKind;
use toml::Table;

use crate::{error::{PhoenixError, CompErrID}, compiler};

use self::{module::Module, scanner::Scanner, chunk::Chunk};

pub mod chunk;
pub mod scanner;
mod token;
pub mod module;

type AHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Compiler {
    modules: AHashMap<Rc<String>, Module>,
    interned_str: HashSet<Rc<String>, BuildHasherDefault<AHasher>>,
}

impl Compiler {
    fn intern_str_ref(interned: &mut HashSet<Rc<String>, BuildHasherDefault<AHasher>>, str: &String) -> Rc<String> {
        match interned.get(str) { Some(str) => str.clone(), None => {interned.insert(Rc::new(str.clone())); interned.get(str).unwrap().clone()} } }
    fn intern_str(interned: &mut HashSet<Rc<String>, BuildHasherDefault<AHasher>>, str: String) -> Rc<String> {
        match interned.get(&str) { Some(str) => str.clone(), None => { let rc = Rc::new(str); interned.insert(rc.clone()); rc } } }

    pub fn new() -> Self { Self { modules: AHashMap::default(), interned_str: Default::default() }}

    pub fn compile(path: PathBuf) -> Result<Chunk, Vec<PhoenixError>> {//Temp chunk    
        macro_rules! config_err { ($($arg:tt)*) => { vec![PhoenixError::Config(format!($($arg)*))] }; }

        let feather_toml = path.join("Feather.toml");
        println!("{}", feather_toml.display());
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

            let mut compiler = Compiler::new();
            let id = Compiler::intern_str_ref(&mut compiler.interned_str, &name);
            let compiler = Module::new(
                Scanner::new(fs::read_to_string(main).map_err(|err| config_err!("{err}"))?).scan().map_err(|err| vec![err])?, 
                id,
                Arc::new(Mutex::new(compiler))
            ).compile()?;
            let mut compiler = compiler.lock().unwrap();

            let id = &Compiler::intern_str(&mut (*compiler).interned_str, name);
            let chunk = compiler.modules.get_mut(&*id).unwrap().chunk.take().unwrap();
            return Ok(chunk.build());
        }

        Ok(Chunk::new())
    }
}

