use uuid::Uuid;

use query::Filter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum Status {
    Waiting,
    Queuing,
    Working,
    Completed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Waiting
    }
}

pub struct ItemDesc {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub tags: Vec<String>,
}

pub trait Item {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_status(&self) -> &Status;
    fn has_tag(&self, tag: &str) -> bool;

    fn set_name(&mut self, name: &str);
    fn set_description(&mut self, description: &str);
    fn set_status(&mut self, status: Status);
    fn set_tag(&mut self, tag: &str, set: bool);
}

impl ItemDesc {
    pub fn new<N, D, T>(name: N, description: D, tags: T) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        T: IntoIterator,
        T::Item: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            status: Status::default(),
            tags: tags.into_iter().map(T::Item::into).collect(),
        }
    }
}

impl Item for ItemDesc {
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
        self.tags.contains(&tag.into())
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    fn set_description(&mut self, description: &str) {
        self.description = description.into();
    }
    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
    fn set_tag(&mut self, tag: &str, set: bool) {
        if set {
            // TODO: add sorting...?
            if self.has_tag(tag) {
                self.tags.push(tag.into())
            }
        } else {
            if let Some(pos) = self.tags.iter().position(|s| s == &tag) {
                self.tags.remove(pos);
            }
        }
    }
}

impl From<String> for ItemDesc {
    fn from(string: String) -> Self {
        let mut string = string;

        let mut tags = Vec::new();
        let mut name_description = String::new();

        let mut chars = string.chars();

        while let Some(c) = chars.next() {
            if c == '#' {
                let mut tag = String::new();
                while let Some(t) = chars.next() {
                    if t.is_whitespace() || t == ':' {
                        name_description.push(t);
                        break;
                    } else {
                        tag.push(t)
                    }
                }
                tags.push(tag)
            } else {
                name_description.push(c)
            }
        }

        if let Some(name_sep_idx) = name_description.find(":") {
            let name = name_description.split_off(name_sep_idx);
            ItemDesc::new(name, name_description, tags)
        } else {
            ItemDesc::new(name_description, "", tags)
        }
    }
}

pub type ItemId = Uuid;

pub trait List {
    type Item: ?Sized + Item;

    fn add(&mut self, item: ItemDesc) -> ItemId;
    fn remove(&mut self, item: &ItemId);

    fn get(&self, item: &ItemId) -> Option<&Self::Item>;
    fn get_mut(&mut self, item: &ItemId) -> Option<&mut Self::Item>;

    fn select(&self, &Filter) -> Vec<ItemId>;
}
