import { invoke } from "@tauri-apps/api/core";
import type {
  QuoteDto,
  SearchResultDto,
  MemoVersionDto,
  AddQuoteInput,
} from "./types";

export const api = {
  search: (query: string, top = 10) =>
    invoke<SearchResultDto[]>("search_quotes", { query, top }),

  list: () => invoke<QuoteDto[]>("list_quotes"),

  listByTag: (tag: string) => invoke<QuoteDto[]>("list_by_tag", { tag }),

  add: (input: AddQuoteInput) => invoke<string>("add_quote", { input }),

  history: (quoteId: string) =>
    invoke<MemoVersionDto[]>("get_history", { quoteId }),

  updateMemo: (quoteId: string, memo: string) =>
    invoke<number>("update_memo", { quoteId, memo }),
};
