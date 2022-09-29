use std::future::Future;

use anyhow::Result;
use tokio::fs;

pub trait FileWriter {
    type ResultFuture<'a>: Future<Output = Result<()>>
    where
        Self: 'a;

    fn write(&self, content: String) -> Self::ResultFuture<'_>;
}

pub struct TestFileWriter {
    pub file_path: String,
}

impl FileWriter for TestFileWriter {
    type ResultFuture<'a>
    = impl Future<Output = Result<()>> where Self: 'a;

    fn write(&self, content: String) -> Self::ResultFuture<'_> {
        let file_path = self.file_path.clone();
        async move {
            fs::write(file_path, content).await?;
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_file_writer() -> Result<()> {
    let w = TestFileWriter {
        file_path: "/tmp/gat_filewriter_test".to_string(),
    };
    let expected_content = "hello GAT async".to_string();
    w.write(expected_content.clone()).await?;
    let cnt_vec = fs::read(w.file_path.clone()).await?;
    let content = String::from_utf8(cnt_vec)?;
    assert_eq!(content, expected_content);
    fs::remove_file(w.file_path).await?;
    Ok(())
}
