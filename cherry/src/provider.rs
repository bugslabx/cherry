use sqlx::{Database, Encode, Type};

use crate::sql::end::section::EndSection;
use crate::sql::set_clause::SetSection;
use crate::sql::where_clause::condition::Condition;

pub trait WhereProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_where(&mut self, c: Condition<'a>);

    fn surround_where(&mut self);

    fn take_surround(&mut self) -> Vec<Condition<'a>>;
}

pub trait SetValueProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_set_section(&mut self, section: SetSection<'a>);
}

pub trait EndProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_end_section(&mut self, section: EndSection<'a>);
}