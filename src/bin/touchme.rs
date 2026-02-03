// bin/touchme.rs

use touchme::*;

const RE_YYYYMMDD: &str = r"(\d\d\d\d)(\d\d)(\d\d)";
const RE_YYYY_MM_DD: &str = r"(\d\d\d\d).(\d\d).(\d\d)";

fn main() -> anyhow::Result<()> {
    let me = env!("CARGO_BIN_NAME");
    let opts = OptsCommon::parse();
    opts.start_pgm(me);
    let re1 = Regex::new(RE_YYYYMMDD)?;
    let re2 = Regex::new(RE_YYYY_MM_DD)?;

    debug!("opts: {opts:?}");

    for a in &opts.args {
        let md = fs::metadata(a)?;
        if md.is_file() {
            if let Some(ts) = detect_date_filename(path::Path::new(a), &[&re1, &re2])
                && !opts.dry_run
            {
                update_timestamp(path::Path::new(a), ts)?;
                info!("Updated timestamp of {a}");
            }
        } else if md.is_dir() {
            if !opts.recursive {
                info!("{a} is a directory and recursion is not requested.");
                continue;
            }
            let files = recurse_subdir(path::Path::new(a))?;
            debug!("Found {} files:\n{:?}", files.len(), files);
            for f in files {
                if let Some(ts) = detect_date_filename(&f, &[&re1, &re2])
                    && !opts.dry_run
                {
                    update_timestamp(&f, ts)?;
                    info!("Updated timestamp of {f:?}");
                }
            }
        }
    }

    Ok(())
}

fn recurse_subdir(p: &path::Path) -> anyhow::Result<Vec<path::PathBuf>> {
    let mut files = Vec::new();
    if p.is_dir() {
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(recurse_subdir(&path)?);
            }
            if path.is_file() {
                files.push(path);
            }
        }
    } else if p.is_file() {
        files.push(p.to_path_buf());
    }
    Ok(files)
}

fn detect_date_filename(file_path: &path::Path, re_date: &[&Regex]) -> Option<time::SystemTime> {
    let filename = file_path.file_name()?.to_string_lossy();
    debug!("filename: {filename:?}");

    for re in re_date {
        if let Some(caps) = re.captures(&filename) {
            let (year, month, day) = (
                caps[1].parse::<i32>().ok()?,
                caps[2].parse::<u32>().ok()?,
                caps[3].parse::<u32>().ok()?,
            );
            let ts = NaiveDate::from_ymd_opt(year, month, day)?
                .and_hms_opt(0, 0, 0)?
                .and_utc();
            debug!("Detected {year:04}-{month:02}-{day:02}");
            return Some(ts.into());
        }
    }
    None
}

fn update_timestamp(file_path: &path::Path, ts: time::SystemTime) -> anyhow::Result<()> {
    let file = fs::File::open(file_path)?;
    file.set_modified(ts)?;
    Ok(())
}
// EOF
