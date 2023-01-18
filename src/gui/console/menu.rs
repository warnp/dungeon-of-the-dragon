#[macro_export]
macro_rules! menu {
    [$($x:expr),*] => {
        Select::with_theme(&ColorfulTheme::default())
                .items(&$($x)*)
                .default(0)
                .interact_on_opt(&Term::stderr())?
    }
}