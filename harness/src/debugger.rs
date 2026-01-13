use {
    crate::{register_tracing::compute_hash, InvocationInspectCallback, Mollusk},
    solana_program_runtime::invoke_context::InvokeContext,
    solana_pubkey::Pubkey,
    solana_transaction_context::InstructionAccount,
    std::{fs::File, io::Write},
};

const DEFAULT_PATH: &str = "target/sbf/trace";

pub struct DefaultDebuggerCallback {
    pub sbf_trace_dir: String,
}

impl DefaultDebuggerCallback {
    pub fn handler(
        &self,
        mollusk: &Mollusk,
        program_id: &Pubkey,
        _instruction_data: &[u8],
        instruction_accounts: &[InstructionAccount],
        invoke_context: &InvokeContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let sbf_trace_dir = current_dir.join(&self.sbf_trace_dir);
        std::fs::create_dir_all(&sbf_trace_dir)?;

        let base_fname = sbf_trace_dir.join("debug_session");
        let mut debug_session_file = File::create(base_fname.with_extension("log"))?;

        std::iter::once(program_id)
            .chain(instruction_accounts.iter().map(|ia| {
                invoke_context
                    .transaction_context
                    .get_key_of_account_at_index(ia.index_in_transaction)
                    .unwrap()
            }))
            .for_each(|pubkey| {
                if let Some(elf_data) = mollusk.program_cache.get_program_elf_bytes(&pubkey) {
                    let _ = debug_session_file.write(
                        format!("{}={}", pubkey, compute_hash(elf_data.as_slice())).as_bytes(),
                    );
                }
            });

        Ok(())
    }
}

impl Default for DefaultDebuggerCallback {
    fn default() -> Self {
        Self {
            // User can override default path with `SBF_TRACE_DIR` environment variable.
            sbf_trace_dir: std::env::var("SBF_TRACE_DIR").unwrap_or(DEFAULT_PATH.to_string()),
        }
    }
}

impl InvocationInspectCallback for DefaultDebuggerCallback {
    fn before_invocation(
        &self,
        mollusk: &Mollusk,
        program_id: &Pubkey,
        instruction_data: &[u8],
        instruction_accounts: &[InstructionAccount],
        invoke_context: &InvokeContext,
    ) {
        let _ = self.handler(
            mollusk,
            program_id,
            instruction_data,
            instruction_accounts,
            invoke_context,
        );
    }

    fn after_invocation(&self, _: &Mollusk, _: &InvokeContext, _: bool) {}
}
