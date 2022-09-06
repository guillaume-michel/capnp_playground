fn main() {
    ::capnpc::CompilerCommand::new()
        .file("capnp/command.capnp")
        .run()
        .unwrap();
}
