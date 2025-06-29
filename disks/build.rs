use vergen_git2::Git2Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rebuild if i18n files change
    println!("cargo:rerun-if-changed=i18n");

    // Emit version information (if not cached by just vendor)
    let git2_builder = Git2Builder::default()
        .commit_date(true)
        .sha(true)
        .build()?;

    vergen::Emitter::default()
        .add_instructions(&git2_builder)?
        .fail_on_error()
        .emit()?;


    println!("cargo:rerun-if-env-changed=VERGEN_GIT_COMMIT_DATE");
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_SHA");

    Ok(())
}
