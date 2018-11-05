extern crate rayon;
extern crate rustc_data_structures;
extern crate slog;
pub extern crate swc_atoms;
pub extern crate swc_common;
pub extern crate swc_ecmascript;
pub extern crate swc_macros;

use slog::Logger;
use std::{path::Path, rc::Rc};
use swc_common::{
    errors::{Handler, SourceMapperDyn},
    SourceMap,
};
use swc_ecmascript::{
    ast::Module,
    parser::{PResult, Parser, Session as ParseSess, SourceFileInput},
};

pub struct Compiler {
    codemap: Rc<SourceMap>,
    threads: rayon::ThreadPool,
    logger: Logger,
    handler: Handler,
}

impl Compiler {
    pub fn new(
        logger: Logger,
        codemap: Rc<SourceMap>,
        handler: Handler,
        threads: rayon::ThreadPool,
    ) -> Self {
        Compiler {
            codemap,
            threads,
            logger,
            handler,
        }
    }

    /// TODO
    pub fn parse_js(&self, path: &Path) -> PResult<Module> {
        let fm = self.codemap.load_file(path).expect("failed to load file");

        Parser::new(
            ParseSess {
                handler: &self.handler,
                logger: &self.logger,
                cfg: Default::default(),
            },
            SourceFileInput::from(&*fm),
        )
        .parse_module()
    }
}