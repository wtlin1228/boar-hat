use swc_core::{
    ecma::ast::*,
    ecma::visit::{Visit, VisitWith},
};

pub struct TopVisitor;

impl Visit for TopVisitor {
    fn visit_ident(&mut self, n: &Ident) {
        println!("{:?}", n.to_id());
        println!("{:?}\n\n", n);

        n.visit_children_with(self);
    }
}
