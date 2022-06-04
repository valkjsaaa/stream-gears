mod uploader;

use crate::uploader::UploadLine;
use anyhow::Error;
use biliup::client::Client;
use pyo3::prelude::*;
use std::path::PathBuf;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;


#[pyfunction]
fn upload(
    video_path: Vec<PathBuf>,
    cookie_file: PathBuf,
    title: String,
    tid: u16,
    tag: String,
    copyright: u8,
    source: String,
    desc: String,
    dynamic: String,
    cover: String,
    dtime: Option<u32>,
    line: Option<UploadLine>,
    limit: usize,
) -> PyResult<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    // 输出到控制台中
    let formatting_layer = tracing_subscriber::FmtSubscriber::builder()
        // will be written to stdout.
        // builds the subscriber.
        .finish();
    let file_appender = tracing_appender::rolling::never("", "upload.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    let collector = formatting_layer.with(file_layer);

    tracing::subscriber::with_default(collector, || -> PyResult<()> {
        match rt.block_on(uploader::upload(
            video_path, cookie_file, line, limit, title, tid, tag, copyright, source, desc, dynamic, cover, dtime,
        )) {
            Ok(res) => Ok(res),
            // Ok(_) => {  },
            Err(err) => {
                return Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{}, {}", err.root_cause(), err.to_string())));
            }
        }
    })?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn stream_gears(_py: Python, m: &PyModule) -> PyResult<()> {
    // let file_appender = tracing_appender::rolling::daily("", "upload.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // tracing_subscriber::fmt()
    //     .with_writer(non_blocking)
    //     .init();
    m.add_function(wrap_pyfunction!(upload, m)?)?;
    m.add_class::<UploadLine>()?;
    Ok(())
}
