use std::{
    io::Cursor,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use axum::{
    Router,
    body::Body,
    http::{
        HeaderValue, Response, StatusCode,
        header::{self, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    },
    response::IntoResponse,
    routing::{get, head},
};
use fast_sign::{
    data::{Data, StreamConfig},
    util::ipa_util,
};
use serde::Deserialize;
use tokio::io::{AsyncWriteExt, duplex};
use tokio_util::io::ReaderStream;
use validator::Validate;

use crate::{
    app::AppState,
    error::ApiError,
    handler::valid_query::ValidQuery,
    util::qrcode_util::{QrcodeRender, QrcodeUtil},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/install.plist", get(install_plist))
        .route("/qrcode.png", get(qrcode_png))
        .route("/red", get(redirect_url))
        .route("/download", get(download_ipa))
        .route("/download", head(download_head))
}
async fn get_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut total_size = 0u64;
    let mut dir_stack = vec![path.as_ref().to_path_buf()];

    while let Some(current_dir) = dir_stack.pop() {
        let entries = std::fs::read_dir(&current_dir)?;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                dir_stack.push(entry.path());
            }
            // 可选：处理符号链接
            // else if metadata.file_type()?.is_symlink() {
            //     // 根据需要决定是否跟随符号链接
            // }
        }
    }

    Ok(total_size)
}
async fn download_head() -> impl IntoResponse {
    let path = PathBuf::from_str("/Users/lake/dounine/github/ipa/fast-sign/data/Payload").unwrap();
    let total_bytes = get_dir_size(&path).await.unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-itunes-ipa"),
        )
        .header(
            CONTENT_LENGTH,
            HeaderValue::from_str(&(total_bytes / 2).to_string()).unwrap(),
        )
        .body(Body::empty())
        .unwrap()
}

