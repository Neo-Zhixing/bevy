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
        self.0.get(&TypeId::of::<T>())?.as_any().downcast_ref()
    }
    /// Obtains a mutable reference to the stored value of a given type.
    pub fn get_mut<T: Send + Sync + Clone + 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>())?.as_any_mut().downcast_mut()
    }
    /// Checks if the map contains a value of the given type.
    pub fn has<T: Send + Sync + Clone + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
}
