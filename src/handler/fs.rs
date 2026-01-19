use crate::model::result::base::BizResult;

pub struct FsHandler;
impl FsHandler {
    pub async fn ls() -> BizResult<String> {
        BizResult::ok(String::from("ls"))
    }

    pub async fn mkdir() -> BizResult<String> {
        BizResult::ok(String::from("mkdir"))
    }

    pub async fn touch() -> BizResult<String> {
        BizResult::ok(String::from("touch"))
    }

    pub async fn read() -> BizResult<String> {
        BizResult::ok(String::from("read"))
    }

    pub async fn write() -> BizResult<String> {
        BizResult::ok(String::from("write"))
    }

    pub async fn mv() -> BizResult<String> {
        BizResult::ok(String::from("mv"))
    }

    pub async fn rm() -> BizResult<String> {
        BizResult::ok(String::from("rm"))
    }
}
