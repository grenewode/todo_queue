pub mod mem {
    use list::{Item, ItemDesc, List, Status};
    use query::Query;

    #[derive(Debug)]
    pub struct MemItem {
        name: String,
        description: String,
        status: Status,
    }

    impl From<ItemDesc> for MemItem {
        fn from(desc: ItemDesc) -> Self {
            let ItemDesc {
                name,
                description,
                status,
            } = desc;
            Self {
                name,
                description,
                status,
            }
        }
    }

    impl Item for MemItem {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_description(&self) -> &str {
            &self.description
        }

        fn get_status(&self) -> &Status {
            &self.status
        }

        fn set_name(&mut self, name: &str) {
            self.name = name.into()
        }

        fn set_description(&mut self, description: &str) {
            self.description = description.into()
        }

        fn set_status(&mut self, status: Status) {
            self.status = status
        }
    }

    #[derive(Debug, Default)]
    pub struct MemList {
        items: Vec<MemItem>,
    }

    impl List for MemList {
        fn add(&mut self, item: ItemDesc) -> &mut Item {
            self.items.push(item.into());
            self.items.last_mut().unwrap()
        }

        fn select(&self, (filter, sort): Query) -> Vec<&Item> {
            let mut selected: Vec<_> = self.items
                .iter()
                .map(|item| item as &Item)
                .filter(|item| filter.matches(*item))
                .collect();
            selected.sort_by(|a, b| sort.cmp(*a, *b));
            selected
        }

        fn select_mut(&mut self, (filter, sort): Query) -> Vec<&mut Item> {
            let mut selected: Vec<_> = self.items
                .iter_mut()
                .map(|item| item as &mut Item)
                .filter(|item| filter.matches(*item))
                .collect();
            selected.sort_by(|a, b| sort.cmp(*a, *b));
            selected
        }
    }
}
