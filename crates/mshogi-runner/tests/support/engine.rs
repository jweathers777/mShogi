use mshogi_runner::EngineConfig;
pub fn fixture(path: &str, mode: &str) -> EngineConfig {
    EngineConfig {
        path: path.into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("verification fixture".into()),
        deterministic: true,
    }
}
