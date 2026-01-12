use {
    crate::{InvocationInspectCallback, Mollusk},
    solana_program_runtime::invoke_context::{Executable, InvokeContext, RegisterTrace},
    solana_pubkey::Pubkey,
    solana_transaction_context::{InstructionAccount, InstructionContext},
    std::{cell::RefCell, rc::Rc},
};

#[derive(Default)]
pub struct DefaultDebuggerCallback {
    in_simulation: Rc<RefCell<bool>>,
}

impl InvocationInspectCallback for DefaultDebuggerCallback {
    // fn before_invocation(
    //     &self,
    //     mollusk: &Mollusk,
    //     program_id: &Pubkey,
    //     instruction_data: &[u8],
    //     instruction_accounts: &[InstructionAccount],
    //     invoke_context: &mut InvokeContext,
    // ) {
    //     if *self.in_simulation.borrow() {
    //         return;
    //     }
    //     let account_metas: Vec<_> = instruction_accounts
    //         .iter()
    //         .map(|ia| AccountMeta {
    //             pubkey: *invoke_context
    //                 .transaction_context
    //                 .get_key_of_account_at_index(ia.index_in_transaction)
    //                 .unwrap(),
    //             is_signer: ia.is_signer(),
    //             is_writable: ia.is_writable(),
    //         })
    //         .collect();

    //     let instruction = Instruction::new_with_bytes(*program_id,
    // instruction_data, account_metas);     let accounts: Vec<_> =
    // instruction_accounts         .iter()
    //         .map(|ia| {
    //             let pubkey = *invoke_context
    //                 .transaction_context
    //                 .get_key_of_account_at_index(ia.index_in_transaction)
    //                 .unwrap();
    //             let account_ref = invoke_context
    //                 .transaction_context
    //                 .accounts()
    //                 .try_borrow(ia.index_in_transaction)
    //                 .unwrap();
    //             let resulting_account = Account {
    //                 lamports: account_ref.lamports(),
    //                 data: account_ref.data().to_vec(),
    //                 owner: *account_ref.owner(),
    //                 executable: account_ref.executable(),
    //                 rent_epoch: account_ref.rent_epoch(),
    //             };
    //             (pubkey, resulting_account)
    //         })
    //         .collect();

    //     let fallback_accounts = mollusk.get_account_fallbacks(
    //         std::iter::once(&instruction.program_id),
    //         std::iter::once(&instruction),
    //         accounts.as_slice(),
    //     );

    //     let (sanitized_message, transaction_accounts) =
    // crate::compile_accounts::compile_accounts(         std::slice::from_ref(&
    // instruction),         accounts.iter(),
    //         &fallback_accounts,
    //     );

    //     let mut transaction_context =
    // mollusk.create_transaction_context(transaction_accounts);
    //     let sysvar_cache = mollusk.sysvars.setup_sysvar_cache(&accounts);

    //     *self.in_simulation.borrow_mut() = true;
    //     let _message_result = mollusk.process_transaction_message(
    //         &sanitized_message,
    //         &mut transaction_context,
    //         &sysvar_cache,
    //     );
    //     *self.in_simulation.borrow_mut() = false;
    // }
    fn before_invocation<'a, 'b>(
        &self,
        _mollusk: &Mollusk,
        _program_id: &Pubkey,
        _instruction_data: &'a [u8],
        _instruction_accounts: &[InstructionAccount],
        _invoke_context: &mut InvokeContext<'b, 'a>,
    ) {
        // if *self.in_simulation.borrow() {
        //     return;
        // }

        // *self.in_simulation.borrow_mut() = true;
        // let _invoke_result = if invoke_context.is_precompile(program_id) {
        //     invoke_context.process_precompile(
        //         program_id,
        //         &instruction_data,
        //         std::iter::once(instruction_data.as_ref()),
        //     )
        // } else {
        //     let mut compute_units_consumed = 0;
        //     let mut timings = ExecuteTimings::default();
        //     invoke_context.process_instruction(&mut compute_units_consumed,
        // &mut timings) };
        // *self.in_simulation.borrow_mut() = false;
    }

    fn after_invocation(&self, _: &Mollusk, invoke_context: &InvokeContext, _: bool) {
        if *self.in_simulation.borrow() {
            eprintln!("SIMULATION PROGRAM_ID ORDER COLLECTED: ");
            invoke_context.iterate_vm_traces(
                &|instruction_context: InstructionContext, _: &Executable, _: RegisterTrace| {
                    let program_id = instruction_context.get_program_key().unwrap();
                    eprintln!("program_id: {}", program_id);
                },
            );
        }
    }

    fn set_simulation_state(&self, state: bool) {
        eprintln!("SIMULATION MODE STATE: {}", state);
        *self.in_simulation.borrow_mut() = state;
    }
}
