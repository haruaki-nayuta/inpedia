import { useState } from "react";
import { api } from "../api";
import type { AddQuoteInput } from "../types";
import "./AddForm.css";

interface Props {
  onAdded: () => void;
  onClose: () => void;
}

export function AddForm({ onAdded, onClose }: Props) {
  const [quote, setQuote] = useState("");
  const [author, setAuthor] = useState("");
  const [title, setTitle] = useState("");
  const [url, setUrl] = useState("");
  const [tagsRaw, setTagsRaw] = useState("");
  const [memo, setMemo] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>();

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!quote.trim()) return;
    setLoading(true);
    setError(undefined);
    try {
      const input: AddQuoteInput = {
        quote: quote.trim(),
        source_author: author.trim() || undefined,
        source_title: title.trim() || undefined,
        source_url: url.trim() || undefined,
        tags: tagsRaw.split(",").map((t) => t.trim()).filter(Boolean),
        memo: memo.trim() || undefined,
      };
      await api.add(input);
      onAdded();
      onClose();
    } catch (e: unknown) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="add-overlay" onClick={onClose}>
      <form className="add-panel" onSubmit={handleSubmit} onClick={(e) => e.stopPropagation()}>
        <div className="add-header">
          <span>引用を追加</span>
          <button type="button" className="add-close" onClick={onClose}>✕</button>
        </div>

        <div className="add-body">
          <label>
            引用テキスト *
            <textarea
              value={quote}
              onChange={(e) => setQuote(e.target.value)}
              rows={4}
              required
              autoFocus
            />
          </label>
          <label>著者<input value={author} onChange={(e) => setAuthor(e.target.value)} /></label>
          <label>出典タイトル<input value={title} onChange={(e) => setTitle(e.target.value)} /></label>
          <label>URL<input value={url} onChange={(e) => setUrl(e.target.value)} /></label>
          <label>タグ（カンマ区切り）<input value={tagsRaw} onChange={(e) => setTagsRaw(e.target.value)} placeholder="認知, 哲学" /></label>
          <label>
            メモ
            <textarea value={memo} onChange={(e) => setMemo(e.target.value)} rows={3} />
          </label>
          {error && <p className="add-error">{error}</p>}
        </div>

        <div className="add-footer">
          <button type="button" onClick={onClose} className="btn-cancel">キャンセル</button>
          <button type="submit" className="btn-submit" disabled={loading}>
            {loading ? "embedding 生成中…" : "登録"}
          </button>
        </div>
      </form>
    </div>
  );
}
