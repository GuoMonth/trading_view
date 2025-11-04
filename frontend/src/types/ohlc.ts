export interface OhlcDataModel {
  id: number;
  symbol: string;
  period: number;
  timestamp: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  created_at: string;
  updated_at: string;
}

export interface ApiResponse<T> {
  code: number;
  message: string;
  data: T | null;
}

export interface OhlcResponse {
  data: OhlcDataModel[];
}

export interface OhlcDateRangeRequest {
  start: string;
  end: string;
}