import type { QuoteDto } from "../types";
import "./QuoteCard.css";

interface Props {
  quote: QuoteDto;
  score?: number;
  onSelect: (q: QuoteDto) => void;
}

/** Expand {{img:hash}} and {{vid:hash}} markers in memo text */
function renderMemo(memo: string): React.ReactNode[] {
  const parts = memo.split(/({{(?:img|vid):[^}]+}})/g);
  return parts.map((part, i) => {
    const imgMatch = part.match(/^{{img:([^}]+)}}$/);
    const vidMatch = part.match(/^{{vid:([^}]+)}}$/);
    if (imgMatch) {
      return (
        <img
          key={i}
          src={`asset://${imgMatch[1]}`}
          alt=""
          className="inline-asset"
        />
      );
    }
    if (vidMatch) {
      return (
        <video key={i} src={`asset://${vidMatch[1]}`} controls className="inline-asset" />
      );
    }
    return <span key={i}>{part}</span>;
  });
}

export function QuoteCard({ quote: q, score, onSelect }: Props) {
  return (
    <div className="card" onClick={() => onSelect(q)}>
      <p className="card-quote">&#8220;{q.quote}&#8221;</p>

      <div className="card-meta">
        {q.source_author && <span className="meta-author">— {q.source_author}</span>}
        {q.source_title && <span className="meta-title">『{q.source_title}』</span>}
        {score !== undefined && (
          <span className="meta-score">score {score.toFixed(3)}</span>
        )}
      </div>

      {q.tags.length > 0 && (
        <div className="card-tags">
          {q.tags.map((t) => (
            <span key={t} className="tag">{t}</span>
          ))}
        </div>
      )}

      {q.latest_memo && (
        <div className="card-memo">{renderMemo(q.latest_memo)}</div>
      )}
    </div>
  );
}
