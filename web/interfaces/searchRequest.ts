interface SearchRequest {
	searchText: string;
	filter: {
		notFake: boolean;
		// .... other categories we find after indexing data
	}
	//! Name change
	// startIndex: number;
	// numberOfResults: number
	offset: number;
	limit: number;
}