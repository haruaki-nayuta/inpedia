import { useState, useEffect, useCallback, useRef } from "react";
import { api } from "./api";
import type { QuoteDto, SearchResultDto, MemoVersionDto } from "./types";
import { QuoteCard } from "./components/QuoteCard";
import { DiffView } from "./components/DiffView";
import { AddForm } from "./components/AddForm";
import "./App.css";

type View = "search" | "list";

export default function App() {
  const [view, setView] = useState<View>("search");
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResultDto[]>([]);
  const [allQuotes, setAllQuotes] = useState<QuoteDto[]>([]);
  const [searching, setSearching] = useState(false);
  const [showAdd, setShowAdd] = useState(false);
  const [history, setHistory] = useState<{ quote: QuoteDto; versions: MemoVersionDto[] } | null>(null);
  const [tagFilter, setTagFilter] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const loadList = useCallback(async () => {
    try {
      const qs = tagFilter.trim()
        ? await api.listByTag(tagFilter.trim())
        : await api.list();
      setAllQuotes(qs);
    } catch (e) {
      console.error(e);
    }
  }, [tagFilter]);

  useEffect(() => {
    if (view === "list") loadList();
  }, [view, loadList]);

  useEffect(() => {
    if (!query.trim()) { setResults([]); return; }
    const timer = setTimeout(async () => {
      setSearching(true);
      try {
        const res = await api.search(query, 10);
        setResults(res);
      } catch (e) {
        console.error(e);
      } finally {
        setSearching(false);
      }
    }, 400);
    return () => clearTimeout(timer);
  }, [query]);

  async function openHistory(q: QuoteDto) {
    const versions = await api.history(q.id);
    setHistory({ quote: q, versions });
  }

  return (
    <div className="app">
      {/* ── Top bar ── */}
      <header className="topbar">
        <span className="logo">inpedia</span>
        <nav className="nav">
          <button className={view === "search" ? "active" : ""} onClick={() => setView("search")}>検索</button>
          <button className={view === "list" ? "active" : ""} onClick={() => setView("list")}>一覧</button>
        </nav>
        <button className="btn-add" onClick={() => setShowAdd(true)}>+ 追加</button>
      </header>

      {/* ── Search view ── */}
      {view === "search" && (
        <div className="search-view">
          <input
            ref={inputRef}
            className="search-input"
            placeholder="引用を検索…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            autoFocus
          />
          {searching && <p className="status">検索中…</p>}
          {!searching && query && results.length === 0 && (
            <p className="status">結果なし</p>
          )}
          <div className="cards">
            {results.map((r) => (
              <QuoteCard
                key={r.quote.id}
                quote={r.quote}
                score={r.score}
                onSelect={openHistory}
              />
            ))}
          </div>
        </div>
      )}

      {/* ── List view ── */}
      {view === "list" && (
        <div className="list-view">
          <input
            className="tag-input"
            placeholder="タグで絞り込み…"
            value={tagFilter}
            onChange={(e) => setTagFilter(e.target.value)}
          />
          <div className="cards">
            {allQuotes.map((q) => (
              <QuoteCard key={q.id} quote={q} onSelect={openHistory} />
            ))}
          </div>
        </div>
      )}

      {/* ── Modals ── */}
      {showAdd && (
        <AddForm onAdded={loadList} onClose={() => setShowAdd(false)} />
      )}
      {history && (
        <DiffView
          versions={history.versions}
          onClose={() => setHistory(null)}
        />
      )}
    </div>
  );
}
