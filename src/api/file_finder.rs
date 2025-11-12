use std::{env, fs};

pub struct FileFinder{
    pub paths:Vec<String>,
    pub prefixes:Vec<String>,
    pub suffixes:Vec<String>,
}

impl Default for FileFinder{
    fn default() -> Self {
        Self{
            paths:Vec::default(),
            suffixes:Vec::default(),
            prefixes:Vec::default()
        }
    }
}
impl FileFinder{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn from_env()->Self{
        let prefixes = Vec::<String>::default();
        let paths:Vec<String> = env::var("PATH").unwrap().split(
            cfg_if::cfg_if! {
                if #[cfg(windows)]{
                    ';'
                }
                else{
                    ':'
                }
                }
            ).collect();

        let suffixes:Vec<String> =
        cfg_if::cfg_if! {
                if #[cfg(windows)]{
                    env::var("PATHEXT").unwrap().split(';').map(|x| [x,x.to_ascii_lowercase()]).flatten().collect()
                }
            else{
                Vec::default()
            }
        };

        FileFinder{
            paths,
            prefixes,
            suffixes
        }
    }

    pub fn search(&self,target:&str) -> Vec<String>{
        let mut result:Vec<String> = Vec::default();
        for path in self.paths{
            for prefix in [String::default()].iter().chain(self.prefixes.iter()){
                for suffix in [String::default()].iter().chain(self.suffixes.iter()){
                    let target = format!("{}/{}{}{}",path,prefix,target,suffix);

                    if fs::exists(&target).unwrap(){
                        result.push(target)
                    }
                }
            }
        }
        result
    }

}