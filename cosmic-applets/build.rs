use std::fs;
use xdgen::{App, Context, FluentString};

fn main() {
    let ctx = Context::new("../i18n/", "desktop_entries").unwrap();

    [
        (
            "com.system76.CosmicAppletA11y",
            "cosmic-applet-a11y",
            "cosmic-applet-a11y-comment",
            "cosmic-applet-a11y-keywords",
        ),
        (
            "com.system76.CosmicAppletInputSources",
            "cosmic-applet-input-sources",
            "cosmic-applet-input-sources-comment",
            "cosmic-applet-input-sources-keywords",
        ),
        (
            "com.system76.CosmicAppletWorkspaces",
            "cosmic-applet-workspaces",
            "cosmic-applet-workspaces-comment",
            "cosmic-applet-workspaces-keywords",
        ),
    ]
    .into_iter()
    .map(|(id, name, comment, keywords)| {
        let template_path = ["../", name, "/data/", id, ".desktop"].concat();

        let app = App::new(FluentString(name))
            .comment(FluentString(comment))
            .keywords(FluentString(keywords));

        (id, app.expand_desktop(&template_path, &ctx).unwrap())
    })
    .for_each(|(id, contents)| {
        let parent = "../target/xdgen/";
        fs::create_dir_all(parent).unwrap();
        fs::write([parent, id, ".desktop"].concat().as_str(), contents).unwrap();
    });
}
