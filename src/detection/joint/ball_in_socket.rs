use nalgebra::traits::transformation::Transform;
use detection::joint::anchor::Anchor;
use object::body::{RigidBody, SoftBody};

pub struct BallInSocket<N, LV, AV, M, II> {
    priv up_to_date: bool,
    priv anchor1:    Anchor<N, LV, AV, M, II>,
    priv anchor2:    Anchor<N, LV, AV, M, II>,
}

impl<N, LV: Eq, AV, M, II> BallInSocket<N, LV, AV, M, II> {
    pub fn up_to_date(&self) -> bool {
        self.up_to_date
    }

    pub fn anchor1<'r>(&'r self) -> &'r Anchor<N, LV, AV, M, II> {
        &self.anchor1
    }

    pub fn anchor2<'r>(&'r self) -> &'r Anchor<N, LV, AV, M, II> {
        &self.anchor2
    }

    pub fn set_local1(&mut self, local1: LV) {
        if local1 != self.anchor1.position {
            self.up_to_date = false;
            self.anchor1.position = local1
        }
    }

    pub fn set_local2(&mut self, local2: LV) {
        if local2 != self.anchor2.position {
            self.up_to_date = false;
            self.anchor2.position = local2
        }
    }
}

impl<N: Clone, LV: Clone, AV, M: Transform<LV>, II> BallInSocket<N, LV, AV, M, II> {
    pub fn anchor1_pos(&self) -> LV {
        match self.anchor1.body {
            Some(b) => {
                match *b {
                    RigidBody(rb) => rb.transform_ref().transform(&self.anchor1.position),
                    SoftBody(_)   => fail!("Not yet implemented.")
                }
            },
            None => self.anchor1.position.clone()
        }
    }

    pub fn anchor2_pos(&self) -> LV {
        match self.anchor2.body {
            Some(b) => {
                match *b {
                    RigidBody(rb) => rb.transform_ref().transform(&self.anchor2.position),
                    SoftBody(_)   => fail!("Not yet implemented.")
                }
            },
            None => self.anchor2.position.clone()
        }
    }
}