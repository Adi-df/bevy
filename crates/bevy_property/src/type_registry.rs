use crate::{DeserializeProperty, Property};
use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct PropertyTypeRegistry {
    pub registrations: HashMap<String, PropertyTypeRegistration>,
    pub short_names: HashMap<String, String>,
}

impl PropertyTypeRegistry {
    pub fn register<T>(&mut self)
    where
        T: Property + DeserializeProperty,
    {
        let registration = PropertyTypeRegistration::of::<T>();
        self.short_names
            .insert(registration.short_name.to_string(), registration.name.to_string());
        self.registrations
            .insert(registration.name.to_string(), registration);
    }

    pub fn get(&self, type_name: &str) -> Option<&PropertyTypeRegistration> {
        self.registrations.get(type_name)
    }

    pub fn get_short(&self, short_type_name: &str) -> Option<&PropertyTypeRegistration> {
        self.short_names
            .get(short_type_name)
            .and_then(|name| self.registrations.get(name))
    }
}

#[derive(Clone)]
pub struct PropertyTypeRegistration {
    pub ty: TypeId,
    pub deserialize: fn(
        deserializer: &mut dyn erased_serde::Deserializer,
        property_type_registry: &PropertyTypeRegistry,
    ) -> Result<Box<dyn Property>, erased_serde::Error>,
    pub short_name: &'static str,
    pub name: &'static str,
}

impl PropertyTypeRegistration {
    pub fn of<T: Property + DeserializeProperty>() -> Self {
        let ty = TypeId::of::<T>();
        Self {
            ty,
            deserialize: |deserializer: &mut dyn erased_serde::Deserializer, property_type_registry: &PropertyTypeRegistry| {
                T::deserialize(deserializer, property_type_registry)
            },
            name: std::any::type_name::<T>(),
            short_name: std::any::type_name::<T>().split("::").last().unwrap(),
        }
    }
}