//#![windows_subsystem = "windows"]

use web_view::*;

fn main() {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <title>React starter app</title>
    </head>
    <body>
        <div id="app"></div>
		{scripts}
    </body>
</html>"#,
        scripts = inline_script(include_str!("js/build/index.js"))
    );

    let webview = web_view::builder()
        .title("Rust & React App")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|__webview, _arg| Ok(()))
        .build()
        .unwrap();

    let _res = webview.run().unwrap();
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
