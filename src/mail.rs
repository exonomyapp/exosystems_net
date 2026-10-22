//! Mailbox administration for the domain's mail server.
//!
//! The site and the mail server run on the same instance, so the console
//! manages the domain's mailboxes directly: the account that owns the
//! mailbox, its Maildir, the alias that routes the address to it, and the
//! webmail user that reads it.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// The domain the mailboxes belong to.
pub const MAIL_DOMAIN: &str = "exosystems.net";

/// The host the webmail is reached on.
pub const MAIL_HOST: &str = "mail.exosystems.net";

/// One mailbox on the domain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mailbox {
    /// The full address, local part and domain.
    pub address: String,
    /// The local part, which is also the account name on the instance.
    pub local_part: String,
    /// The name that appears on mail leaving this mailbox.
    pub display_name: String,
    /// Whether the webmail holds a user for this mailbox.
    pub webmail_user: bool,
}

/// A mailbox that was just created, carrying its password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatedMailbox {
    pub address: String,
    pub password: String,
}

#[cfg(feature = "ssr")]
const ALIASES: &str = "/etc/aliases";

#[cfg(feature = "ssr")]
const PASSWORD_FILE: &str = "/root/.mailbox-passwords";

#[cfg(feature = "ssr")]
const ROUNDCUBE_DB: &str = "/var/lib/roundcubemail/db.sqlite";

/// An error carrying a message for the caller.
#[cfg(feature = "ssr")]
fn fail(message: impl Into<String>) -> ServerFnError {
    ServerFnError::ServerError(message.into())
}

/// Run a command on the instance and return its output.
#[cfg(feature = "ssr")]
fn run(program: &str, args: &[&str], stdin: Option<&str>) -> Result<String, ServerFnError> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new(program)
        .args(args)
        .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| fail(format!("{program} could not start: {e}")))?;

    if let Some(text) = stdin {
        let mut pipe = child
            .stdin
            .take()
            .ok_or_else(|| fail(format!("{program} took no input")))?;
        let _ = pipe.write_all(text.as_bytes());
    }

    let out = child
        .wait_with_output()
        .map_err(|e| fail(e.to_string()))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        let message = if err.is_empty() {
            format!("{program} did not complete")
        } else {
            err
        };
        return Err(fail(message));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// A local part is lowercase letters and digits with dots, underscores and
/// hyphens. It becomes an account name and a directory name, so it is held
/// to a narrow shape.
#[cfg(feature = "ssr")]
fn local_part_ok(local_part: &str) -> bool {
    if local_part.is_empty() || local_part.len() > 64 {
        return false;
    }
    if local_part.starts_with('-') || local_part.ends_with('.') || local_part.contains("..") {
        return false;
    }
    local_part
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_' || c == '-')
}

/// Escape a value for a single-quoted SQL string.
#[cfg(feature = "ssr")]
fn sql_value(value: &str) -> String {
    value.replace('\'', "''")
}

/// A password drawn from letters and digits, twenty-four characters long.
#[cfg(feature = "ssr")]
fn new_password() -> String {
    use rand::Rng;
    const ALPHABET: &[u8] = b"abcdefghijkmnopqrstuvwxyz23456789";
    let mut rng = rand::thread_rng();
    (0..24)
        .map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char)
        .collect()
}

/// The accounts on the instance that hold a mailbox, read from the
/// password file the console keeps.
#[cfg(feature = "ssr")]
fn known_mailboxes() -> Vec<(String, String)> {
    let mut entries = Vec::new();
    if let Ok(text) = std::fs::read_to_string(PASSWORD_FILE) {
        for line in text.lines() {
            if let Some((local_part, password)) = line.split_once(':') {
                let local_part = local_part.trim().to_string();
                if local_part_ok(&local_part) {
                    entries.push((local_part, password.to_string()));
                }
            }
        }
    }
    entries
}

/// The webmail users and the names that go with them.
#[cfg(feature = "ssr")]
fn webmail_users() -> Vec<(String, String)> {
    let sep = '\u{1f}';
    let sql = format!(
        ".separator {sep}\nSELECT u.username, COALESCE(i.name, '') FROM users u LEFT JOIN identities i ON i.user_id = u.user_id WHERE u.mail_host = '{host}';\n",
        sep = sep,
        host = MAIL_HOST
    );
    let out = match run("sqlite3", &[ROUNDCUBE_DB], Some(&sql)) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    out.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, sep);
            let username = parts.next()?.trim().to_string();
            let name = parts.next().unwrap_or("").trim().to_string();
            if username.is_empty() {
                None
            } else {
                Some((username, name))
            }
        })
        .collect()
}

