use super::{common::query_raw_text_from, QueryMut};
use crate::field::{FieldName, ParsedField};
use core::str::Lines;

/// [Query](QueryMut) with a cache.
#[derive(Debug, Clone)]
pub struct MemoQuerier<'a> {
    text: &'a str,
    lines: Lines<'a>,
    cache: Cache<'a>,
}

impl<'a> MemoQuerier<'a> {
    /// Query the `text` with a cache.
    pub fn new(text: &'a str) -> Self {
        MemoQuerier {
            text,
            lines: text.lines(),
            cache: Cache::default(),
        }
    }
}

impl<'a> QueryMut<'a> for MemoQuerier<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        if let Some(value) = self.cache.get(field.name()) {
            return value;
        }
        let value = query_raw_text_from(self.lines.by_ref(), self.text, field);
        self.cache.add(field.name(), value);
        value
    }
}

macro_rules! def_cache {
    ($(
        $(#[$attrs:meta])*
        $field:ident $(,)? $(;)?
    )*) => {
        #[derive(Debug, Clone, Copy)]
        enum CacheErr {
            OccupiedWithNone,
            Unoccupied,
        }

        #[derive(Debug, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        struct Cache<'a> {$(
            $(#[$attrs])*
            $field: Result<&'a str, CacheErr>, // Result<&str, CacheErr> uses less memory than Option<Option<&str>>
        )*}

        impl<'a> Cache<'a> {
            fn get(&self, field: &FieldName) -> Option<Option<&'a str>> {
                match field {$(
                    FieldName::$field => match self.$field {
                        Ok(value) => Some(Some(value)),
                        Err(CacheErr::OccupiedWithNone) => Some(None),
                        Err(CacheErr::Unoccupied) => None,
                    },
                )*}
            }

            fn add(&mut self, field: &FieldName, value: Option<&'a str>) {
                match (field, value) {$(
                    (FieldName::$field, Some(value)) => self.$field = Ok(value),
                    (FieldName::$field, None) => self.$field = Err(CacheErr::OccupiedWithNone),
                )*}
            }
        }

        impl<'a> Default for Cache<'a> {
            fn default() -> Self {
                Cache {$(
                    $field: Err(CacheErr::Unoccupied),
                )*}
            }
        }

        #[test]
        fn test_cache_fields() {$({
            use pretty_assertions::assert_eq;
            let field = &FieldName::$field;
            let mut cache = Cache::default();
            assert_eq!(cache.get(field), None);
            cache.add(field, None);
            assert_eq!(cache.get(field), Some(None));
            cache.add(field, Some("foo"));
            assert_eq!(cache.get(field), Some(Some("foo")));
        })*}
    };
}

def_cache!(
    FileName Name Base Version Description Groups
    CompressedSize InstalledSize Md5Sum Sha256Sum PgpSignature
    Url License Arch BuildDate Packager
    Depends CheckDepends MakeDepends OptDepends Provides Conflicts Replaces
);
