use crate::idx_store::IDX_STORE;
use crate::kv_store::CONF_STORE;
use crate::walk_metrics::WALK_METRICS;
use crate::{utils, walk_exec, watch_exec};
use log::info;

#[cfg(windows)]
use log::error;
#[cfg(windows)]
use std::sync::mpsc;
#[cfg(windows)]
use std::sync::mpsc::Sender;

#[cfg(windows)]
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[cfg(windows)]
use crate::usn_journal_watcher::Watcher;

#[cfg(windows)]
const STORE_PATH: &'static str = "orangecachedata";
#[cfg(windows)]
const RECYCLE_PATH: &'static str = "$RECYCLE.BIN";
const VERSION: &'static str = "0.6.1";
const LAST_INDEX_TS: &'static str = "last_index_ts";

pub fn run() {
    std::thread::spawn(|| {
        do_run();
    });
}

fn do_run() {
    housekeeping();

    let reindex = need_reindex();
    info!("need reindex: {}", reindex);
    if reindex {
        walk_exec::run();
        CONF_STORE.put_str(LAST_INDEX_TS.to_string(), curr_ts().to_string());
        info!("walk exec done");
    };

    IDX_STORE.disable_full_indexing();
    WALK_METRICS.write().unwrap().end_of_no_reindex();
    info!("start fs watch");

    #[cfg(windows)]
    if cfg!(target_os = "windows") {
        if reindex {
            info!("use watcher due to reindex");
            watch_exec::run();
        } else {
            info!("try use usn");
            win_watch();
        }
    }
    #[cfg(unix)]
    watch_exec::run();
}

#[cfg(windows)]
fn win_watch() {
    watch_exec::run();
}

pub fn reindex() {
    CONF_STORE.put_str("reindex".to_string(), "1".to_string());
}

fn need_reindex() -> bool {
    let key = LAST_INDEX_TS.to_string();

    return match CONF_STORE.get_str(key.clone()) {
        None => true,
        Some(val) => {
            let ts = val.parse::<u64>().unwrap();
            if curr_ts() - ts > 3600 * 24 * 30 {
                return true;
            }
            false
        }
    };
}

fn curr_ts() -> u64 {
    let curr_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    curr_ts
}

pub fn housekeeping() {
    info!("housekeeping...");

    let reidx_opt = CONF_STORE.get_str("reindex".to_string());
    match reidx_opt {
        None => {
            info!("no need to reindex");
        }
        Some(_) => {
            clear();
            info!("detect reindex sign");
            return;
        }
    }

    let version_opt = CONF_STORE.get_str("version".to_string());
    match version_opt {
        None => {
            clear();
            info!("init version {}", VERSION);
        }
        Some(version) => {
            if !version.eq(VERSION) {
                clear();
                info!("clean old version cachedata");
            } else {
                info!("no need to clean, current version:{}", VERSION);
            }
        }
    }
}

fn clear() {
    let _ = std::fs::remove_dir_all(&format!("{}/{}/idx", utils::data_dir(), STORE_PATH));
    CONF_STORE.clear();
    CONF_STORE.put_str("version".to_string(), VERSION.to_string());
}