/// The domain's mailboxes, from the accounts the console holds and the
/// users the webmail holds.
#[cfg(feature = "ssr")]
fn list_mailboxes() -> Result<Vec<Mailbox>, ServerFnError> {
    let known = known_mailboxes();
    let web = webmail_users();

    let mut out: Vec<Mailbox> = Vec::new();
    let mut push = |local_part: String, display_name: String, webmail_user: bool| {
        if local_part.is_empty() {
            return;
        }
        if out.iter().any(|m| m.local_part == local_part) {
            return;
        }
        out.push(Mailbox {
            address: format!("{local_part}@{MAIL_DOMAIN}"),
            local_part,
            display_name,
            webmail_user,
        });
    };

    for (local_part, _) in &known {
        let display_name = web
            .iter()
            .find(|(u, _)| u == local_part)
            .map(|(_, n)| n.clone())
            .unwrap_or_default();
        let webmail_user = web.iter().any(|(u, _)| u == local_part);
        push(local_part.clone(), display_name, webmail_user);
    }
    for (username, name) in &web {
        if known.iter().any(|(l, _)| l == username) {
            continue;
        }
        push(username.clone(), name.clone(), true);
    }
    out.sort_by(|a, b| a.local_part.cmp(&b.local_part));
    Ok(out)
}

/// Point the alias at the account, so that a message to the address
/// reaches the mailbox that carries it.
#[cfg(feature = "ssr")]
fn ensure_alias(local_part: &str) -> Result<(), ServerFnError> {
    let text = std::fs::read_to_string(ALIASES)
        .map_err(|e| fail(format!("the alias file is unreadable: {e}")))?;
    let mut lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
    let mut found = false;
    for line in lines.iter_mut() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once(':') {
            if name.trim() == local_part {
                *line = format!("{local_part}: {local_part}");
                found = true;
            }
        }
    }
    if !found {
        lines.push(format!("{local_part}: {local_part}"));
    }
    let mut out = lines.join("\n");
    out.push('\n');
    std::fs::write(ALIASES, out)
        .map_err(|e| fail(format!("the alias file is unwritable: {e}")))?;
    run("newaliases", &[], None)?;
    Ok(())
}

/// Drop the alias that pointed at the account.
#[cfg(feature = "ssr")]
fn remove_alias(local_part: &str) -> Result<(), ServerFnError> {
    let text = std::fs::read_to_string(ALIASES)
        .map_err(|e| fail(format!("the alias file is unreadable: {e}")))?;
    let mut lines: Vec<String> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            if let Some((name, _)) = trimmed.split_once(':') {
                if name.trim() == local_part {
                    continue;
                }
            }
        }
        lines.push(line.to_string());
    }
    let mut out = lines.join("\n");
    out.push('\n');
    std::fs::write(ALIASES, out)
        .map_err(|e| fail(format!("the alias file is unwritable: {e}")))?;
    run("newaliases", &[], None)?;
    Ok(())
}

/// The webmail user and the identity that carries the display name.
#[cfg(feature = "ssr")]
fn create_webmail_user(local_part: &str, display_name: &str) -> Result<(), ServerFnError> {
    let name = if display_name.trim().is_empty() {
        local_part.to_string()
    } else {
        display_name.trim().to_string()
    };
    let sql = format!(
        "INSERT OR IGNORE INTO users (username, mail_host, created, language) VALUES ('{user}', '{host}', datetime('now'), 'en');\
         INSERT INTO identities (user_id, changed, del, standard, name, email, \"reply-to\", bcc, signature, html_signature) \
         SELECT user_id, datetime('now'), 0, 1, '{name}', '{address}', '{address}', '', '', 0 FROM users WHERE username = '{user}' AND NOT EXISTS (SELECT 1 FROM identities WHERE user_id = users.user_id);\n",
        user = sql_value(local_part),
        host = MAIL_HOST,
        name = sql_value(&name),
        address = sql_value(&format!("{local_part}@{MAIL_DOMAIN}")),
    );
    run("sqlite3", &[ROUNDCUBE_DB], Some(&sql))?;
    Ok(())
}

/// Withdraw the webmail user and its identity.
#[cfg(feature = "ssr")]
fn remove_webmail_user(local_part: &str) -> Result<(), ServerFnError> {
    let sql = format!(
        "DELETE FROM identities WHERE user_id IN (SELECT user_id FROM users WHERE username = '{user}');\
         DELETE FROM users WHERE username = '{user}';\n",
        user = sql_value(local_part)
    );
    run("sqlite3", &[ROUNDCUBE_DB], Some(&sql))?;
    Ok(())
}

/// The password file the instance keeps for the mailboxes.
#[cfg(feature = "ssr")]
fn write_password_file(entries: &[(String, String)]) -> Result<(), ServerFnError> {
    let mut text = String::new();
    for (local_part, password) in entries {
        text.push_str(&format!("{local_part}:{password}\n"));
    }
    std::fs::write(PASSWORD_FILE, text)
        .map_err(|e| fail(format!("the password file is unwritable: {e}")))?;
    run("chmod", &["600", PASSWORD_FILE], None)?;
    Ok(())
}

