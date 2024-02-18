use crate::get_files_and_dirs;
use html::{
    content::Header,
    metadata::Head,
    root::{Body, Html},
    text_content::UnorderedList,
};
use std::{env, path::PathBuf};

pub fn diplay_directory_listing(dir_resource: &PathBuf) -> String {
    let body = get_body(dir_resource);
    let head = get_head(dir_resource);

    let html = Html::builder()
        .lang(String::from("en"))
        .push(head)
        .push(body)
        .build()
        .to_string();
    return html;
}

fn get_head(dir_resource: &PathBuf) -> Head {
    let dir_name = dir_resource
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .map(|name| {
            if env::current_dir()
                .ok()
                .is_some_and(|path| path == *dir_resource)
            {
                return "/".into();
            } else {
                return name;
            }
        })
        .unwrap_or("".into());

    let head = Head::builder()
        .meta(|meta_builder| meta_builder.charset("utf-8"))
        .title(|title_builder| title_builder.text(format!("Directory listing for {}", dir_name)))
        .build();

    return head;
}

fn get_body(dir_resource: &PathBuf) -> Body {
    let mut body_builder = Body::builder();

    let heading = get_dir_as_heading(dir_resource);
    let list = get_files_as_ul(dir_resource);

    body_builder.push(heading);
    body_builder.thematic_break(|hr| hr);
    body_builder.push(list);

    let body = body_builder.build();
    return body;
}

fn get_dir_as_heading(dir_resource: &PathBuf) -> Header {
    return Header::builder()
        .heading_1(|h1| {
            h1.text(
                dir_resource
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .map(|mut name| {
                        if env::current_dir()
                            .ok()
                            .is_some_and(|path| path == *dir_resource)
                        {
                            name = "".into()
                        }
                        name
                    })
                    .map(|mut name| {
                        if dir_resource.is_dir() {
                            name.push('/')
                        }
                        name
                    })
                    .unwrap_or("".into()),
            )
        })
        .build();
}

fn get_files_as_ul(dir_resource: &PathBuf) -> UnorderedList {
    let mut list_builder = UnorderedList::builder();
    let all_files = get_files_and_dirs(&dir_resource).unwrap();

    for file in all_files {
        let relpath = file.strip_prefix(dir_resource).unwrap();
        if file.is_dir() {
            //list_builder.list_item(|li| li.text(format!("{}/", relpath.display())));
            list_builder.list_item(|li| {
                li.anchor(|a| {
                    a.href(format!("{}/", relpath.display()));
                    a.text(format!("{}/", relpath.display()))
                })
            });
        } else {
            //list_builder.list_item(|li| li.text(relpath.display().to_string()));
            list_builder.list_item(|li| {
                li.anchor(|a| {
                    a.href(relpath.display().to_string());
                    a.text(relpath.display().to_string())
                })
            });
        }
    }

    return list_builder.build();
}
