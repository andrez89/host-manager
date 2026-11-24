#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    Italian,
    English,
}

#[derive(Clone, Copy)]
pub enum Text {
    AddPlaceholder,
    Add,
    BlockAll,
    UnblockAll,
    HideLocked,
    ShowLocked,
    AdminRequired,
    ApplyHosts,
    ApplySuccess,
    ApplyError,
    On,
    Off,
    UnlockToToggle,
    ToggleSite,
    Unlock,
    Lock,
    Remove,
    InvalidDomain,
    PkexecStart,
    #[cfg(target_os = "macos")]
    MacAuthDenied,
    #[cfg(target_os = "windows")]
    WindowsAuthDenied,
    MissingPayload,
}

impl Language {
    pub fn from_locale(locale: Option<&str>) -> Self {
        match locale
            .unwrap_or_default()
            .split(['-', '_', '.', '@'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "it" => Self::Italian,
            _ => Self::English,
        }
    }

    pub fn system() -> Self {
        Self::from_locale(sys_locale::get_locale().as_deref())
    }

    pub fn text(self, text: Text) -> &'static str {
        match (self, text) {
            (Self::Italian, Text::AddPlaceholder) => "Aggiungi un sito (es. facebook.com)",
            (Self::English, Text::AddPlaceholder) => "Add a site (e.g. facebook.com)",
            (Self::Italian, Text::Add) => "Aggiungi",
            (Self::English, Text::Add) => "Add",
            (Self::Italian, Text::BlockAll) => "Blocca tutti",
            (Self::English, Text::BlockAll) => "Block all",
            (Self::Italian, Text::UnblockAll) => "Sblocca tutti",
            (Self::English, Text::UnblockAll) => "Unblock all",
            (Self::Italian, Text::HideLocked) => "Nascondi siti protetti",
            (Self::English, Text::HideLocked) => "Hide locked sites",
            (Self::Italian, Text::ShowLocked) => "Mostra siti protetti",
            (Self::English, Text::ShowLocked) => "Show locked sites",
            (Self::Italian, Text::AdminRequired) => {
                "Le modifiche richiedono autorizzazione amministrativa."
            }
            (Self::English, Text::AdminRequired) => "Changes require administrator authorization.",
            (Self::Italian, Text::ApplyHosts) => "Applica modifiche",
            (Self::English, Text::ApplyHosts) => "Apply changes",
            (Self::Italian, Text::ApplySuccess) => "File hosts aggiornato.",
            (Self::English, Text::ApplySuccess) => "Hosts file updated.",
            (Self::Italian, Text::ApplyError) => "Errore:",
            (Self::English, Text::ApplyError) => "Error:",
            (Self::Italian, Text::On) => "ON",
            (Self::English, Text::On) => "ON",
            (Self::Italian, Text::Off) => "OFF",
            (Self::English, Text::Off) => "OFF",
            (Self::Italian, Text::UnlockToToggle) => "Rimuovi il lucchetto per cambiare stato",
            (Self::English, Text::UnlockToToggle) => "Unlock this site to change its state",
            (Self::Italian, Text::ToggleSite) => "Blocca o sblocca il sito",
            (Self::English, Text::ToggleSite) => "Block or unblock this site",
            (Self::Italian, Text::Unlock) => "Rimuovi lucchetto",
            (Self::English, Text::Unlock) => "Unlock site",
            (Self::Italian, Text::Lock) => "Proteggi sito",
            (Self::English, Text::Lock) => "Lock site",
            (Self::Italian, Text::Remove) => "Rimuovi",
            (Self::English, Text::Remove) => "Remove",
            (Self::Italian, Text::InvalidDomain) => "Dominio non valido",
            (Self::English, Text::InvalidDomain) => "Invalid domain",
            (Self::Italian, Text::PkexecStart) => "Avvio pkexec",
            (Self::English, Text::PkexecStart) => "Could not start pkexec",
            #[cfg(target_os = "macos")]
            (Self::Italian, Text::MacAuthDenied) => "Autorizzazione amministratore annullata",
            #[cfg(target_os = "macos")]
            (Self::English, Text::MacAuthDenied) => "Administrator authorization was cancelled",
            #[cfg(target_os = "windows")]
            (Self::Italian, Text::WindowsAuthDenied) => "Richiesta UAC annullata",
            #[cfg(target_os = "windows")]
            (Self::English, Text::WindowsAuthDenied) => "UAC request was cancelled",
            (Self::Italian, Text::MissingPayload) => "Percorso del payload mancante",
            (Self::English, Text::MissingPayload) => "Missing payload path",
        }
    }

    pub fn blocked_summary(self, blocked: usize, total: usize) -> String {
        match self {
            Self::Italian => format!("{blocked} / {total} siti bloccati"),
            Self::English => format!("{blocked} / {total} sites blocked"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_italian_for_italian_system_locales() {
        assert_eq!(
            Language::from_locale(Some("it_IT.UTF-8")),
            Language::Italian
        );
        assert_eq!(Language::from_locale(Some("it-CH")), Language::Italian);
    }

    #[test]
    fn falls_back_to_english_for_other_or_missing_locales() {
        assert_eq!(Language::from_locale(Some("en_US")), Language::English);
        assert_eq!(Language::from_locale(Some("fr_FR")), Language::English);
        assert_eq!(Language::from_locale(None), Language::English);
    }
}
