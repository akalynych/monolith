use crate::utils;

//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[test]
fn passing_remove_protocl_and_fragment() {
    if cfg!(windows) {
        assert_eq!(
            utils::file_url_to_fs_path("file:///C:/documents/some-path/some-file.svg#fragment"),
            "C:\\documents\\some-path\\some-file.svg"
        );
    } else {
        assert_eq!(
            utils::file_url_to_fs_path("file:///tmp/some-path/some-file.svg#fragment"),
            "/tmp/some-path/some-file.svg"
        );
    }
}

#[test]
fn passing_decodes_urls() {
    if cfg!(windows) {
        assert_eq!(
            utils::file_url_to_fs_path("file:///C:/Documents%20and%20Settings/some-file.html"),
            "C:\\Documents and Settings\\some-file.html"
        );
    } else {
        assert_eq!(
            utils::file_url_to_fs_path("file:///home/user/My%20Documents"),
            "/home/user/My Documents"
        );
    }
}
