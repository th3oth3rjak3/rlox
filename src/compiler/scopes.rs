use crate::opcode::OP_POP;

use super::Compiler;

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Enters a new lexical block scope for the active function compiler.
    pub fn begin_scope(&mut self) {
        self.current_function_mut().begin_scope();
    }

    /// Exits the current lexical block scope for the active function compiler.
    pub fn end_scope(&mut self) {
        self.current_function_mut().end_scope();

        loop {
            let local_count = self.current_function().local_count();
            let scope_depth = self.current_function().scope_depth();

            if local_count == 0 {
                break;
            }

            let local = self.current_function().local_at(local_count - 1);

            if matches!(local.depth, Some(d) if d > scope_depth) {
                self.emit_byte(OP_POP);
                self.current_function_mut().pop_last_local();
            } else {
                break;
            }
        }
    }
}
