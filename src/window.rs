use eframe::epaint::Pos2;
use egui_node_graph::NodeId;
use enum_ordinalize::Ordinalize;

use std::fmt::Display;
use std::fs::{DirBuilder, File};
use std::hash::Hash;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use strum::{AsRefStr, EnumCount, EnumIter};

use crate::app::EditorStateType;
use crate::errors::AppError;
use crate::nodes::{
    add_node, inner_data_types::density_function::DensityFunctionType, node_types::NodeTemplate,
    GraphState,
};
use crate::ui::ComboBoxEnum;

// #[derive(Clone)]
pub struct Window {
    pub window_type: WindowType,
    pub name: String,
    pub filepath: PathBuf,
    pub namespace: String,
    /// empty if window is newly created
    pub file: Option<File>,
    pub state: EditorStateType,
    pub user_state: GraphState,
    pub dirty: bool,
    pub root_node: NodeId,
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window_type == other.window_type
            && self.name == other.name
            && self.namespace == other.namespace
    }
}
impl Eq for Window {}
impl Hash for Window {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.window_type.hash(state);
        self.name.hash(state);
        self.namespace.hash(state);
    }
}
impl PartialOrd for Window {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.window_type.partial_cmp(&other.window_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.namespace.partial_cmp(&other.namespace) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}

#[derive(
    PartialEq, PartialOrd, Copy, Clone, EnumCount, EnumIter, AsRefStr, Hash, Eq, Ordinalize, Debug,
)]
pub enum WindowType {
    #[strum(serialize = "Density Function")]
    DensityFunction,
    Noise,
    Biome,
}

impl WindowType {
    pub fn get_node_template(&self) -> NodeTemplate {
        match self {
            WindowType::DensityFunction => {
                NodeTemplate::DensityFunction(DensityFunctionType::Constant)
            }
            WindowType::Noise => NodeTemplate::Noise,
            WindowType::Biome => todo!(),
        }
    }
}

impl ComboBoxEnum for WindowType {}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} ({})",
            self.namespace,
            self.name,
            self.window_type.as_ref()
        )
    }
}

impl Window {
    pub fn new(
        filename: String,
        namespace: String,
        window_type: WindowType,
        project_path: &PathBuf,
    ) -> Self {
        let mut state = EditorStateType::default();
        let mut user_state = GraphState::default();

        let root_node = Self::add_default_node(&mut state, &mut user_state, window_type);
        let mut filepath = project_path.clone();
        filepath.push(&namespace);
        filepath.push(Self::path_from(window_type));
        filepath.push(&filename);
        filepath.set_extension("json");
        Self {
            window_type,
            name: filename,
            file: None,
            state,
            user_state,
            dirty: false,
            filepath,
            namespace,
            root_node,
        }
    }
    pub fn from_file(file: File, path: PathBuf, project_path: &PathBuf) -> Result<Self, AppError> {
        match path.strip_prefix(project_path) {
            Ok(filepath) => {
                let mut components = filepath.components();
                let namespace = components.next();
                let filename = components.as_path().file_stem();
                let window_type_dir = components.as_path().parent();
                if let (Some(namespace), Some(filename), Some(window_type_dir)) =
                    (namespace, filename, window_type_dir)
                {
                    if let (Some(window_type), Some(name), Some(namespace)) = (
                        Self::window_type_from(&window_type_dir.to_path_buf()),
                        filename.to_str(),
                        namespace.as_os_str().to_str(),
                    ) {
                        let mut state = EditorStateType::default();
                        let mut user_state = GraphState::default();
                        let root_node =
                            Self::add_default_node(&mut state, &mut user_state, window_type);

                        Ok(Self {
                            window_type,
                            name: name.to_string(),
                            filepath: filepath.to_path_buf(),
                            namespace: namespace.to_string(),
                            file: Some(file),
                            state,
                            user_state,
                            dirty: false,
                            root_node,
                        })
                    } else {
                        Err(AppError::FileStructure(path.into()))
                    }
                } else {
                    Err(AppError::FileStructure(path.into()))
                }
            }
            Err(x) => Err(AppError::FileRead(x.to_string())),
        }
    }

    fn add_default_node(
        state: &mut EditorStateType,
        user_state: &mut GraphState,
        window_type: WindowType,
    ) -> NodeId {
        add_node(
            state,
            user_state,
            NodeTemplate::Output(window_type),
            Pos2::new(600., 200.),
        )
    }
    pub fn save_to_file(&mut self, s: String) -> Result<(), std::io::Error> {
        if let Some(file) = self.file.as_mut() {
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;
            file.write_all(s.as_bytes())?;
        } else {
            let mut dir = self.filepath.clone();
            dir.pop();
            DirBuilder::new().recursive(true).create(dir)?;

            self.file = Some(File::create(&self.filepath).unwrap());
            self.file.as_mut().unwrap().write_all(s.as_bytes())?;
        }
        Ok(())
    }

    fn path_from(window_type: WindowType) -> String {
        match window_type {
            WindowType::DensityFunction => "worldgen/density_function",
            WindowType::Noise => "worldgen/noise",
            WindowType::Biome => "worldgen/biome",
        }
        .to_string()
    }
    /// returns `WindowType` from the exact part of path that is distinct for it.
    fn window_type_from(path: &PathBuf) -> Option<WindowType> {
        match path {
            x if x == Path::new("worldgen/density_function") => Some(WindowType::DensityFunction),
            x if x == Path::new("worldgen/noise") => Some(WindowType::Noise),
            _ => None,
        }
    }
}
