pub fn verify_wrapper(
    protocol: mshogi_runner::experiments::ExternalProtocol,
    config: impl serde::Serialize,
) {
    use mshogi_runner::experiments::{ExternalReport, run_external};
    let wrapped = run_external(protocol, &serde_json::to_string(&config).unwrap()).unwrap();
    wrapped.validate().unwrap();
    let mut decoded: ExternalReport =
        serde_json::from_str(&serde_json::to_string(&wrapped).unwrap()).unwrap();
    decoded.validate().unwrap();
    decoded.report_json.push(' ');
    assert!(decoded.validate().is_err());
}
