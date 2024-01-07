use std::{any::{Any, TypeId}, collections::BTreeMap};


trait ConfigMapObj: Send + Sync {
    fn clone_dyn(&self) -> Box<dyn ConfigMapObj>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T> ConfigMapObj for T where T: Clone + Send + Sync + 'static {
    fn clone_dyn(&self) -> Box<dyn ConfigMapObj> {
        let this = <Self as Clone>::clone(self);
        Box::new(this)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
impl Clone for Box<dyn ConfigMapObj> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

/// Option map storing a single value of a given type.
#[derive(Default, Clone)]
pub struct ConfigMap(BTreeMap<TypeId, Box<dyn ConfigMapObj>>);
impl ConfigMap {
    /// Creates a new empty `ConfigMap`.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Inserts a value into the map.
    pub fn insert<T: Send + Sync + Clone + 'static>(&mut self, option: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(option));
    }
    /// Obtains a reference to the stored value of a given type.
    pub fn get<'a, T: Send + Sync + Clone + 'static>(&'a self) -> Option<&'a T> {
        let item: &dyn ConfigMapObj = self.0.get(&TypeId::of::<T>())?.as_ref();
        let item = item.as_any();
        let boxed_item: &T = item.downcast_ref::<T>().unwrap();
        Some(boxed_item)
    }
    /// Obtains a mutable reference to the stored value of a given type.
    pub fn get_mut<T: Send + Sync + Clone + 'static>(&mut self) -> Option<&mut T> {
        let item: &mut dyn ConfigMapObj = self.0.get_mut(&TypeId::of::<T>())?.as_mut();
        let item = item.as_any_mut();
        let boxed_item: &mut T = item.downcast_mut::<T>().unwrap();
        Some(boxed_item)
    }
    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<T: Send + Sync + Clone + 'static>(&mut self) -> ConfigMapEntry<T> {
        let entry = self.0.entry(TypeId::of::<T>());
        ConfigMapEntry {
            entry,
            _marker: std::marker::PhantomData,
        }
    }
    /// Checks if the map contains a value of the given type.
    pub fn has<T: Send + Sync + Clone + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
    /// Number of distinct types stored in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// A view into a single entry in a config map, which may either be vacant or occupied.
pub struct ConfigMapEntry<'a, T: Send + Sync + Clone + 'static> {
    entry: std::collections::btree_map::Entry<'a, TypeId, Box<dyn ConfigMapObj>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Send + Sync + Clone + 'static> ConfigMapEntry<'a, T> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: T) -> &'a mut T {
        self.entry.or_insert_with(|| Box::new(default)).as_mut().as_any_mut().downcast_mut::<T>().unwrap()
    }
    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        self.entry.or_insert_with(|| Box::new(default())).as_mut().as_any_mut().downcast_mut::<T>().unwrap()
    }
    /// Provides in-place mutable access to an occupied entry before any potential inserts into the map.
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        let entry = self.entry.and_modify(|item| {
            let item = item.as_mut().as_any_mut().downcast_mut::<T>().unwrap();
            f(item);
        });
        ConfigMapEntry {
            entry,
            _marker: std::marker::PhantomData,
        }
    }
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        self.or_insert_with(Default::default)
    }
}
