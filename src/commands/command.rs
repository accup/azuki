pub trait Command {
    type Args;

    fn execute(&self, args: &Self::Args) -> std::io::Result<()>;
}
