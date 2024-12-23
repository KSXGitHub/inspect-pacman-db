use inspect_pacman_db::{
    query::{ForgetfulQuerier, Query},
    value::{Architecture, Description, FileName, Name},
};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    let querier = ForgetfulQuerier::new(TEXT);

    assert_eq!(querier.name(), Some(Name("gnome-shell")));

    assert_eq!(
        querier.file_name(),
        Some(FileName("gnome-shell-1:46.2-1-x86_64.pkg.tar.zst")),
    );

    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);

    assert_eq!(
        querier.description(),
        Some(Description("Next generation desktop shell")),
    );
}
