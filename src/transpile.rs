// use std::{path::PathBuf, sync::Arc};
// use alloc::path::PathBuf;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::{EsVersion, Module, Program};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
// use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

use crate::error::WandResult;
use crate::string::WasmString;
use crate::writer::Writer;

pub struct TranspileModule {
    source: String,
    filename: String,
}

impl TranspileModule {
    pub fn parse(source: String, filename: String) -> WandResult<ParsedModule> {
        WasmString::from("transpiling").log();
        let module = TranspileModule { source, filename };

        module.parse_module()
    }

    pub fn parse_module(self) -> WandResult<ParsedModule> {
        WasmString::from("parsing").log();

        let map = Lrc::new(SourceMap::default());

        WasmString::from("creating source file").log();

        let fm = map.new_source_file(FileName::Custom(self.filename), self.source);

        let mut tsconfig = TsConfig::default();
        tsconfig.dynamic_import = true;

        let lex = Lexer::new(
            Syntax::Typescript(tsconfig),
            EsVersion::Es2020,
            StringInput::from(&*fm),
            None,
        );

        WasmString::from("created lexer").log();

        let mut parser = Parser::new_from(lex);

        WasmString::from("created parser").log();

        let module = parser.parse_module()?;

        WasmString::from(&format!("{:?}", module)).log();

        Ok(ParsedModule { module, map })
    }
}

pub struct ParsedModule {
    module: Module,
    map: Lrc<SourceMap>,
}

impl Into<String> for ParsedModule {
    fn into(self) -> String {
        let vec = self.to_buf();
        String::from_utf8(vec).unwrap()
    }
}

impl ParsedModule {
    pub fn to_buf(self) -> Vec<u8> {
        let writer = Writer::new(self.map);
        writer.emit(Program::Module(self.module))
    }
}
