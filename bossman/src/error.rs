pub trait OptionExt<T, E> {
    fn ctx(self, field: &'static str) -> Result<T, E>;
}

pub trait CollectionExt<T, E> {
    fn get_or_err(&self, field: &'static str, ctx: &'static str) -> Result<T, E>;
}
