#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __callback_data_type {
    (boo $Type:ty) => {
        $Type // bool
    };
    (opt $Type:ty) => {
        Option<$Type>
    };
    (vec $Type:ty) => {
        Vec<$Type>
    };
    (set $Type:ty) => {
        ahash::AHashSet<$Type>
    };
    (map $Type:ty, $Type2:ty) => {
        ahash::AHashMap<$Type, $Type2>
    };
    (cus $Type:ty) => {
        $Type
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __call_callback {
    (boo, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {
        if $self.$param {
            ($callbacks.$param)($event_helper);
        }
    };
    (opt, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {
        if let Some(value) = $self.$param.clone() {
            ($callbacks.$param)($event_helper, value);
        }
    };
    (vec, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {{
        let vector = $self.$param.clone();
        if !vector.is_empty() {
            ($callbacks.$param)($event_helper, vector);
        }
    }};
    (set, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {{
        let set = $self.$param.clone();

        set.iter().for_each(|key| {
            $callbacks.$param.1.get(key).map(|func| func($event_helper));
        });

        if !set.is_empty() {
            ($callbacks.$param.0)($event_helper, set);
        }
    }};
    (map, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {{
        let map = $self.$param.clone();

        map.iter().for_each(|(key, value)| {
            $callbacks
                .$param
                .1
                .get(&key)
                .map(|func| func($event_helper, value.clone()));
        });

        if !map.is_empty() {
            ($callbacks.$param.0)($event_helper, map);
        }
    }};
    (cus, $self:ident, $event_helper:ident, $callbacks:ident, $param:ident) => {
        CallbackCallable::call_callbacks(&$self.$param, $event_helper, &$callbacks.$param);
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __get_value {
    ($(#[$outer_param:meta])*, boo, $param:ident, $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> bool {
            self.$param
        }
    };
    ($(#[$outer_param:meta])*, opt, $param:ident, $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> &Option<$Type> {
            self.$param.as_ref()
        }
    };
    ($(#[$outer_param:meta])*, vec, $param:ident, $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> &Vec<$Type> {
            &self.$param
        }
    };
    ($(#[$outer_param:meta])*, set, $param:ident, $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> &AHashSet<$Type> {
            &self.$param
        }
    };
    ($(#[$outer_param:meta])*, map, $param:ident, $Type:ty, $Type2:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> &AHashMap<$Type, $Type2> {
            &self.$param
        }

        $(#[$outer_param])*
        paste::paste! {
            pub fn [<$param _with_key>](&self, key: &$Type) -> &Option<$Type2> {
                self.$param.get(key)
            }
        }
    };
    ($(#[$outer_param:meta])*, cus, $param:ident, $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&self) -> &$Type {
            &self.$param
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __clear_value {
    (ign $type_kw:ident $self:ident $param:ident) => {};
    (clr boo $self:ident $param:ident) => {
        $self.$param = false;
    };
    (clr opt $self:ident $param:ident) => {
        $self.$param = None;
    };
    (clr vec $self:ident $param:ident) => {
        $self.$param.clear();
    };
    (clr set $self:ident $param:ident) => {
        $self.$param.clear();
    };
    (clr map $self:ident $param:ident) => {
        $self.$param.clear();
    };
    (clr cus $self:ident $param:ident) => {
        $self.$param.clear();
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __callback_type {
    (boo $Type:ty) => {
        CB<D>
    };
    (opt $Type:ty) => {
        CBI<D, $Type>
    };
    (vec $Type:ty) => {
        CBI<D, Vec<$Type>>
    };
    (set $Type:ty) => {
        (CBI<D, ahash::AHashSet<$Type>>, ahash::AHashMap<$Type, CB<D>>)
    };
    (map $Type:ty, $Type2:ty) => {
        (CBI<D, ahash::AHashMap<$Type, $Type2>>, ahash::AHashMap<$Type, CBI<D, $Type2>>)
    };
    (cus $Type:ty) => { <$Type as CallbackCallable<D>>::CallbackStruct };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __callback_type_default {
    (boo) => {
        |_| {}
    };
    (opt) => {
        |_, _| {}
    };
    (vec) => {
        |_, _| {}
    };
    (set) => {
        (|_, _| {}, Default::default())
    };
    (map) => {
        (|_, _| {}, Default::default())
    };
    (cus) => {
        Default::default()
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __define_callback_func {
    ($(#[$outer_param:meta])*, boo, $param:ident: $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&mut self, callback: CB<D>) {
            self.$param = callback;
        }
    };
    ($(#[$outer_param:meta])*, opt, $param:ident: $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&mut self, callback: CBI<D, $Type>) {
            self.$param = callback;
        }
    };
    ($(#[$outer_param:meta])*, vec, $param:ident: $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&mut self, callback: CBI<D, Vec<$Type>>) {
            self.$param = callback;
        }
    };
    ($(#[$outer_param:meta])*, set, $param:ident: $Type:ty) => {
        $(#[$outer_param])*
        pub fn $param(&mut self, callback: CBI<D, ahash::AHashSet<$Type>>) {
            self.$param.0 = callback;
        }

        $(#[$outer_param])*
        paste::paste! {
            pub fn [<$param _with_key>](&mut self, key: $Type, callback: CB<D>) {
                self.$param.1.insert(key, callback);
            }
        }
    };
    ($(#[$outer_param:meta])*, map, $param:ident: $Type:ty, $Type2:ty) => {
        $(#[$outer_param])*
        pub fn $param(&mut self, callback: CBI<D, ahash::AHashMap<$Type, $Type2>>) {
            self.$param.0 = callback;
        }

        $(#[$outer_param])*
        paste::paste! {
            pub fn [<$param _with_key>](&mut self, key: $Type, callback: CBI<D, $Type2>) {
                self.$param.1.insert(key, callback);
            }
        }
    };
    ($(#[$outer_param:meta])*, cus, $param:ident: $Type:ty) => {};
}

/// Creates a callback struct based on the captured struct's fields
/// and implements several things on the captured struct.
///
/// The struct has functions for adding callbacks.
///
/// Look at [callbacks](crate::callbacks) for examples.
///
/// Clear keywords (determines whether the field will be added to the `clear` function or not):
/// - `ign` (ignore)
/// - `clr` (clear)
///
/// Type keywords (wraps the given type and determines some macro functionality):
/// - `set` (AHashSet)
/// - `map` (AHashMap)
/// - `vec` (Vec)
/// - `opt` (Option)
/// - `boo` (no wrapping, used for boolean switches)
/// - `cus` (custom type, expected to implement the `call_callbacks(&self)` function)
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! create_callbacks {
    (
        $(#[$outer:meta])*
        $vis:vis struct $CallbackData:ident: $Callbacks:ident<D> {
            $(
                $(#[$outer_param:meta])*
                $clear_kw:ident $type_kw:ident $cbvis:vis $param:ident: $Type:ty$( => $Type2:ty)?
            ),*$(,)?
        }

        $($t:tt)*
    ) => {
        use crate::{event_helper::EventHelper, definitions::{CallbackCallable, CB, CBI}};

        $(#[$outer])*
        #[derive(Clone, Default)]
        $vis struct $CallbackData {
            $(
                $(#[$outer_param])*
                $cbvis $param: __callback_data_type!($type_kw $Type$(, $Type2)?)
            ),*
        }

        impl<D> CallbackCallable<D> for $CallbackData {
            type CallbackStruct = $Callbacks<D>;

            fn call_callbacks(&self, event_helper: &mut EventHelper<D>, callbacks: &$Callbacks<D>) {
                $(
                    $(#[$outer_param])*
                    __call_callback!($type_kw, self, event_helper, callbacks, $param);
                )*
            }
        }

        #[allow(dead_code)]
        impl $CallbackData {
            pub fn clear(&mut self) {
                $(
                    $(#[$outer_param])*
                    __clear_value!($clear_kw $type_kw self $param);
                )*
            }

            $(
                $(#[$outer_param])*
                pub fn $param(&self) -> &__callback_data_type!($type_kw $Type$(, $Type2)?) {
                    &self.$param
                }
            )*
        }

        #[allow(dead_code)]
        $vis struct $Callbacks<D> {
            $(
                $(#[$outer_param])*
                $cbvis $param: __callback_type!($type_kw $Type$(, $Type2)?)
            ),*
        }

        impl<D> Clone for $Callbacks<D> {
            fn clone(&self) -> Self {
                Self {
                    $(
                        $(#[$outer_param])*
                        $param: self.$param.clone()
                    ),*
                }
            }
        }

        impl<D> Default for $Callbacks<D> {
            fn default() -> Self {
                Self {
                    $(
                        $(#[$outer_param])*
                        $param: __callback_type_default!($type_kw)
                    ),*
                }
            }
        }

        #[allow(dead_code)]
        impl<D> $Callbacks<D> {
            $(
                __define_callback_func!($(#[$outer_param])*, $type_kw, $param: $Type$(, $Type2)?);
            )*
        }

        create_callbacks! { $($t)* }
    };
    () => {};
}