async fn download_ipa() -> impl IntoResponse {
    // 创建一个双向管道，容量为4096字节
    let (writer, reader) = duplex(4096);
    let path = PathBuf::from_str("/Users/lake/dounine/github/ipa/fast-sign/data/Payload").unwrap();
    let p = path.clone();
    // 在另一个任务中执行压缩
    tokio::spawn(async move {
        let mut data_config = StreamConfig::default();
        data_config.mem = Some(false);
        let mut ipa = ipa_util::dir_to_zip(&p, &data_config).await?;
        let mut writer2 = Data::from_tokio_stream(writer, data_config.clone());

        ipa.enable_crc32_computer();
        // let mut writer = Data::from MyWriter::new(writer, data_config.clone());
        let level = fast_sign::CompressionLevel::NoCompression;
        // let mut output: Data = Data::from_lazy_path_for_role(
        //     &PathBuf::from_str("./signed.ipa").unwrap(),
        //     true,
        //     true,
        //     true,
        //     data_config.clone(),
        // );
        // ipa.package_with_tokio_callback(&mut output, level, &mut |a, b| Box::pin(async move {
        //     Ok(())
        // }))
        //     .await
        //     .unwrap();
        ipa.package_with_tokio_callback(&mut writer2, level, &mut |a, b| {
            Box::pin(async move { Ok(()) })
        })
        .await
        .unwrap();

        if let Data::TokioStream { inner, .. } = &mut writer2 {
            inner.flush().await.unwrap();
            inner.shutdown().await.unwrap();
        }

        // writer.shutdown().await.unwrap();
        // 打开要压缩的文件
        // let mut file = File::open("example.txt").await?;
        // let mut zip_writer = ZipFileWriter::new(&mut writer);

        // // 添加一个条目到zip文件
        // let entry_builder = ZipEntryBuilder::new("example.txt".into(), Compression::Deflate);
        // zip_writer
        //     .write_entry_whole(entry_builder, &mut file)
        //     .await?;

        // // 关闭zip writer，这会写入中央目录
        // zip_writer.close().await?;

        // 关闭写入端，这样reader会读到EOF
        // writer.shutdown().await?;
        //

        Ok::<_, ApiError>(())
    });

    // 将reader转换为流
    let stream = ReaderStream::new(reader);
    let body = Body::from_stream(stream);

    let total_bytes = get_dir_size(&path).await.unwrap();
    // // 构建响应，设置适当的头
    let response = axum::response::Response::builder()
        .header(CONTENT_TYPE, "application/x-itunes-ipa")
        // .header(
        //     CONTENT_LENGTH,
        //     HeaderValue::from_str(&(total_bytes / 2).to_string()).unwrap(),
        // )
        .header(CONTENT_DISPOSITION, "attachment; filename=\"download.ipa\"")
        .body(body)
        .unwrap();
    response.into_response()
    // ([(CONTENT_TYPE, "application/octet-stream")], body)
}
#[derive(Debug, Deserialize, Validate)]
pub struct PlistParams {
    pub ipa_url: String,
    pub icon_url: String,
    pub bundle_id: String,
    pub bundle_version: String,
    pub bundle_name: String,
}
async fn redirect_url() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>正在安装...</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background-color: #f5f5f5;
        }
        .container {
            text-align: center;
            padding: 40px;
            background: white;
            border-radius: 12px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        .spinner {
            width: 40px;
            height: 40px;
            border: 4px solid #f3f3f3;
            border-top: 4px solid #007aff;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin: 0 auto 20px;
        }
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        p {
            color: #333;
            margin: 10px 0;
        }
        a {
            color: #007aff;
            text-decoration: none;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="spinner"></div>
        <p>正在跳转到安装页面...</p>
        <p>如果没有自动跳转，请<a href="itms-services://?action=download-manifest&url=https://testsign.ipadump.com/install.plist">点击这里</a></p>
    </div>
    <script>
        setTimeout(function() {
            window.location.href = "itms-services://?action=download-manifest&url=https://testsign.ipadump.com/install.plist";
        }, 500);
    </script>
</body>
</html>
"#;

    let mut resp = Response::new(Body::from(html));
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    resp
}

async fn qrcode_png() -> impl IntoResponse {
    let mut qrcode = QrcodeUtil::new("https://testsign.ipadump.com/api/ipa/red");
    let mark = std::fs::read("/Users/lake/dounine/github/ipa/fast-sign/data/icon.png").unwrap();
    qrcode.water_mark(&mark);
    qrcode.rounded(true);
    match qrcode.render() {
        Ok(image) => {
            let mut data = Cursor::new(vec![]);
            match image.write_to(&mut data, image::ImageFormat::Png) {
                Ok(_) => {}
                Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }

            let mut resp = Response::new(axum::body::Body::from(data.into_inner()));
            resp.headers_mut()
                .insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));
            return resp;
        }
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
async fn install_plist(ValidQuery(params): ValidQuery<PlistParams>) -> impl IntoResponse {
    let resp = format!(
        r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
            <dict>
                <key>items</key>
                <array>
                    <dict>
                        <key>assets</key>
                        <array>
                            <dict>
                                <key>kind</key>
                                <string>software-package</string>
                                <key>url</key>
                                <string>{}</string>
                            </dict>
                            <dict>
                                <key>kind</key>
                                <string>display-image</string>
                                <key>needs-shine</key>
                                <true/>
                                <key>url</key>
                                <string>{}</string>
                            </dict>
                        </array>
                        <key>metadata</key>
                        <dict>
                            <key>bundle-identifier</key>
                            <string>{}</string>
                            <key>bundle-version</key>
                            <string>{}</string>
                            <key>kind</key>
                            <string>software</string>
                            <key>title</key>
                            <string>{}</string>
                        </dict>
                    </dict>
                </array>
            </dict>
        </plist>"#,
        params.ipa_url,
        params.icon_url,
        params.bundle_id,
        params.bundle_version,
        params.bundle_name
    );
    let mut resp = Response::new(axum::body::Body::from(resp));
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-plist"),
    );
    resp
}
