

export interface SearchResultsRes {
	search_id: string; // UUID
	data: RecordResponse[];
	number_of_results: number;
	page: number;
	total_pages: number;
}

export interface RecordResponse {
	data: Record;
	similar_docs: SimilarityInfoFull[];
}

export interface SimilarityInfoFull {
	doc: Record;
	similarity: number;
}

export interface Record {
	id: number;
	title: string;
	text: string;
	label: 1 | 0; // can be either 1 or 0
}

