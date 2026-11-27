// Windows: icoon, versie-info en manifest in het .exe-bestand. Op andere platforms niets.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let ver = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "1.4.3".to_string());
        let manifest = format!(
            r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity version="{ver}.0" processorArchitecture="*" name="FM27TrainingAssistant" type="win32"/>
  <description>FM27 Training Assistant</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <supportedOS Id="{{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}}"/>
      <supportedOS Id="{{1f676c76-80e1-4239-95bb-83d0f6d0da78}}"/>
    </application>
  </compatibility>
</assembly>"#
        );
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon/app.ico");
        res.set("ProductName", "FM27 Training Assistant");
        res.set("FileDescription", "FM27 Training Assistant");
        res.set("CompanyName", "FM27 Training Assistant");
        res.set("OriginalFilename", "FM27 Training Assistant.exe");
        res.set("InternalName", "fm27-trainer");
        res.set(
            "LegalCopyright",
            "Copyright (c) 2026 cilek14. Upstream portions (c) 2026 Sem Hiel (MIT).",
        );
        res.set_manifest(&manifest);
        if let Err(e) = res.compile() {
            println!("cargo:warning=Windows-icoon niet ingebouwd: {e}");
        }
    }
    println!("cargo:rerun-if-changed=assets/icon/app.ico");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
