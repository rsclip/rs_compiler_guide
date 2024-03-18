pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize) -> String;
}