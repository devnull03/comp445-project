interface singleResult {
	id: number;
	title: string;
	articleText: string;
	fakeOrNot: boolean;
	similarityScore: number;
}

interface searchResults {
	data: singleResult[],
	numberOfResults: number,
	totalPages: number
}