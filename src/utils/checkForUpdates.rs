use tokio::process::Command;
use super::logger::logger;

pub async fn check_for_updates() {
    logger("info", "Git", "Checking for updates...", None);

    let fetch_res = Command::new("git")
        .arg("fetch")
        .output()
        .await;

    if fetch_res.is_err() {
        logger("warn", "Git", "Failed to check for updates: git fetch failed", None);
        return;
    }

    let local_res = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .await;

    let remote_res = Command::new("git")
        .args(["rev-parse", "@{u}"])
        .output()
        .await;

    match (local_res, remote_res) {
        (Ok(local_out), Ok(remote_out)) => {
            let local = String::from_utf8_lossy(&local_out.stdout).trim().to_string();
            let remote = String::from_utf8_lossy(&remote_out.stdout).trim().to_string();

            if local != remote {
                let behind_res = Command::new("git")
                    .args(["rev-list", "--right-only", "--count", "HEAD...@{u}"])
                    .output()
                    .await;

                let commit_info_res = Command::new("git")
                    .args(["log", "-1", "--pretty=format:%h - %s (%cr)", "@{u}"])
                    .output()
                    .await;

                if let Ok(behind_out) = behind_res {
                    let behind = String::from_utf8_lossy(&behind_out.stdout).trim().to_string();
                    logger("warn", "Git", &format!("Your version is {} commits behind the remote.", behind), None);
                }

                if let Ok(commit_info_out) = commit_info_res {
                    let remote_commit = String::from_utf8_lossy(&commit_info_out.stdout).trim().to_string();
                    logger("warn", "Git", &format!("Latest commit: {}", remote_commit), None);
                }

                logger("warn", "Git", "Please run \"git pull\" to update.", None);
            } else {
                logger("info", "Git", "You are running the latest version.", None);
            }
        }
        _ => {
            logger("warn", "Git", "Failed to check for updates: could not resolve local or remote HEAD", None);
        }
    }
}
