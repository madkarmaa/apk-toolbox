fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = vergen_gitcl::BuildBuilder::all_build()?;
    let cargo = vergen_gitcl::CargoBuilder::all_cargo()?;
    let rustc = vergen_gitcl::RustcBuilder::all_rustc()?;
    let git = vergen_gitcl::GitclBuilder::all_git()?;

    vergen_gitcl::Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&git)?
        .emit()?;

    Ok(())
}
