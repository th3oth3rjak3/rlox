use super::Compiler;

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Enters a new lexical block scope for the active function compiler.
    pub fn begin_scope(&mut self) {
        self.current_function_mut().begin_scope();
    }

    /// Exits the current lexical block scope for the active function compiler.
    pub fn end_scope(&mut self) {
        self.current_function_mut().end_scope();
    }
}
