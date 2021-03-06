use std::collections::HashSet;
use todo_queue_lib::list::{Item, ItemDesc, ItemId, List, Status};
use todo_queue_lib::query::Filter;
use std::path::{Path, PathBuf};
use std::fs::File;
use serde_json;
use rand;
use error::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct NativeItem {
    name: String,
    description: String,
    status: Status,
    tags: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NativeList {
    items: Vec<(ItemId, NativeItem)>,
    path: PathBuf,
}

impl From<ItemDesc> for NativeItem {
    fn from(item: ItemDesc) -> Self {
        let ItemDesc {
            name,
            description,
            status,
            tags,
        } = item;

        Self {
            name,
            description,
            status,
            tags: tags.into_iter().collect(),
        }
    }
}

impl Item for NativeItem {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_status(&self) -> &Status {
        &self.status
    }
    fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    fn set_description(&mut self, description: &str) {
        self.description = description.into()
    }
    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
    fn set_tag(&mut self, tag: &str, set: bool) {
        if set {
            self.tags.insert(tag.into());
        } else {
            self.tags.remove(tag);
        }
    }
}

impl List for NativeList {
    type Item = NativeItem;

    fn add(&mut self, item: ItemDesc) -> ItemId {
        let item: Self::Item = item.into();
        let id = rand::random();

        self.items.push((id, item));
        id
    }

    fn remove(&mut self, target_id: &ItemId) {
        if let Some(idx) = self.items.iter().position(|me| me.0 == *target_id) {
            self.items.remove(idx);
        }
    }

    fn get(&self, target_id: &ItemId) -> Option<&Self::Item> {
        self.items
            .iter()
            .find(|me| me.0 == *target_id)
            .map(|&(_, ref item)| item)
    }

    fn get_mut(&mut self, target_id: &ItemId) -> Option<&mut Self::Item> {
        self.items
            .iter_mut()
            .find(|me| me.0 == *target_id)
            .map(|&mut (_, ref mut item)| item)
    }

    fn select(&self, filter: &Filter) -> Vec<ItemId> {
        self.items
            .iter()
            .filter(|&&(ref id, ref item)| filter.matches(id, item))
            .map(|m| m.0)
            .collect()
    }
}

impl NativeList {
    fn default_with_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            items: Vec::new(),
            path: path.into(),
        }
    }

    pub fn save_pretty(&self) -> Result<()> {
        let file = File::create(&self.path).context(ErrorKind::SaveList)?;
        serde_json::to_writer_pretty(file, self).context(ErrorKind::SaveList)?;
        Ok(())
    }

    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
            let list = Self::default_with_path(path);
            list.save_pretty().context(ErrorKind::SaveList)?;
            Ok(list)
        } else {
            let file = File::open(path).context(ErrorKind::LoadList)?;
            Ok(serde_json::from_reader(file).context(ErrorKind::LoadList)?)
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}
