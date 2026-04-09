import { diffLines } from "diff";
import type { MemoVersionDto } from "../types";
import "./DiffView.css";

interface Props {
  versions: MemoVersionDto[];
  onClose: () => void;
}

export function DiffView({ versions, onClose }: Props) {
  return (
    <div className="diff-overlay" onClick={onClose}>
      <div className="diff-panel" onClick={(e) => e.stopPropagation()}>
        <div className="diff-header">
          <span>版の変遷 — {versions.length} 版</span>
          <button className="diff-close" onClick={onClose}>✕</button>
        </div>

        <div className="diff-body">
          {versions.map((v, i) => (
            <div key={v.version} className="diff-version">
              <div className="diff-version-label">
                v{v.version} <span className="diff-date">{v.created_at}</span>
              </div>

              {i === 0 ? (
                <pre className="diff-pre">{v.memo || "(空)"}</pre>
              ) : (
                <DiffBlock prev={versions[i - 1].memo} next={v.memo} />
              )}
            </div>
          ))}
        </div>
      </div>
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
