use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // regenerate lalrpop parser
    lalrpop::process_root().unwrap();

    // gather build info
    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let git = GitclBuilder::default()
        .branch(true)
        .sha(false)
        .dirty(false)
        .build()?;

    // emit build data into the build environment
    Emitter::new()
        .add_instructions(&build)?
        .add_instructions(&git)?
        .emit()?;

    Ok(())
}
