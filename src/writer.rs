use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use swc_common::{chain, sync::Lrc, Globals, SourceMap, GLOBALS};
use swc_ecma_ast::Program;
use swc_ecma_codegen::Node;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};
use swc_ecma_transforms::{
    fixer,
    helpers::{self, HELPERS},
    typescript,
};
use swc_ecma_visit::FoldWith;

pub struct Writer {
    map: Lrc<SourceMap>,
}

impl Writer {
    pub fn new(map: Lrc<SourceMap>) -> Writer {
        Writer { map }
    }

    pub fn emit(&self, program: Program) -> Vec<u8> {
        let mut src_map_buf = vec![];
        let mut buf = vec![];

        {
            let writer = JsWriter::new(self.map.clone(), "\n", &mut buf, Some(&mut src_map_buf));
            let config = Config { minify: false };

            // let comments = KeepComments::default();

            let mut emitter = Emitter {
                cfg: config,
                comments: None,
                cm: self.map.clone(),
                wr: Box::new(writer),
            };

            let mut passes = chain!(
                helpers::inject_helpers(),
                typescript::strip(),
                // RewriteExt,
                fixer(None)
            );

            let program = GLOBALS.set(&Globals::new(), || {
                HELPERS.set(&helpers::Helpers::new(false), || {
                    program.fold_with(&mut passes)
                })
            });

            program.emit_with(&mut emitter).unwrap();
        }

        buf
    }
}
