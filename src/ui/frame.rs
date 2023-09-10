use crate::util::{Pos, Size};

pub struct Frame {
    pub title: String
}

/*
Synth -> [.., Parent]
Parent -> [entity, from_child]
Frame -> [..]
*/

// parent.from_child
// parent.transform_child