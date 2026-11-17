//! Atualização automática no estilo Discord: checa ao abrir e a cada 6 horas,
//! baixa em segundo plano e aplica quando o usuário reinicia.
//!
//! Configurado na compilação (o CI de release define):
//! - `OXID_UPDATE_BASE`: URL base dos manifestos, ex.: `https://downloads.oxid.app`
//! - `OXID_UPDATER_PUBKEY`: chave pública de `tauri signer generate`
//! - `OXID_CHANNEL`: `stable` (padrão), `beta` ou `canary`
//!
//! O manifesto fica em `{base}/{canal}/latest.json`. Sem base ou chave, o
//! updater fica desligado (builds de desenvolvimento).

use std::time::Duration;

use serde::Serialize;
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, Url};
use tauri_plugin_updater::UpdaterExt;

const CHECK_EVERY: Duration = Duration::from_secs(6 * 60 * 60);

pub const CHANNEL: &str = match option_env!("OXID_CHANNEL") {
    Some(c) => c,
    None => "stable",
};

pub fn pubkey() -> Option<&'static str> {
    option_env!("OXID_UPDATER_PUBKEY").filter(|k| !k.is_empty())
}

pub fn endpoint() -> Option<Url> {
    let base = option_env!("OXID_UPDATE_BASE").filter(|b| !b.is_empty())?;
    manifest_url(base, CHANNEL)
}

fn manifest_url(base: &str, channel: &str) -> Option<Url> {
    Url::parse(&format!(
        "{}/{channel}/latest.json",
        base.trim_end_matches('/')
    ))
    .ok()
}

#[derive(Clone, Serialize)]
pub struct UpdateReady {
    version: String,
}

/// Atualização já baixada, esperando reinício. Guardada para a interface
/// consultar ao carregar (o evento pode sair antes de ela escutar).
#[derive(Default)]
pub struct PendingUpdate(pub Mutex<Option<UpdateReady>>);

/// Loop de verificação; roda em segundo plano durante toda a vida do app.
pub async fn run(app: AppHandle) {
    let (Some(endpoint), Some(_)) = (endpoint(), pubkey()) else {
        eprintln!("updater desligado: build sem OXID_UPDATE_BASE/OXID_UPDATER_PUBKEY");
        return;
    };
    loop {
        if let Err(e) = check_once(&app, endpoint.clone()).await {
            eprintln!("falha ao verificar atualização: {e}");
        }
        tokio::time::sleep(CHECK_EVERY).await;
    }
}

async fn check_once(app: &AppHandle, endpoint: Url) -> tauri_plugin_updater::Result<()> {
    let updater = app.updater_builder().endpoints(vec![endpoint])?.build()?;
    let Some(update) = updater.check().await? else {
        return Ok(());
    };
    // Baixa e instala; no Windows o instalador aplica ao reiniciar.
    update.download_and_install(|_, _| {}, || {}).await?;
    let ready = UpdateReady {
        version: update.version.clone(),
    };
    *app.state::<PendingUpdate>().0.lock().unwrap() = Some(ready.clone());
    let _ = app.emit("update-ready", ready);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn manifest_url_per_channel() {
        let u = super::manifest_url("https://downloads.oxid.app/", "beta").unwrap();
        assert_eq!(u.as_str(), "https://downloads.oxid.app/beta/latest.json");
    }
}
