use super::types::client::ClientInfo;

pub fn parse_client(agent: Option<&str>) -> Option<ClientInfo> {
    let agent_str = agent?.trim();
    if agent_str.is_empty() {
        return None;
    }

    let mut parts = agent_str.splitn(2, ' ');
    let core = parts.next()?;
    let meta_part = parts.next();

    let mut core_parts = core.split('/');
    let name = core_parts.next()?.to_string();
    if name.is_empty() {
        return None;
    }
    let version = core_parts.next().map(|s| s.to_string());

    let mut url = None;
    let mut codename = None;
    let mut release_date = None;

    if let Some(meta) = meta_part {
        if meta.starts_with('(') && meta.ends_with(')') {
            let content = &meta[1..meta.len() - 1]; // slice(1, -1)
            if content.starts_with("http") {
                url = Some(content.to_string());
            } else {
                let mut meta_parts = content.split('/');
                if let Some(tag) = meta_parts.next() {
                    if !tag.is_empty() {
                        codename = Some(tag.to_string());
                    }
                }
                if let Some(date) = meta_parts.next() {
                    release_date = Some(date.to_string());
                }
            }
        }
    }

    Some(ClientInfo {
        name,
        version,
        url,
        codename,
        release_date,
    })
}
