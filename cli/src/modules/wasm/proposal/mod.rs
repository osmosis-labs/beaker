pub mod ops {
    use std::io::Read;
    use std::{fs::File, future::Future, io::BufReader};

    use anyhow::{Context as _, Result};
    use cosmos_sdk_proto::cosmos::gov::v1beta1::MsgSubmitProposal;
    use cosmrs::crypto::secp256k1::SigningKey;
    use cosmrs::{tx::Fee, Any};

    use crate::{
        framework::Context,
        modules::wasm::{response::StoreCodeResponse, WasmConfig},
        support::cosmos::Client,
    };

    pub trait MessageExt: prost::Message {
        /// Serialize this protobuf message as a byte vector.
        fn to_bytes(&self) -> Result<Vec<u8>>;
    }

    impl<M> MessageExt for M
    where
        M: prost::Message,
    {
        fn to_bytes(&self) -> Result<Vec<u8>> {
            let mut bytes = Vec::new();
            prost::Message::encode(self, &mut bytes)?;
            Ok(bytes)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn propose_store_code<'a, Ctx: Context<'a, WasmConfig>>(
        ctx: &Ctx,
        contract_name: &str,
        title: &str,
        description: &str,
        network: &str,
        fee: &Fee,
        timeout_height: &u32,
        signing_key: SigningKey,
    ) -> Result<StoreCodeResponse> {
        let global_config = ctx.global_config()?;
        let account_prefix = global_config.account_prefix().as_str();

        let client = Client::new(
            global_config
                .networks()
                .get(network)
                .with_context(|| format!("Unable to find network config: {network}"))?
                .to_owned(),
        )
        .to_signing_client(signing_key, account_prefix);

        let wasm = read_wasm(ctx, contract_name)?;
        let store_code_proposal = cosmrs::proto::cosmwasm::wasm::v1::StoreCodeProposal {
            title: title.to_string(),
            description: description.to_string(),
            run_as: client.signer_account_id().to_string(),
            wasm_byte_code: wasm,
            instantiate_permission: None, // TODO: add instantitate permission
        };

        // let deposit = vec!["0uosmo".parse::<CoinFromStr>()?.inner().into()];
        // TODO: add deposit input
        let deposit = vec![];

        let msg_submit_proposal = MsgSubmitProposal {
            content: Some(Any {
                type_url: "/cosmwasm.wasm.v1.StoreCodeProposal".to_owned(),
                value: store_code_proposal.to_bytes()?,
            }),
            initial_deposit: deposit,
            proposer: client.signer_account_id().to_string(),
        };

        let msg_submit_proposal = Any {
            type_url: "/cosmos.gov.v1beta1.MsgSubmitProposal".to_owned(),
            value: msg_submit_proposal.to_bytes()?,
        };

        block(async {
            let response = client
                .sign_and_broadcast(vec![msg_submit_proposal], fee.clone(), "", timeout_height)
                .await?;

            dbg!(&response);

            // let proposal_id: u64 = response
            //     .pick("proposal_deposit", "proposal_id")
            //     .to_string()
            //     .parse()?;

            // TODO: Create its own response type
            let store_code_response = StoreCodeResponse { code_id: 0 };

            // TODO: Update state
            // State::update_state_file(ctx.root()?, &|s: &State| -> State {
            //     s.update_code_id(network, contract_name, &code_id)
            // })?;
            // store_code_response.log();

            Ok(store_code_response)
        })
    }

    // TODO: Refactor this to share module
    fn block<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }
    fn read_wasm<'a, Ctx: Context<'a, WasmConfig>>(
        ctx: &Ctx,
        contract_name: &str,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let wasm_path = ctx
            .root()?
            .as_path()
            .join("artifacts")
            .join(format!("{contract_name}.wasm"));
        let wasm_path_str = &wasm_path.as_os_str().to_string_lossy();
        let f = File::open(&wasm_path).with_context(|| {
            format!(
            "`{wasm_path_str}` not found, please build and optimize the contract before store code`"
        )
        })?;
        let mut reader = BufReader::new(f);
        let mut wasm = Vec::new();
        reader.read_to_end(&mut wasm)?;
        Ok(wasm)
    }
}

pub mod entrypoint {
    use clap::Subcommand;
    use cosmrs::tx::Fee;

    use crate::{
        framework::Context,
        modules::wasm::{args::BaseTxArgs, WasmConfig},
    };

    #[derive(Subcommand, Debug)]
    pub enum ProposalCmd {
        /// Proposal for storing .wasm on chain for later initialization
        StoreCode {
            /// Name of the contract to store
            contract_name: String,

            /// Proposal title
            #[clap(long)]
            title: String,

            /// Proposal decsription
            #[clap(short, long)]
            description: String,

            #[clap(flatten)]
            base_tx_args: BaseTxArgs,
        },
    }

    pub fn execute<'a, Ctx: Context<'a, WasmConfig>>(
        ctx: Ctx,
        cmd: &ProposalCmd,
    ) -> Result<(), anyhow::Error> {
        match cmd {
            ProposalCmd::StoreCode {
                contract_name,
                title,
                description,
                base_tx_args,
            } => {
                let BaseTxArgs {
                    network,
                    signer_args,
                    gas_args,
                    timeout_height,
                }: &BaseTxArgs = base_tx_args;

                super::ops::propose_store_code(
                    &ctx,
                    contract_name,
                    title,
                    description,
                    network,
                    &Fee::try_from(gas_args)?,
                    timeout_height,
                    signer_args.private_key(&ctx.global_config()?)?,
                )?;
                Ok(())
            }
        }
    }
}