/// The machine that holds the mail server. A build running on another
/// machine has no mail server to manage, and the account tools would act on
/// the wrong machine.
#[cfg(feature = "ssr")]
fn mail_server_present() -> bool {
    std::path::Path::new(ROUNDCUBE_DB).is_file()
}

/// The mailboxes the domain holds.
#[server]
pub async fn mailbox_list(token: String) -> Result<Vec<Mailbox>, ServerFnError> {
    crate::auth::require_admin(&token).await?;
    if !mail_server_present() {
        return Err(fail(
            "This build is not running on the instance that holds the mail server.",
        ));
    }
    list_mailboxes()
}

/// Create a mailbox: the account, its password, its mail store, the alias
/// that routes the address to it, and the webmail user that reads it.
/// The password is returned once and held in the instance's password file.
#[server]
pub async fn create_mailbox(
    token: String,
    local_part: String,
    display_name: String,
) -> Result<CreatedMailbox, ServerFnError> {
    crate::auth::require_admin(&token).await?;
    if !mail_server_present() {
        return Err(fail(
            "This build is not running on the instance that holds the mail server.",
        ));
    }

    let local_part = local_part.trim().to_lowercase();
    if !local_part_ok(&local_part) {
        return Err(fail(
            "An address is lowercase letters and digits, with dots, underscores and hyphens, up to 64 characters.",
        ));
    }
    if known_mailboxes().iter().any(|(l, _)| *l == local_part) {
        return Err(fail(format!(
            "{local_part}@{MAIL_DOMAIN} already exists."
        )));
    }
    if webmail_users().iter().any(|(u, _)| *u == local_part) {
        return Err(fail(format!(
            "{local_part}@{MAIL_DOMAIN} already has a webmail user."
        )));
    }

    let password = new_password();
    let home = format!("/home/{local_part}");

    run(
        "useradd",
        &["-m", "-d", &home, "-s", "/sbin/nologin", &local_part],
        None,
    )?;
    if let Err(e) = run("chpasswd", &[], Some(&format!("{local_part}:{password}\n"))) {
        let _ = run("userdel", &[&local_part], None);
        return Err(e);
    }
    let _ = run("usermod", &["-a", "-G", "mail", &local_part], None);
    let store = format!("{home}/Maildir");
    if let Err(e) = run("install", &["-d", "-o", &local_part, "-g", &local_part, "-m", "700", &store], None) {
        let _ = run("userdel", &[&local_part], None);
        return Err(e);
    }
    for folder in ["tmp", "new", "cur"] {
        let path = format!("{store}/{folder}");
        if let Err(e) = run("install", &["-d", "-o", &local_part, "-g", &local_part, "-m", "700", &path], None) {
            let _ = run("userdel", &[&local_part], None);
            return Err(e);
        }
    }

    if let Err(e) = ensure_alias(&local_part) {
        let _ = run("userdel", &[&local_part], None);
        return Err(e);
    }
    if let Err(e) = create_webmail_user(&local_part, &display_name) {
        let _ = remove_alias(&local_part);
        let _ = run("userdel", &[&local_part], None);
        return Err(e);
    }

    let mut entries = known_mailboxes();
    entries.push((local_part.clone(), password.clone()));
    entries.sort();
    if let Err(e) = write_password_file(&entries) {
        return Err(e);
    }

    Ok(CreatedMailbox {
        address: format!("{local_part}@{MAIL_DOMAIN}"),
        password,
    })
}

/// Withdraw a mailbox: the webmail user, the alias and the account. The
/// mail store stays on the instance as the archive of what the address
/// received.
#[server]
pub async fn remove_mailbox(token: String, local_part: String) -> Result<(), ServerFnError> {
    crate::auth::require_admin(&token).await?;
    if !mail_server_present() {
        return Err(fail(
            "This build is not running on the instance that holds the mail server.",
        ));
    }

    let local_part = local_part.trim().to_lowercase();
    if !local_part_ok(&local_part) {
        return Err(fail("That is not an address on this domain."));
    }
    let known = known_mailboxes();
    if !known.iter().any(|(l, _)| *l == local_part) {
        return Err(fail(format!(
            "{local_part}@{MAIL_DOMAIN} is not a mailbox on this instance."
        )));
    }

    remove_webmail_user(&local_part)?;
    remove_alias(&local_part)?;
    let _ = run("userdel", &[&local_part], None);

    let entries: Vec<(String, String)> = known
        .into_iter()
        .filter(|(l, _)| *l != local_part)
        .collect();
    write_password_file(&entries)?;

    Ok(())
}
