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
  const [source, setSource] = useState("");
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
        source: source.trim() || undefined,
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
          <label>
            引用元
            <input
              value={source}
              onChange={(e) => setSource(e.target.value)}
              placeholder="著者名・書籍・URL など"
            />
          </label>
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
