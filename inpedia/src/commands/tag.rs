use anyhow::Result;

pub async fn run(_tag: &str, _json: bool) -> Result<()> {
    anyhow::bail!("tag コマンドは廃止されました。メモや引用元フィールドに分類を記載してください。");
}
