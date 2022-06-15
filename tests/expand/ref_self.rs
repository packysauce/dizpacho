struct RefString(String);

#[dizpacho::dizpacho]
impl RefString {
    #[dizpacho(AsRef<str>::as_ref)]
    #[dizpacho(std::ops::Deref<Target=str>::deref)]
    fn nab_string(&self) -> &str {
        &self.0
    }
}
