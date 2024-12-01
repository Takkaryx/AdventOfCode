use dbg_pls::DebugPls;

#[derive(Clone, Debug, DebugPls)]
pub struct Cfiles {
    name: String,
    size: f64,
}

#[derive(Clone, Debug, DebugPls)]
pub struct Cdir {
    parent: Option<Box<Cdir>>,
    name: String,
    subdirs: Vec<&mut Cdir>,
    files: Vec<Cfiles>,
}

impl Cdir {
    fn new(name: String) -> Self {
        Cdir {
            parent: None,
            name,
            subdirs: Vec::new(),
            files: Vec::new(),
        }
    }
}
