use std::rc::Rc;

use super::link::Link;

#[derive(Default)]
pub struct Blockchain {
    pub genesis: Rc<Link>,
    pub latest_links: Rc<Link>,
}
