mod root;
mod install;
mod stdin;
mod uninstall;

pub(crate) use {
    install::run_install, stdin::run_stdin, uninstall::run_uninstall,
};
