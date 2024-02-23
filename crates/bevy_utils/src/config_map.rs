use std::{any::{Any, TypeId}, collections::BTreeMap, ops::Deref};

/// Option map storing a single value of a given type.
#[derive(Default)]
pub struct ConfigMap(BTreeMap<TypeId, Box<dyn Any + Send + Sync>>);
impl ConfigMap {
    /// Creates a new empty `ConfigMap`.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Inserts a value into the map.
    pub fn insert<T: Send + Sync + 'static>(&mut self, option: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(option));
    }
    /// Obtains a reference to the stored value of a given type.
    pub fn get<'a, T: Send + Sync + 'static>(&'a self) -> Option<&'a T> {
        Some(self.0.get(&TypeId::of::<T>())?.downcast_ref::<T>().unwrap())
    }
    /// Obtains a mutable reference to the stored value of a given type.
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        Some(self.0.get_mut(&TypeId::of::<T>())?.downcast_mut::<T>().unwrap())
    }
    /// Obtains a mutable reference to the stored value of a given type.
    pub fn take<T: Send + Sync  + 'static>(&mut self) -> Option<T> {
        let item = self.0.remove(&TypeId::of::<T>())?.downcast::<T>().unwrap();
        Some(*item)
    }
    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<T: Send + Sync + 'static>(&mut self) -> ConfigMapEntry<T> {
        let entry = self.0.entry(TypeId::of::<T>());
        ConfigMapEntry {
            entry,
            _marker: std::marker::PhantomData,
        }
    }
    /// Checks if the map contains a value of the given type.
    pub fn has<T: Send + Sync + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
    /// Number of distinct types stored in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// A view into a single entry in a config map, which may either be vacant or occupied.
pub struct ConfigMapEntry<'a, T: Send + Sync + 'static> {
    entry: std::collections::btree_map::Entry<'a, TypeId, Box<dyn Any + Send + Sync>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Send + Sync + 'static> ConfigMapEntry<'a, T> {
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: T) -> &'a mut T {
        self.entry.or_insert_with(|| Box::new(default)).downcast_mut::<T>().unwrap()
    }
    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        self.entry.or_insert_with(|| Box::new(default())).downcast_mut::<T>().unwrap()
    }
    /// Provides in-place mutable access to an occupied entry before any potential inserts into the map.
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        let entry = self.entry.and_modify(|item| {
            let item = item.downcast_mut::<T>().unwrap();
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
