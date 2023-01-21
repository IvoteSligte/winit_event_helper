#![allow(dead_code)]

use std::{
    borrow::Borrow,
    collections::hash_map::{Drain, Entry, Iter, IterMut, Keys, Values, ValuesMut},
    hash::Hash,
    ops::{Index, IndexMut},
};

use ahash::AHashMap;
use winit::{event::DeviceId, window::WindowId};

use crate::{
    callbacks::{
        device::{DeviceCallbackData, DeviceCallbacks},
        window::{WindowCallbackData, WindowCallbacks},
    },
    CallbackCallable, EventHelper,
};

#[derive(PartialEq, Eq)]
/// Copied (and slightly altered) from the [defaultmap](https://crates.io/crates/defaultmap) crate!
/// As such, the [defaultmap::DefaultHashMap](https://docs.rs/defaultmap/0.5.0/defaultmap/struct.DefaultHashMap.html) documentation applies.
///
/// An `AHashMap` that returns a default when keys are accessed that are not present.
pub struct DefaultAHashMap<K: Eq + Hash, V: Default> {
    pub(crate) map: AHashMap<K, V>,
    default: V,
}

impl<D> CallbackCallable<D> for DefaultAHashMap<DeviceId, DeviceCallbackData> {
    type CallbackStruct = DefaultAHashMap<DeviceId, DeviceCallbacks<D>>;

    fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &Self::CallbackStruct) {
        self.map
            .iter()
            .filter_map(|(key, device_callback_data)| {
                callbacks.map.get(key).zip(Some(device_callback_data))
            })
            .for_each(|(device_callbacks, device_callback_data)| {
                device_callback_data.call_callbacks(event_helper, device_callbacks);
            });
    }
}

impl<D> CallbackCallable<D> for DefaultAHashMap<WindowId, WindowCallbackData> {
    type CallbackStruct = DefaultAHashMap<WindowId, WindowCallbacks<D>>;

    fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &Self::CallbackStruct) {
        self.map
            .iter()
            .filter_map(|(key, window_callback_data)| {
                callbacks.map.get(key).zip(Some(window_callback_data))
            })
            .for_each(|(window_callbacks, window_callback_data)| {
                window_callback_data.call_callbacks(event_helper, window_callbacks);
            });
    }
}

impl<K: Eq + Hash, V: Default> Default for DefaultAHashMap<K, V> {
    fn default() -> Self {
        Self {
            map: AHashMap::default(),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default> From<AHashMap<K, V>> for DefaultAHashMap<K, V> {
    fn from(map: AHashMap<K, V>) -> Self {
        Self {
            map,
            default: Default::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default> DefaultAHashMap<K, V> {
    pub fn get<Q, QB: Borrow<Q>>(&self, key: QB) -> &V
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key.borrow()).unwrap_or(&self.default)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        self.map.entry(key).or_default()
    }
}

impl<'a, K: Eq + Hash, KB: Borrow<K>, V: Default> Index<KB> for DefaultAHashMap<K, V> {
    type Output = V;

    fn index(&self, index: KB) -> &V {
        self.get(index)
    }
}

impl<K: Eq + Hash, V: Default> IndexMut<K> for DefaultAHashMap<K, V> {
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut V {
        self.get_mut(index)
    }
}

impl<K: Eq + Hash, V: Default> FromIterator<(K, V)> for DefaultAHashMap<K, V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            map: AHashMap::from_iter(iter),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default> DefaultAHashMap<K, V> {
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional)
    }
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit()
    }
    #[inline]
    pub fn keys(&self) -> Keys<K, V> {
        self.map.keys()
    }
    #[inline]
    pub fn values(&self) -> Values<K, V> {
        self.map.values()
    }
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        self.map.values_mut()
    }
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.map.iter_mut()
    }
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self.map.entry(key)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        self.map.drain()
    }
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }
    #[inline]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.contains_key(k)
    }
    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.remove(k)
    }
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.map.retain(f)
    }
}
