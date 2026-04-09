import { useState } from "react";
import { diffLines } from "diff";
import type { QuoteDto, MemoVersionDto } from "../types";
import "./QuoteDetail.css";

interface Props {
  quote: QuoteDto;
  score?: number;
  versions: MemoVersionDto[];
}

export function QuoteDetail({ quote, score, versions }: Props) {
  const [showHistory, setShowHistory] = useState(false);

  return (
    <div className="qd">
      <blockquote className="qd-quote">{quote.quote}</blockquote>

      <div className="qd-meta">
        {quote.source && <span className="qd-source">— {quote.source}</span>}
        {score !== undefined && <span className="qd-score">score {score.toFixed(3)}</span>}
        <span className="qd-date">{quote.created_at}</span>
      </div>

      {quote.latest_memo && (
        <div className="qd-memo">
          <p className="qd-memo-label">メモ</p>
          <pre className="qd-memo-body">{quote.latest_memo}</pre>
        </div>
      )}

      {versions.length > 1 && (
        <div className="qd-history">
          <button
            className="qd-history-btn"
            onClick={() => setShowHistory((s) => !s)}
          >
            {showHistory ? "▾" : "▸"} 更新履歴（{versions.length} 版）
          </button>

          {showHistory && (
            <div className="qd-history-body">
              {versions.map((v, i) => (
                <div key={v.version} className="qd-version">
                  <div className="qd-version-label">
                    v{v.version}
                    <span className="qd-version-date">{v.created_at}</span>
                  </div>
                  {i === 0 ? (
                    <pre className="diff-pre">{v.memo || "(空)"}</pre>
                  ) : (
                    <DiffBlock prev={versions[i - 1].memo} next={v.memo} />
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function DiffBlock({ prev, next }: { prev: string; next: string }) {
  const changes = diffLines(prev, next);
  return (
    <pre className="diff-pre">
      {changes.map((c, i) => {
        if (c.added) return <span key={i} className="diff-add">{c.value}</span>;
        if (c.removed) return <span key={i} className="diff-del">{c.value}</span>;
        return <span key={i} className="diff-eq">{c.value}</span>;
      })}
    </pre>
  );
}
