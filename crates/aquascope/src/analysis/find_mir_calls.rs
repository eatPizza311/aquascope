use flowistry::mir::utils::OperandExt;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_middle::mir::{
  visit::Visitor, Body, Location, Place, Terminator, TerminatorKind,
};
use rustc_span::Span;

pub trait FindCalls<'tcx> {
  fn find_calls(&self) -> HashMap<Location, CallInfo<'tcx>>;
}

#[derive(Debug)]
pub struct CallInfo<'tcx> {
  pub receiver_place: Place<'tcx>,
  pub fn_span: Span,
}

struct CallFinder<'tcx> {
  call_node_info: HashMap<Location, CallInfo<'tcx>>,
}

impl<'tcx> Visitor<'tcx> for CallFinder<'tcx> {
  fn visit_terminator(
    &mut self,
    terminator: &Terminator<'tcx>,
    location: Location,
  ) {
    if let TerminatorKind::Call {
      func: _,
      args,
      destination: _,
      target: _,
      cleanup: _,
      from_hir_call: _,
      fn_span,
    } = &terminator.kind
    {
      if !args.is_empty() {
        if let Some(receiver_place) = args[0].to_place() {
          // TODO: can we map calls more accurately to method calls?
          // this here is a rough approximation for demo purposes.
          self.call_node_info.insert(location, CallInfo {
            receiver_place,
            fn_span: *fn_span,
          });
        }
      }
    }
  }
}

impl<'tcx> FindCalls<'tcx> for Body<'tcx> {
  fn find_calls(&self) -> HashMap<Location, CallInfo<'tcx>> {
    let mut finder = CallFinder {
      call_node_info: HashMap::default(),
    };
    finder.visit_body(self);
    finder.call_node_info
  }
}
