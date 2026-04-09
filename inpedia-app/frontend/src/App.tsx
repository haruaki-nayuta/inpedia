import { useState, useEffect, useCallback } from "react";
import { api } from "./api";
import type { QuoteDto, SearchResultDto, MemoVersionDto } from "./types";
import { QuoteDetail } from "./components/QuoteDetail";
import { AddForm } from "./components/AddForm";
import "./App.css";

type SidebarItem = { quote: QuoteDto; score?: number };

export default function App() {
  const [query, setQuery] = useState("");
  const [searchResults, setSearchResults] = useState<SearchResultDto[]>([]);
  const [allQuotes, setAllQuotes] = useState<QuoteDto[]>([]);
  const [searching, setSearching] = useState(false);
  const [showAdd, setShowAdd] = useState(false);
  const [selected, setSelected] = useState<SidebarItem | null>(null);
  const [selectedVersions, setSelectedVersions] = useState<MemoVersionDto[]>([]);

  const loadList = useCallback(async () => {
    try {
      const qs = await api.list();
      setAllQuotes(qs);
    } catch (e) {
      console.error(e);
    }
  }, []);

  useEffect(() => { loadList(); }, [loadList]);

  useEffect(() => {
    if (!query.trim()) { setSearchResults([]); return; }
    const t = setTimeout(async () => {
      setSearching(true);
      try { setSearchResults(await api.search(query, 20)); }
      catch (e) { console.error(e); }
      finally { setSearching(false); }
    }, 400);
    return () => clearTimeout(t);
  }, [query]);

  async function selectQuote(q: QuoteDto, score?: number) {
    setSelected({ quote: q, score });
    try {
      const versions = await api.history(q.id);
      setSelectedVersions(versions);
    } catch (e) {
      console.error(e);
      setSelectedVersions([]);
    }
  }

  const items: SidebarItem[] = query.trim()
    ? searchResults.map((r) => ({ quote: r.quote, score: r.score }))
    : allQuotes.map((q) => ({ quote: q }));

  return (
    <div className="app">
      <header className="topbar">
        <span className="logo">inpedia</span>
        <input
          className="search-input"
          placeholder="検索…"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          autoFocus
        />
        {searching && <span className="searching-dot">●</span>}
        <button className="btn-add" onClick={() => setShowAdd(true)}>+ 追加</button>
      </header>

      <div className="main">
        {/* ── Sidebar 30% ── */}
        <aside className="sidebar">
          {items.length === 0 && !searching && (
            <p className="sidebar-empty">{query ? "結果なし" : "引用がありません"}</p>
          )}
          {items.map(({ quote: q, score }) => (
            <div
              key={q.id}
              className={`sidebar-item${selected?.quote.id === q.id ? " selected" : ""}`}
              onClick={() => selectQuote(q, score)}
            >
              <p className="si-quote">{q.quote}</p>
              {q.source && <p className="si-source">— {q.source}</p>}
              {score !== undefined && <span className="si-score">{score.toFixed(2)}</span>}
            </div>
          ))}
        </aside>

        {/* ── Content 70% ── */}
        <main className="content">
          {selected ? (
            <QuoteDetail
              quote={selected.quote}
              score={selected.score}
              versions={selectedVersions}
            />
          ) : (
            <div className="content-empty">← 左の引用を選択してください</div>
          )}
        </main>
      </div>

      {showAdd && (
        <AddForm
          onAdded={() => { loadList(); }}
          onClose={() => setShowAdd(false)}
        />
      )}
    </div>
  );
}
