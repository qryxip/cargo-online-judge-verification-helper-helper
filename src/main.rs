#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]

use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

fn main() -> anyhow::Result<()> {
    let opt = match Opt::from_args() {
        Opt::OnlineJudgeVerificationHelperHelper(opt) | Opt::OjVerifyHelper(opt) => opt,
    };

    match opt {
        OptOnlineJudgeVerificationHelperHelper::GenDoc(
            OptOnlineJudgeVerificationHelperHelperGenDoc::Rust {
                crates_dir,
                manifest_path,
            },
        ) => cargo_online_judge_verification_helper_helper::gen_doc_rust(
            crates_dir.as_deref(),
            manifest_path.as_deref(),
        ),
        OptOnlineJudgeVerificationHelperHelper::GenDoc(
            OptOnlineJudgeVerificationHelperHelperGenDoc::OjVerify {
                md_dir,
                manifest_path,
            },
        ) => cargo_online_judge_verification_helper_helper::gen_doc_oj_verify(
            md_dir.as_deref(),
            manifest_path.as_deref(),
        ),
    }
}

#[derive(StructOpt)]
#[structopt(
    about,
    author,
    bin_name("cargo"),
    global_settings(&[AppSettings::DeriveDisplayOrder, AppSettings::UnifiedHelpMessage])
)]
enum Opt {
    OnlineJudgeVerificationHelperHelper(OptOnlineJudgeVerificationHelperHelper),
    OjVerifyHelper(OptOnlineJudgeVerificationHelperHelper),
}

#[derive(StructOpt)]
#[structopt(about, author)]
enum OptOnlineJudgeVerificationHelperHelper {
    /// Generate documentation
    GenDoc(OptOnlineJudgeVerificationHelperHelperGenDoc),
}

#[derive(StructOpt)]
#[structopt(about, author)]
enum OptOnlineJudgeVerificationHelperHelperGenDoc {
    /// Collect crate-level documentation
    Rust {
        /// Crates directory. Defaults to { workspace_root }/crates
        #[structopt(long, value_name("PATH"))]
        crates_dir: Option<PathBuf>,

        /// Path to Cargo.toml
        #[structopt(long, value_name("PATH"))]
        manifest_path: Option<PathBuf>,
    },

    /// Generate markdown files for oj-verify
    OjVerify {
        /// Markdown directory. Defaults to { workspace_root }/md
        #[structopt(long, value_name("PATH"))]
        md_dir: Option<PathBuf>,

        /// Path to Cargo.toml
        #[structopt(long, value_name("PATH"))]
        manifest_path: Option<PathBuf>,
    },
}
