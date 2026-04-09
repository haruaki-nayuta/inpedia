export interface QuoteDto {
  id: string;
  quote: string;
  source?: string;
  created_at: string;
  latest_memo?: string;
}

export interface SearchResultDto {
  quote: QuoteDto;
  score: number;
}

export interface MemoVersionDto {
  version: number;
  memo: string;
  created_at: string;
}

export interface AddQuoteInput {
  quote: string;
  source?: string;
  memo?: string;
}
