fn main() {
    let question = requestty::Question::select("theme")
    .message("What do you want to do?")
    .choices(vec![
        "Foobar".into(),
        "Xyzzy".into(),
        "Plugh".into(),
        requestty::DefaultSeparator,
        "Cancel".into()
    ])
    .build();

println!("{:#?}", requestty::prompt_one(question));
}
