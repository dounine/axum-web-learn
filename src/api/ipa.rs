use std::{io::Cursor, vec};

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Response, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    app::AppState,
    handler::valid_query::ValidQuery,
    util::qrcode_util::{QrcodeRender, QrcodeUtil},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/install.plist", get(install_plist))
        .route("/qrcode.png", get(qrcode_png))
        .route("/red", get(redirect_url))
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
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/png"),
            );
            return resp
        }
        Err(e) => {
            return (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
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
