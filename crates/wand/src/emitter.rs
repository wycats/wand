use swc_common::errors::Emitter;

pub struct DummyEmitter {}

impl Emitter for DummyEmitter {
    fn emit(&mut self, db: &swc_common::errors::DiagnosticBuilder<'_>) {
        panic!("{:?}", db)
    }
}
