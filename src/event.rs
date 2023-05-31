use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn transfer_admin(env: &Env, current_admin: &Address, new_admin: &Address) {
    let topics = (Symbol::new(env, "transfer_admin"), current_admin);
    env.events().publish(topics, new_admin);
}
//
// pub(crate) fn add_relayers(env: &Env, addrs: &Vec<Address>) {
//     // let topics = (Symbol::new(env, "add_relayers"), env.caller());
//     // env.events().publish(topics, addrs);
// }
//
// pub(crate) fn remove_relayers(env: &Env, addrs: &Vec<Address>) {
//     // let topics = (Symbol::new(env, "remove_relayers"), addrs);
//     // env.events().publish(topics, addrs);
// }
