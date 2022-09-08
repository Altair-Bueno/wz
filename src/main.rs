mod builder;
mod run;
mod sheath;

fn main() {
    let config = wz_conf::load();
    run::run(config)
}
